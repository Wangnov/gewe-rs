//! Postgres 存储实现

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgPool, Row};

use super::{BackupInfo, ConfigMeta, ConfigStorage, PromptInfo, PromptStorage};
use crate::config::AppConfigV2;

/// Postgres 存储实现
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

    /// 运行迁移
    pub async fn run_migrations(&self) -> Result<(), String> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| format!("运行迁移失败: {}", e))?;
        Ok(())
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
