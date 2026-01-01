//! 文件系统存储实现

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs;

use super::{BackupInfo, ConfigMeta, ConfigStorage, PromptInfo, PromptStorage};
use crate::config::AppConfigV2;

/// 文件存储实现
pub struct FileStorage {
    config_path: PathBuf,
    prompts_dir: PathBuf,
    backup_dir: PathBuf,
}

impl FileStorage {
    pub fn new(config_path: PathBuf, prompts_dir: PathBuf, backup_dir: PathBuf) -> Self {
        Self {
            config_path,
            prompts_dir,
            backup_dir,
        }
    }

    /// 计算 ETag
    fn compute_etag(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// 解析备份文件名
    fn parse_backup_filename(filename: &str) -> Option<BackupInfo> {
        // 格式: bot-app.v2.toml.v{version}.{timestamp}
        let parts: Vec<&str> = filename.split('.').collect();
        if parts.len() < 5 {
            return None;
        }

        let version_part = parts.get(parts.len() - 2)?;
        if !version_part.starts_with('v') {
            return None;
        }
        let version: u64 = version_part[1..].parse().ok()?;

        let timestamp_str = parts.last()?;
        let created_at = chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y%m%d%H%M%S")
            .ok()
            .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))?;

        Some(BackupInfo {
            version,
            filename: filename.to_string(),
            created_at,
            remark: None,
        })
    }
}

#[async_trait]
impl ConfigStorage for FileStorage {
    async fn get_current(&self) -> Result<AppConfigV2, String> {
        let content = fs::read_to_string(&self.config_path)
            .await
            .map_err(|e| format!("读取配置文件失败: {}", e))?;

        AppConfigV2::parse(&content).map_err(|e| format!("解析配置失败: {}", e))
    }

    async fn save_draft(&self, config: &AppConfigV2) -> Result<String, String> {
        let content = config.to_toml().map_err(|e| format!("序列化失败: {}", e))?;

        fs::write(&self.config_path, &content)
            .await
            .map_err(|e| format!("写入文件失败: {}", e))?;

        Ok(Self::compute_etag(&content))
    }

    async fn publish(&self, remark: Option<String>) -> Result<BackupInfo, String> {
        // 确保备份目录存在
        fs::create_dir_all(&self.backup_dir)
            .await
            .map_err(|e| format!("创建备份目录失败: {}", e))?;

        // 扫描现有备份获取最新版本号
        let backups = self.scan_backups().await?;
        let new_version = backups.first().map(|b| b.version + 1).unwrap_or(1);

        // 创建备份
        let now = Utc::now();
        let timestamp = now.format("%Y%m%d%H%M%S").to_string();
        let filename = format!("bot-app.v2.toml.v{}.{}", new_version, timestamp);
        let backup_path = self.backup_dir.join(&filename);

        fs::copy(&self.config_path, &backup_path)
            .await
            .map_err(|e| format!("创建备份失败: {}", e))?;

        Ok(BackupInfo {
            version: new_version,
            filename,
            created_at: now,
            remark,
        })
    }

    async fn rollback(&self, version: u64) -> Result<(), String> {
        let backups = self.scan_backups().await?;
        let backup = backups
            .iter()
            .find(|b| b.version == version)
            .ok_or_else(|| format!("未找到版本 {} 的备份", version))?;

        let backup_path = self.backup_dir.join(&backup.filename);
        if !backup_path.exists() {
            return Err(format!("备份文件不存在: {}", backup.filename));
        }

        fs::copy(&backup_path, &self.config_path)
            .await
            .map_err(|e| format!("恢复备份失败: {}", e))?;

        Ok(())
    }

    async fn get_meta(&self) -> Result<ConfigMeta, String> {
        let content = fs::read_to_string(&self.config_path)
            .await
            .map_err(|e| format!("读取配置文件失败: {}", e))?;

        let etag = Self::compute_etag(&content);
        let backups = self.scan_backups().await?;
        let version = backups.first().map(|b| b.version).unwrap_or(0);

        Ok(ConfigMeta {
            version,
            etag,
            has_draft: false,
            last_published_at: backups.first().map(|b| b.created_at),
            last_saved_at: None,
            last_reload_at: Some(Utc::now()),
            last_reload_result: Some("ok".to_string()),
            available_backups: backups,
        })
    }

    async fn scan_backups(&self) -> Result<Vec<BackupInfo>, String> {
        // 确保备份目录存在
        if !self.backup_dir.exists() {
            fs::create_dir_all(&self.backup_dir)
                .await
                .map_err(|e| format!("创建备份目录失败: {}", e))?;
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();
        let mut entries = fs::read_dir(&self.backup_dir)
            .await
            .map_err(|e| format!("读取备份目录失败: {}", e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| format!("读取目录条目失败: {}", e))?
        {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with("bot-app.v2.toml.v") {
                if let Some(info) = Self::parse_backup_filename(&filename) {
                    backups.push(info);
                }
            }
        }

        // 按版本号降序排列
        backups.sort_by(|a, b| b.version.cmp(&a.version));
        Ok(backups)
    }
}

#[async_trait]
impl PromptStorage for FileStorage {
    async fn list_prompts(&self) -> Result<Vec<PromptInfo>, String> {
        if !self.prompts_dir.exists() {
            fs::create_dir_all(&self.prompts_dir)
                .await
                .map_err(|e| format!("创建 prompts 目录失败: {}", e))?;
            return Ok(Vec::new());
        }

        let mut prompts = Vec::new();
        let mut entries = fs::read_dir(&self.prompts_dir)
            .await
            .map_err(|e| format!("读取 prompts 目录失败: {}", e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| format!("读取目录条目失败: {}", e))?
        {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.ends_with(".txt") || filename.ends_with(".md") {
                if let Ok(metadata) = entry.metadata().await {
                    if let Ok(modified) = metadata.modified() {
                        prompts.push(PromptInfo {
                            name: filename,
                            size: metadata.len(),
                            modified_at: modified.into(),
                        });
                    }
                }
            }
        }

        prompts.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(prompts)
    }

    async fn get_prompt(&self, name: &str) -> Result<String, String> {
        let file_path = self.prompts_dir.join(name);
        fs::read_to_string(&file_path)
            .await
            .map_err(|e| format!("读取 Prompt 失败: {}", e))
    }

    async fn put_prompt(&self, name: &str, content: &str) -> Result<(), String> {
        fs::create_dir_all(&self.prompts_dir)
            .await
            .map_err(|e| format!("创建目录失败: {}", e))?;

        let file_path = self.prompts_dir.join(name);
        fs::write(&file_path, content)
            .await
            .map_err(|e| format!("写入 Prompt 失败: {}", e))
    }

    async fn delete_prompt(&self, name: &str) -> Result<(), String> {
        let file_path = self.prompts_dir.join(name);
        fs::remove_file(&file_path)
            .await
            .map_err(|e| format!("删除 Prompt 失败: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // 辅助函数：创建测试配置
    fn create_test_config() -> AppConfigV2 {
        AppConfigV2::parse(
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
        .unwrap()
    }

    // 辅助函数：创建文件存储
    async fn create_test_storage() -> (FileStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("bot-app.v2.toml");
        let prompts_dir = temp_dir.path().join("prompts");
        let backup_dir = temp_dir.path().join("backups");

        tokio::fs::create_dir_all(&prompts_dir).await.unwrap();

        let storage = FileStorage::new(config_path, prompts_dir, backup_dir);
        (storage, temp_dir)
    }

    #[test]
    fn test_file_storage_new() {
        let config_path = PathBuf::from("config.toml");
        let prompts_dir = PathBuf::from("prompts");
        let backup_dir = PathBuf::from("backups");

        let storage =
            FileStorage::new(config_path.clone(), prompts_dir.clone(), backup_dir.clone());

        assert_eq!(storage.config_path, config_path);
        assert_eq!(storage.prompts_dir, prompts_dir);
        assert_eq!(storage.backup_dir, backup_dir);
    }

    #[test]
    fn test_compute_etag() {
        let content = "test content";
        let etag = FileStorage::compute_etag(content);

        // ETag 应该是 64 个字符的十六进制字符串
        assert_eq!(etag.len(), 64);
        assert!(etag.chars().all(|c| c.is_ascii_hexdigit()));

        // 相同内容应该生成相同的 ETag
        let etag2 = FileStorage::compute_etag(content);
        assert_eq!(etag, etag2);

        // 不同内容应该生成不同的 ETag
        let etag3 = FileStorage::compute_etag("different content");
        assert_ne!(etag, etag3);
    }

    #[test]
    fn test_parse_backup_filename_valid() {
        let filename = "bot-app.v2.toml.v1.20231201120000";
        let info = FileStorage::parse_backup_filename(filename);

        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.version, 1);
        assert_eq!(info.filename, filename);
        assert!(info.remark.is_none());
    }

    #[test]
    fn test_parse_backup_filename_invalid() {
        // 测试各种无效格式
        assert!(FileStorage::parse_backup_filename("invalid").is_none());
        assert!(FileStorage::parse_backup_filename("bot-app.toml").is_none());
        assert!(FileStorage::parse_backup_filename("bot-app.v2.toml.1.20231201120000").is_none());
        assert!(
            FileStorage::parse_backup_filename("bot-app.v2.toml.vabc.20231201120000").is_none()
        );
    }

    #[tokio::test]
    async fn test_save_and_get_current() {
        let (storage, _temp_dir) = create_test_storage().await;
        let config = create_test_config();

        // 保存配置
        let etag = storage.save_draft(&config).await.unwrap();
        assert_eq!(etag.len(), 64);

        // 读取配置
        let loaded_config = storage.get_current().await.unwrap();
        assert_eq!(loaded_config.config_version, config.config_version);
        assert_eq!(loaded_config.bots.len(), config.bots.len());
    }

    #[tokio::test]
    async fn test_get_current_nonexistent() {
        let (storage, _temp_dir) = create_test_storage().await;

        // 读取不存在的配置应该失败
        let result = storage.get_current().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("读取配置文件失败"));
    }

    #[tokio::test]
    async fn test_publish_creates_backup() {
        let (storage, _temp_dir) = create_test_storage().await;
        let config = create_test_config();

        // 保存配置
        storage.save_draft(&config).await.unwrap();

        // 发布版本
        let backup_info = storage
            .publish(Some("First release".to_string()))
            .await
            .unwrap();

        assert_eq!(backup_info.version, 1);
        assert!(backup_info.filename.starts_with("bot-app.v2.toml.v1."));
        assert_eq!(backup_info.remark, Some("First release".to_string()));

        // 验证备份文件是否存在
        let backups = storage.scan_backups().await.unwrap();
        assert_eq!(backups.len(), 1);
        assert_eq!(backups[0].version, 1);
    }

    #[tokio::test]
    async fn test_publish_increments_version() {
        let (storage, _temp_dir) = create_test_storage().await;
        let config = create_test_config();

        storage.save_draft(&config).await.unwrap();

        // 发布第一个版本
        let backup1 = storage.publish(None).await.unwrap();
        assert_eq!(backup1.version, 1);

        // 发布第二个版本
        let backup2 = storage.publish(None).await.unwrap();
        assert_eq!(backup2.version, 2);

        // 验证备份列表
        let backups = storage.scan_backups().await.unwrap();
        assert_eq!(backups.len(), 2);
        assert_eq!(backups[0].version, 2); // 按版本降序排列
        assert_eq!(backups[1].version, 1);
    }

    #[tokio::test]
    async fn test_rollback() {
        let (storage, _temp_dir) = create_test_storage().await;
        let mut config = create_test_config();

        // 保存并发布 v1
        storage.save_draft(&config).await.unwrap();
        storage.publish(Some("v1".to_string())).await.unwrap();

        // 修改配置并发布 v2
        config.bots[0].app_id = "modified_bot".to_string();
        storage.save_draft(&config).await.unwrap();
        storage.publish(Some("v2".to_string())).await.unwrap();

        // 验证当前配置是 v2
        let current = storage.get_current().await.unwrap();
        assert_eq!(current.bots[0].app_id, "modified_bot");

        // 回滚到 v1
        storage.rollback(1).await.unwrap();

        // 验证配置已恢复
        let current = storage.get_current().await.unwrap();
        assert_eq!(current.bots[0].app_id, "test_bot");
    }

    #[tokio::test]
    async fn test_rollback_nonexistent_version() {
        let (storage, _temp_dir) = create_test_storage().await;

        // 回滚到不存在的版本应该失败
        let result = storage.rollback(999).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("未找到版本"));
    }

    #[tokio::test]
    async fn test_get_meta() {
        let (storage, _temp_dir) = create_test_storage().await;
        let config = create_test_config();

        storage.save_draft(&config).await.unwrap();
        storage
            .publish(Some("First version".to_string()))
            .await
            .unwrap();

        let meta = storage.get_meta().await.unwrap();

        assert_eq!(meta.version, 1);
        assert_eq!(meta.etag.len(), 64);
        assert!(!meta.has_draft); // FileStorage 总是返回 false
        assert!(meta.last_published_at.is_some());
        assert_eq!(meta.available_backups.len(), 1);
    }

    #[tokio::test]
    async fn test_scan_backups_empty() {
        let (storage, _temp_dir) = create_test_storage().await;

        let backups = storage.scan_backups().await.unwrap();
        assert_eq!(backups.len(), 0);
    }

    #[tokio::test]
    async fn test_list_prompts_empty() {
        let (storage, _temp_dir) = create_test_storage().await;

        let prompts = storage.list_prompts().await.unwrap();
        assert_eq!(prompts.len(), 0);
    }

    #[tokio::test]
    async fn test_put_and_get_prompt() {
        let (storage, _temp_dir) = create_test_storage().await;

        let name = "test.txt";
        let content = "Test prompt content";

        // 保存 Prompt
        storage.put_prompt(name, content).await.unwrap();

        // 读取 Prompt
        let loaded_content = storage.get_prompt(name).await.unwrap();
        assert_eq!(loaded_content, content);
    }

    #[tokio::test]
    async fn test_list_prompts() {
        let (storage, _temp_dir) = create_test_storage().await;

        // 创建多个 Prompt 文件
        storage.put_prompt("prompt1.txt", "content1").await.unwrap();
        storage.put_prompt("prompt2.md", "content2").await.unwrap();
        storage
            .put_prompt("ignored.json", "content3")
            .await
            .unwrap(); // 应该被忽略

        let prompts = storage.list_prompts().await.unwrap();

        // 只应该列出 .txt 和 .md 文件
        assert_eq!(prompts.len(), 2);
        assert!(prompts.iter().any(|p| p.name == "prompt1.txt"));
        assert!(prompts.iter().any(|p| p.name == "prompt2.md"));
        assert!(!prompts.iter().any(|p| p.name == "ignored.json"));

        // 验证按名称排序
        assert_eq!(prompts[0].name, "prompt1.txt");
        assert_eq!(prompts[1].name, "prompt2.md");
    }

    #[tokio::test]
    async fn test_delete_prompt() {
        let (storage, _temp_dir) = create_test_storage().await;

        let name = "test.txt";
        storage.put_prompt(name, "content").await.unwrap();

        // 验证文件存在
        let prompts = storage.list_prompts().await.unwrap();
        assert_eq!(prompts.len(), 1);

        // 删除 Prompt
        storage.delete_prompt(name).await.unwrap();

        // 验证文件已删除
        let prompts = storage.list_prompts().await.unwrap();
        assert_eq!(prompts.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_prompt() {
        let (storage, _temp_dir) = create_test_storage().await;

        // 删除不存在的文件应该失败
        let result = storage.delete_prompt("nonexistent.txt").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("删除 Prompt 失败"));
    }

    #[tokio::test]
    async fn test_get_nonexistent_prompt() {
        let (storage, _temp_dir) = create_test_storage().await;

        // 读取不存在的 Prompt 应该失败
        let result = storage.get_prompt("nonexistent.txt").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("读取 Prompt 失败"));
    }

    #[tokio::test]
    async fn test_prompt_info_metadata() {
        let (storage, _temp_dir) = create_test_storage().await;

        let name = "test.txt";
        let content = "Test content";
        storage.put_prompt(name, content).await.unwrap();

        let prompts = storage.list_prompts().await.unwrap();
        assert_eq!(prompts.len(), 1);

        let prompt_info = &prompts[0];
        assert_eq!(prompt_info.name, name);
        assert_eq!(prompt_info.size, content.len() as u64);
        // modified_at 应该是最近的时间
        assert!(prompt_info.modified_at <= Utc::now());
    }
}
