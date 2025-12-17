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
