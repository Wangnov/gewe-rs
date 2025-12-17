//! 存储工厂和配置

use std::path::PathBuf;
use std::sync::Arc;

use super::{ConfigStorage, FileStorage, PostgresStorage, PromptStorage};

/// 存储后端类型
#[derive(Debug, Clone)]
pub enum StorageBackend {
    File,
    Postgres,
}

/// 存储工厂
pub struct StorageFactory;

impl StorageFactory {
    /// 创建配置存储
    pub async fn create_config_storage(
        backend: StorageBackend,
        config_path: Option<PathBuf>,
        backup_dir: Option<PathBuf>,
        database_url: Option<String>,
    ) -> Result<Arc<dyn ConfigStorage>, String> {
        match backend {
            StorageBackend::File => {
                let config_path = config_path.ok_or("文件存储需要 config_path")?;
                let backup_dir = backup_dir.unwrap_or_else(|| {
                    config_path
                        .parent()
                        .unwrap_or(std::path::Path::new("."))
                        .join("backups")
                });

                let prompts_dir = config_path
                    .parent()
                    .unwrap_or(std::path::Path::new("."))
                    .join("prompts");

                let storage = FileStorage::new(config_path, prompts_dir, backup_dir);
                Ok(Arc::new(storage) as Arc<dyn ConfigStorage>)
            }
            StorageBackend::Postgres => {
                let database_url = database_url.ok_or("Postgres 存储需要 database_url")?;
                let storage = PostgresStorage::new(&database_url).await?;

                // 运行迁移
                storage.run_migrations().await?;

                Ok(Arc::new(storage) as Arc<dyn ConfigStorage>)
            }
        }
    }

    /// 创建 Prompt 存储
    pub async fn create_prompt_storage(
        backend: StorageBackend,
        prompts_dir: Option<PathBuf>,
        database_url: Option<String>,
    ) -> Result<Arc<dyn PromptStorage>, String> {
        match backend {
            StorageBackend::File => {
                let prompts_dir = prompts_dir.ok_or("文件存储需要 prompts_dir")?;
                let config_path = prompts_dir.parent().unwrap().join("bot-app.v2.toml");
                let backup_dir = prompts_dir.parent().unwrap().join("backups");

                let storage = FileStorage::new(config_path, prompts_dir, backup_dir);
                Ok(Arc::new(storage) as Arc<dyn PromptStorage>)
            }
            StorageBackend::Postgres => {
                let database_url = database_url.ok_or("Postgres 存储需要 database_url")?;
                let storage = PostgresStorage::new(&database_url).await?;
                Ok(Arc::new(storage) as Arc<dyn PromptStorage>)
            }
        }
    }
}

/// 从环境变量检测存储后端
pub fn detect_storage_backend() -> StorageBackend {
    if std::env::var("POSTGRES_URL").is_ok() {
        StorageBackend::Postgres
    } else {
        StorageBackend::File
    }
}
