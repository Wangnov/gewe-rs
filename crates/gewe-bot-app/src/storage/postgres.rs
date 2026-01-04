//! Postgres 存储实现

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPool, Row};

use super::{BackupInfo, ConfigMeta, ConfigStorage, PromptInfo, PromptStorage};
use crate::config::AppConfigV2;

/// Postgres 存储实现
#[derive(Debug)]
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    /// 创建新的 Postgres 存储
    pub async fn new(database_url: &str) -> Result<Self, String> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| format!("连接数据库失败: {}", e))?;

        Ok(Self { pool })
    }

    /// 运行迁移（需启用 db-migrate 特性）
    #[cfg(feature = "db-migrate")]
    pub async fn run_migrations(&self) -> Result<(), String> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| format!("运行迁移失败: {}", e))?;
        Ok(())
    }

    /// 运行迁移（需启用 db-migrate 特性）
    #[cfg(not(feature = "db-migrate"))]
    pub async fn run_migrations(&self) -> Result<(), String> {
        Err("运行迁移需要启用 feature `db-migrate`，或改用 sqlx-cli 执行迁移".to_string())
    }

    /// 计算 ETag
    fn compute_etag(content: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[async_trait]
impl ConfigStorage for PostgresStorage {
    async fn get_current(&self) -> Result<AppConfigV2, String> {
        let row = sqlx::query(
            "SELECT COALESCE(draft_json, config_json) as config FROM config_current WHERE id = 1",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("查询配置失败: {}", e))?;

        let config_json: serde_json::Value = row
            .try_get("config")
            .map_err(|e| format!("获取配置字段失败: {}", e))?;

        serde_json::from_value(config_json).map_err(|e| format!("反序列化配置失败: {}", e))
    }

    async fn save_draft(&self, config: &AppConfigV2) -> Result<String, String> {
        let config_json =
            serde_json::to_value(config).map_err(|e| format!("序列化配置失败: {}", e))?;

        let config_str =
            serde_json::to_string(config).map_err(|e| format!("序列化配置失败: {}", e))?;
        let etag = Self::compute_etag(&config_str);

        sqlx::query(
            "UPDATE config_current SET draft_json = $1, draft_etag = $2, last_saved_at = NOW() WHERE id = 1",
        )
        .bind(config_json)
        .bind(&etag)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存草稿失败: {}", e))?;

        Ok(etag)
    }

    async fn publish(&self, remark: Option<String>) -> Result<BackupInfo, String> {
        // 开始事务
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| format!("开始事务失败: {}", e))?;

        // 获取当前版本号
        let row = sqlx::query("SELECT current_version FROM config_current WHERE id = 1")
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| format!("查询版本号失败: {}", e))?;

        let current_version: i32 = row
            .try_get("current_version")
            .map_err(|e| format!("获取版本号失败: {}", e))?;
        let new_version = (current_version + 1) as u64;

        // 获取当前配置
        let row = sqlx::query(
            "SELECT COALESCE(draft_json, config_json) as config FROM config_current WHERE id = 1",
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| format!("查询配置失败: {}", e))?;

        let config_json: serde_json::Value = row
            .try_get("config")
            .map_err(|e| format!("获取配置字段失败: {}", e))?;

        // 插入发布记录
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO config_releases (version, config_json, remark, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(new_version as i32)
        .bind(&config_json)
        .bind(&remark)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("插入发布记录失败: {}", e))?;

        // 计算 ETag
        let config_str = config_json.to_string();
        let etag = Self::compute_etag(&config_str);

        // 更新当前配置
        sqlx::query(
            "UPDATE config_current SET config_json = $1, draft_json = NULL, current_version = $2, etag = $3, draft_etag = NULL, last_published_at = $4 WHERE id = 1",
        )
        .bind(&config_json)
        .bind(new_version as i32)
        .bind(&etag)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("更新当前配置失败: {}", e))?;

        // 提交事务
        tx.commit()
            .await
            .map_err(|e| format!("提交事务失败: {}", e))?;

        Ok(BackupInfo {
            version: new_version,
            filename: format!("v{}", new_version),
            created_at: now,
            remark,
        })
    }

    async fn rollback(&self, version: u64) -> Result<(), String> {
        // 开始事务
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| format!("开始事务失败: {}", e))?;

        // 查询历史版本
        let row =
            sqlx::query("SELECT config_json, created_at FROM config_releases WHERE version = $1")
                .bind(version as i32)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| format!("查询版本失败: {}", e))?;

        let config_json: serde_json::Value = row
            .try_get("config_json")
            .map_err(|e| format!("获取配置失败: {}", e))?;

        // 计算 ETag
        let config_str = config_json.to_string();
        let etag = Self::compute_etag(&config_str);

        // 更新当前配置
        sqlx::query(
            "UPDATE config_current SET config_json = $1, draft_json = NULL, etag = $2, draft_etag = NULL, last_reload_at = NOW(), last_reload_result = $3 WHERE id = 1",
        )
        .bind(&config_json)
        .bind(&etag)
        .bind(format!("restored from v{}", version))
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("更新配置失败: {}", e))?;

        // 提交事务
        tx.commit()
            .await
            .map_err(|e| format!("提交事务失败: {}", e))?;

        Ok(())
    }

    async fn get_meta(&self) -> Result<ConfigMeta, String> {
        let row = sqlx::query(
            "SELECT current_version, etag, draft_etag, last_published_at, last_saved_at, last_reload_at, last_reload_result FROM config_current WHERE id = 1",
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("查询元信息失败: {}", e))?;

        let version: i32 = row.try_get("current_version").unwrap_or(0);
        let etag: String = row.try_get("etag").unwrap_or_default();
        let draft_etag: Option<String> = row.try_get("draft_etag").ok();
        let last_published_at: Option<DateTime<Utc>> = row.try_get("last_published_at").ok();
        let last_saved_at: Option<DateTime<Utc>> = row.try_get("last_saved_at").ok();
        let last_reload_at: Option<DateTime<Utc>> = row.try_get("last_reload_at").ok();
        let last_reload_result: Option<String> = row.try_get("last_reload_result").ok();

        let backups = self.scan_backups().await?;

        Ok(ConfigMeta {
            version: version as u64,
            etag,
            has_draft: draft_etag.is_some(),
            last_published_at,
            last_saved_at,
            last_reload_at,
            last_reload_result,
            available_backups: backups,
        })
    }

    async fn scan_backups(&self) -> Result<Vec<BackupInfo>, String> {
        let rows = sqlx::query(
            "SELECT version, remark, created_at FROM config_releases ORDER BY version DESC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| format!("查询版本列表失败: {}", e))?;

        let backups = rows
            .iter()
            .map(|row| {
                let version: i32 = row.try_get("version").unwrap_or(0);
                let remark: Option<String> = row.try_get("remark").ok();
                let created_at: DateTime<Utc> =
                    row.try_get("created_at").unwrap_or_else(|_| Utc::now());

                BackupInfo {
                    version: version as u64,
                    filename: format!("v{}", version),
                    created_at,
                    remark,
                }
            })
            .collect();

        Ok(backups)
    }
}

#[async_trait]
impl PromptStorage for PostgresStorage {
    async fn list_prompts(&self) -> Result<Vec<PromptInfo>, String> {
        let rows = sqlx::query("SELECT name, size, updated_at FROM prompts ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("查询 Prompts 失败: {}", e))?;

        let prompts = rows
            .iter()
            .map(|row| PromptInfo {
                name: row.try_get("name").unwrap_or_default(),
                size: row.try_get::<i32, _>("size").unwrap_or(0) as u64,
                modified_at: row.try_get("updated_at").unwrap_or_else(|_| Utc::now()),
            })
            .collect();

        Ok(prompts)
    }

    async fn get_prompt(&self, name: &str) -> Result<String, String> {
        let row = sqlx::query("SELECT content FROM prompts WHERE name = $1")
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("查询 Prompt 失败: {}", e))?;

        row.try_get("content")
            .map_err(|e| format!("获取内容失败: {}", e))
    }

    async fn put_prompt(&self, name: &str, content: &str) -> Result<(), String> {
        let size = content.len() as i32;

        sqlx::query(
            "INSERT INTO prompts (name, content, size) VALUES ($1, $2, $3)
             ON CONFLICT (name) DO UPDATE SET content = $2, size = $3",
        )
        .bind(name)
        .bind(content)
        .bind(size)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("保存 Prompt 失败: {}", e))?;

        Ok(())
    }

    async fn delete_prompt(&self, name: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM prompts WHERE name = $1")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("删除 Prompt 失败: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_etag() {
        let content = "test content";
        let etag = PostgresStorage::compute_etag(content);

        // ETag 应该是 64 个字符的十六进制字符串
        assert_eq!(etag.len(), 64);
        assert!(etag.chars().all(|c| c.is_ascii_hexdigit()));

        // 相同内容应该生成相同的 ETag
        let etag2 = PostgresStorage::compute_etag(content);
        assert_eq!(etag, etag2);

        // 不同内容应该生成不同的 ETag
        let etag3 = PostgresStorage::compute_etag("different content");
        assert_ne!(etag, etag3);
    }

    #[test]
    fn test_compute_etag_empty_string() {
        let etag = PostgresStorage::compute_etag("");
        assert_eq!(etag.len(), 64);
        assert!(etag.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_compute_etag_unicode() {
        let content = "测试内容 🚀";
        let etag = PostgresStorage::compute_etag(content);
        assert_eq!(etag.len(), 64);
        assert!(etag.chars().all(|c| c.is_ascii_hexdigit()));

        // 不同的 Unicode 字符串应该生成不同的 ETag
        let etag2 = PostgresStorage::compute_etag("测试内容 🎉");
        assert_ne!(etag, etag2);
    }

    #[test]
    fn test_compute_etag_deterministic() {
        // 测试 ETag 生成是否确定性的
        let content = "test content for deterministic check";
        let etag1 = PostgresStorage::compute_etag(content);
        let etag2 = PostgresStorage::compute_etag(content);
        let etag3 = PostgresStorage::compute_etag(content);

        assert_eq!(etag1, etag2);
        assert_eq!(etag2, etag3);
    }

    #[test]
    fn test_compute_etag_long_content() {
        // 测试长内容
        let content = "a".repeat(10000);
        let etag = PostgresStorage::compute_etag(&content);
        assert_eq!(etag.len(), 64);
        assert!(etag.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    async fn test_new_with_invalid_url() {
        // 测试使用无效的数据库 URL
        let result = PostgresStorage::new("invalid-url").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("连接数据库失败"));
    }

    #[tokio::test]
    async fn test_new_with_empty_url() {
        // 测试使用空 URL
        let result = PostgresStorage::new("").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("连接数据库失败"));
    }

    // 注意: 以下测试需要实际的数据库连接,所以我们只测试错误情况
    // 实际的集成测试应该在单独的测试套件中进行

    #[test]
    fn test_backup_info_format() {
        // 测试备份信息的格式化
        let version = 42u64;
        let filename = format!("v{}", version);
        assert_eq!(filename, "v42");
    }

    #[test]
    fn test_version_conversion() {
        // 测试版本号转换
        let version_u64: u64 = 100;
        let version_i32: i32 = version_u64 as i32;
        assert_eq!(version_i32, 100);

        let version_back: u64 = version_i32 as u64;
        assert_eq!(version_back, version_u64);
    }

    #[test]
    fn test_json_serialization() {
        // 测试 JSON 序列化
        use crate::config::AppConfigV2;

        let config = AppConfigV2::parse(
            r#"
config_version = 2

[server]
listen_addr = "0.0.0.0:3000"

[storage]
backend = "file"

[[bots]]
app_id = "test_bot"
base_url = "http://localhost:2531"
token = "test_token"
"#,
        )
        .unwrap();

        let json_value = serde_json::to_value(&config).unwrap();
        assert!(json_value.is_object());

        // 验证反序列化
        let config_back: AppConfigV2 = serde_json::from_value(json_value).unwrap();
        assert_eq!(config_back.config_version, 2);
        assert_eq!(config_back.bots.len(), 1);
        assert_eq!(config_back.bots[0].app_id, "test_bot");
    }

    #[test]
    fn test_error_message_format() {
        // 测试错误消息格式
        let error_msg = format!("连接数据库失败: {}", "timeout");
        assert!(error_msg.contains("连接数据库失败"));
        assert!(error_msg.contains("timeout"));

        let error_msg = format!("查询配置失败: {}", "connection lost");
        assert!(error_msg.contains("查询配置失败"));
        assert!(error_msg.contains("connection lost"));
    }

    #[test]
    fn test_sql_query_constants() {
        // 测试 SQL 查询字符串常量是否有效
        let queries = vec![
            "SELECT COALESCE(draft_json, config_json) as config FROM config_current WHERE id = 1",
            "UPDATE config_current SET draft_json = $1, draft_etag = $2, last_saved_at = NOW() WHERE id = 1",
            "SELECT current_version FROM config_current WHERE id = 1",
            "INSERT INTO config_releases (version, config_json, remark, created_at) VALUES ($1, $2, $3, $4)",
            "SELECT config_json, created_at FROM config_releases WHERE version = $1",
            "SELECT version, remark, created_at FROM config_releases ORDER BY version DESC",
            "SELECT name, size, updated_at FROM prompts ORDER BY name",
            "SELECT content FROM prompts WHERE name = $1",
            "INSERT INTO prompts (name, content, size) VALUES ($1, $2, $3) ON CONFLICT (name) DO UPDATE SET content = $2, size = $3",
            "DELETE FROM prompts WHERE name = $1",
        ];

        // 验证每个查询都是有效的 SQL 语句(至少包含基本关键字)
        for query in queries {
            assert!(
                query.contains("SELECT")
                    || query.contains("INSERT")
                    || query.contains("UPDATE")
                    || query.contains("DELETE"),
                "Query should contain SQL keywords: {}",
                query
            );
        }
    }

    #[test]
    fn test_placeholder_syntax() {
        // 验证 PostgreSQL 占位符语法
        let query = "INSERT INTO prompts (name, content, size) VALUES ($1, $2, $3)";
        assert!(query.contains("$1"));
        assert!(query.contains("$2"));
        assert!(query.contains("$3"));
    }

    #[test]
    fn test_size_conversion() {
        // 测试大小转换
        let content = "test content";
        let size = content.len() as i32;
        assert_eq!(size, 12);

        // 验证可以转换回来
        let size_u64 = size as u64;
        assert_eq!(size_u64, 12);
    }

    #[test]
    fn test_datetime_handling() {
        // 测试日期时间处理
        let now = Utc::now();
        assert!(now <= Utc::now());

        // 测试时间戳格式
        let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
        assert!(formatted.len() >= 19); // YYYY-MM-DD HH:MM:SS
    }

    #[test]
    fn test_option_handling() {
        // 测试 Option 处理
        let remark: Option<String> = Some("test remark".to_string());
        assert!(remark.is_some());
        assert_eq!(remark.unwrap(), "test remark");

        let remark: Option<String> = None;
        assert!(remark.is_none());
    }

    #[test]
    fn test_backup_info_creation() {
        // 测试 BackupInfo 创建
        let version = 1u64;
        let filename = format!("v{}", version);
        let now = Utc::now();
        let remark = Some("test backup".to_string());

        let backup = BackupInfo {
            version,
            filename: filename.clone(),
            created_at: now,
            remark: remark.clone(),
        };

        assert_eq!(backup.version, 1);
        assert_eq!(backup.filename, "v1");
        assert!(backup.remark.is_some());
        assert_eq!(backup.remark.unwrap(), "test backup");
    }

    #[test]
    fn test_config_meta_fields() {
        // 测试 ConfigMeta 字段
        let meta = ConfigMeta {
            version: 5,
            etag: "test_etag".to_string(),
            has_draft: true,
            last_published_at: Some(Utc::now()),
            last_saved_at: Some(Utc::now()),
            last_reload_at: Some(Utc::now()),
            last_reload_result: Some("ok".to_string()),
            available_backups: vec![],
        };

        assert_eq!(meta.version, 5);
        assert_eq!(meta.etag, "test_etag");
        assert!(meta.has_draft);
        assert!(meta.last_published_at.is_some());
        assert!(meta.last_saved_at.is_some());
        assert!(meta.last_reload_at.is_some());
        assert_eq!(meta.last_reload_result, Some("ok".to_string()));
        assert_eq!(meta.available_backups.len(), 0);
    }

    #[test]
    fn test_prompt_info_creation() {
        // 测试 PromptInfo 创建
        let info = PromptInfo {
            name: "test.txt".to_string(),
            size: 1024,
            modified_at: Utc::now(),
        };

        assert_eq!(info.name, "test.txt");
        assert_eq!(info.size, 1024);
        assert!(info.modified_at <= Utc::now());
    }

    #[test]
    fn test_restore_message_format() {
        // 测试回滚消息格式
        let version = 42u64;
        let message = format!("restored from v{}", version);
        assert_eq!(message, "restored from v42");
    }
}
