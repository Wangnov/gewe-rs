//! 配置存储抽象层
//!
//! 定义统一的存储接口，支持文件存储和 Postgres 存储
//!
//! 注意：存储抽象层当前为预留功能，待后续完整集成

#![allow(dead_code)]

mod factory;
mod file;
mod postgres;

pub use file::FileStorage;
pub use postgres::PostgresStorage;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::config::AppConfigV2;

/// 配置元信息
#[derive(Debug, Clone)]
pub struct ConfigMeta {
    /// 当前版本号
    pub version: u64,
    /// 当前配置的 ETag（内容哈希）
    pub etag: String,
    /// 是否有未发布的草稿
    pub has_draft: bool,
    /// 最后发布时间
    pub last_published_at: Option<DateTime<Utc>>,
    /// 最后保存时间
    pub last_saved_at: Option<DateTime<Utc>>,
    /// 最后 reload 时间
    pub last_reload_at: Option<DateTime<Utc>>,
    /// 最后 reload 结果
    pub last_reload_result: Option<String>,
    /// 可用的备份版本列表
    pub available_backups: Vec<BackupInfo>,
}

/// 备份/版本信息
#[derive(Debug, Clone)]
pub struct BackupInfo {
    /// 版本号
    pub version: u64,
    /// 备份文件名或标识
    pub filename: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 备注
    pub remark: Option<String>,
}

/// 配置存储接口
#[async_trait]
pub trait ConfigStorage: Send + Sync {
    /// 获取当前配置
    async fn get_current(&self) -> Result<AppConfigV2, String>;

    /// 保存草稿（不发布）
    async fn save_draft(&self, config: &AppConfigV2) -> Result<String, String>;

    /// 发布版本（创建备份/版本记录）
    async fn publish(&self, remark: Option<String>) -> Result<BackupInfo, String>;

    /// 回滚到指定版本
    async fn rollback(&self, version: u64) -> Result<(), String>;

    /// 获取元信息
    async fn get_meta(&self) -> Result<ConfigMeta, String>;

    /// 扫描可用的备份/版本列表
    async fn scan_backups(&self) -> Result<Vec<BackupInfo>, String>;
}

/// Prompt 存储接口
#[async_trait]
pub trait PromptStorage: Send + Sync {
    /// 列出所有 Prompt
    async fn list_prompts(&self) -> Result<Vec<PromptInfo>, String>;

    /// 获取 Prompt 内容
    async fn get_prompt(&self, name: &str) -> Result<String, String>;

    /// 保存 Prompt
    async fn put_prompt(&self, name: &str, content: &str) -> Result<(), String>;

    /// 删除 Prompt
    async fn delete_prompt(&self, name: &str) -> Result<(), String>;
}

/// Prompt 信息
#[derive(Debug, Clone)]
pub struct PromptInfo {
    pub name: String,
    pub size: u64,
    pub modified_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_meta_new() {
        // 测试创建 ConfigMeta
        let meta = ConfigMeta {
            version: 1,
            etag: "abc123".to_string(),
            has_draft: false,
            last_published_at: None,
            last_saved_at: None,
            last_reload_at: None,
            last_reload_result: None,
            available_backups: vec![],
        };

        assert_eq!(meta.version, 1);
        assert_eq!(meta.etag, "abc123");
        assert!(!meta.has_draft);
    }

    #[test]
    fn test_backup_info_new() {
        // 测试创建 BackupInfo
        let now = Utc::now();
        let backup = BackupInfo {
            version: 1,
            filename: "backup-v1.toml".to_string(),
            created_at: now,
            remark: Some("Initial backup".to_string()),
        };

        assert_eq!(backup.version, 1);
        assert_eq!(backup.filename, "backup-v1.toml");
        assert!(backup.remark.is_some());
    }

    #[test]
    fn test_prompt_info_new() {
        // 测试创建 PromptInfo
        let now = Utc::now();
        let prompt = PromptInfo {
            name: "test.txt".to_string(),
            size: 1024,
            modified_at: now,
        };

        assert_eq!(prompt.name, "test.txt");
        assert_eq!(prompt.size, 1024);
    }

    #[test]
    fn test_config_meta_with_backups() {
        // 测试带备份的 ConfigMeta
        let backup = BackupInfo {
            version: 1,
            filename: "backup-v1.toml".to_string(),
            created_at: Utc::now(),
            remark: None,
        };

        let meta = ConfigMeta {
            version: 2,
            etag: "xyz789".to_string(),
            has_draft: true,
            last_published_at: Some(Utc::now()),
            last_saved_at: Some(Utc::now()),
            last_reload_at: None,
            last_reload_result: None,
            available_backups: vec![backup],
        };

        assert_eq!(meta.available_backups.len(), 1);
        assert_eq!(meta.available_backups[0].version, 1);
        assert!(meta.has_draft);
    }

    #[test]
    fn test_backup_info_clone() {
        // 测试 BackupInfo Clone
        let original = BackupInfo {
            version: 5,
            filename: "test.toml".to_string(),
            created_at: Utc::now(),
            remark: Some("test remark".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(cloned.version, original.version);
        assert_eq!(cloned.filename, original.filename);
        assert_eq!(cloned.remark, original.remark);
    }

    #[test]
    fn test_prompt_info_clone() {
        // 测试 PromptInfo Clone
        let original = PromptInfo {
            name: "prompt.txt".to_string(),
            size: 2048,
            modified_at: Utc::now(),
        };

        let cloned = original.clone();
        assert_eq!(cloned.name, original.name);
        assert_eq!(cloned.size, original.size);
    }
}
