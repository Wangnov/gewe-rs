//! API 共享状态

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// API 共享状态
#[derive(Clone)]
pub struct ApiState {
    inner: Arc<ApiStateInner>,
}

struct ApiStateInner {
    /// 配置文件路径
    config_path: PathBuf,
    /// prompts 目录路径
    prompts_dir: PathBuf,
    /// 备份目录路径
    backup_dir: PathBuf,
    /// 配置元信息
    meta: RwLock<ConfigMeta>,
}

/// 配置元信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// 备份信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    /// 版本号
    pub version: u64,
    /// 备份文件名
    pub filename: String,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 备注
    pub remark: Option<String>,
}

impl ApiState {
    /// 创建新的 API 状态
    pub fn new(config_path: PathBuf, prompts_dir: PathBuf, backup_dir: PathBuf) -> Self {
        Self {
            inner: Arc::new(ApiStateInner {
                config_path,
                prompts_dir,
                backup_dir,
                meta: RwLock::new(ConfigMeta::default()),
            }),
        }
    }

    /// 获取配置文件路径
    pub fn config_path(&self) -> &PathBuf {
        &self.inner.config_path
    }

    /// 获取 prompts 目录路径
    pub fn prompts_dir(&self) -> &PathBuf {
        &self.inner.prompts_dir
    }

    /// 获取备份目录路径
    #[allow(dead_code)]
    pub fn backup_dir(&self) -> &PathBuf {
        &self.inner.backup_dir
    }

    /// 获取元信息的只读访问
    pub async fn get_meta(&self) -> ConfigMeta {
        self.inner.meta.read().await.clone()
    }

    /// 更新元信息
    pub async fn update_meta<F>(&self, f: F)
    where
        F: FnOnce(&mut ConfigMeta),
    {
        let mut meta = self.inner.meta.write().await;
        f(&mut meta);
    }

    /// 初始化状态：读取配置文件计算 ETag，扫描备份目录
    pub async fn initialize(&self) -> anyhow::Result<()> {
        // 计算当前配置的 ETag
        if self.inner.config_path.exists() {
            let content = tokio::fs::read_to_string(&self.inner.config_path).await?;
            let etag = compute_etag(&content);
            self.update_meta(|m| {
                m.etag = etag;
                m.last_reload_at = Some(Utc::now());
                m.last_reload_result = Some("ok".to_string());
            })
            .await;
        }

        // 确保备份目录存在
        tokio::fs::create_dir_all(&self.inner.backup_dir).await?;

        // 扫描备份目录
        self.scan_backups().await?;

        Ok(())
    }

    /// 扫描备份目录，更新可用备份列表
    pub async fn scan_backups(&self) -> anyhow::Result<()> {
        let mut backups = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.inner.backup_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let filename = entry.file_name().to_string_lossy().to_string();
            // 备份文件格式: bot-app.v2.toml.v{version}.{timestamp}
            if filename.starts_with("bot-app.v2.toml.v") && filename.contains('.') {
                if let Some(info) = parse_backup_filename(&filename) {
                    backups.push(info);
                }
            }
        }

        // 按版本号降序排列
        backups.sort_by(|a, b| b.version.cmp(&a.version));

        self.update_meta(|m| {
            m.available_backups = backups;
        })
        .await;

        Ok(())
    }

    /// 创建备份
    pub async fn create_backup(&self, remark: Option<String>) -> anyhow::Result<BackupInfo> {
        let meta = self.get_meta().await;
        let new_version = meta.version + 1;
        let now = Utc::now();
        let timestamp = now.format("%Y%m%d%H%M%S").to_string();
        let filename = format!("bot-app.v2.toml.v{}.{}", new_version, timestamp);
        let backup_path = self.inner.backup_dir.join(&filename);

        // 复制当前配置到备份
        tokio::fs::copy(&self.inner.config_path, &backup_path).await?;

        let info = BackupInfo {
            version: new_version,
            filename,
            created_at: now,
            remark,
        };

        // 更新元信息
        self.update_meta(|m| {
            m.version = new_version;
            m.available_backups.insert(0, info.clone());
            m.last_published_at = Some(now);
        })
        .await;

        Ok(info)
    }

    /// 从备份恢复
    pub async fn restore_backup(&self, version: u64) -> anyhow::Result<()> {
        let meta = self.get_meta().await;
        let backup = meta
            .available_backups
            .iter()
            .find(|b| b.version == version)
            .ok_or_else(|| anyhow::anyhow!("未找到版本 {} 的备份", version))?;

        let backup_path = self.inner.backup_dir.join(&backup.filename);
        if !backup_path.exists() {
            return Err(anyhow::anyhow!("备份文件不存在: {}", backup.filename));
        }

        // 复制备份到配置文件
        tokio::fs::copy(&backup_path, &self.inner.config_path).await?;

        // 重新计算 ETag
        let content = tokio::fs::read_to_string(&self.inner.config_path).await?;
        let etag = compute_etag(&content);

        self.update_meta(|m| {
            m.etag = etag;
            m.has_draft = false;
            m.last_reload_at = Some(Utc::now());
            m.last_reload_result = Some(format!("restored from v{}", version));
        })
        .await;

        Ok(())
    }
}

/// 计算内容的 ETag（SHA256 哈希）
pub fn compute_etag(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// 解析备份文件名，提取版本和时间信息
fn parse_backup_filename(filename: &str) -> Option<BackupInfo> {
    // 格式: bot-app.v2.toml.v{version}.{timestamp}
    // 例如: bot-app.v2.toml.v1.20241204120000
    let parts: Vec<&str> = filename.split('.').collect();
    if parts.len() < 5 {
        return None;
    }

    // 倒数第二部分应该是 "v{version}"
    let version_part = parts.get(parts.len() - 2)?;
    if !version_part.starts_with('v') {
        return None;
    }
    let version: u64 = version_part[1..].parse().ok()?;

    // 最后一部分是时间戳
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_etag() {
        let content = "test content";
        let etag = compute_etag(content);
        assert!(!etag.is_empty());
        assert_eq!(etag.len(), 64); // SHA256 produces 64 hex chars
    }

    #[test]
    fn test_parse_backup_filename() {
        let filename = "bot-app.v2.toml.v1.20241204120000";
        let info = parse_backup_filename(filename);
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.version, 1);
        assert_eq!(info.filename, filename);
    }
}
