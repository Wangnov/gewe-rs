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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_storage_backend_clone() {
        let backend = StorageBackend::File;
        let cloned = backend.clone();
        assert!(matches!(cloned, StorageBackend::File));

        let backend = StorageBackend::Postgres;
        let cloned = backend.clone();
        assert!(matches!(cloned, StorageBackend::Postgres));
    }

    #[test]
    fn test_storage_backend_debug() {
        let backend = StorageBackend::File;
        let debug_str = format!("{:?}", backend);
        assert_eq!(debug_str, "File");

        let backend = StorageBackend::Postgres;
        let debug_str = format!("{:?}", backend);
        assert_eq!(debug_str, "Postgres");
    }

    #[test]
    fn test_detect_storage_backend_default() {
        // 确保环境变量不存在
        env::remove_var("POSTGRES_URL");

        let backend = detect_storage_backend();
        assert!(matches!(backend, StorageBackend::File));
    }

    #[test]
    fn test_detect_storage_backend_postgres() {
        // 设置环境变量
        env::set_var("POSTGRES_URL", "postgres://localhost");

        let backend = detect_storage_backend();
        assert!(matches!(backend, StorageBackend::Postgres));

        // 清理环境变量
        env::remove_var("POSTGRES_URL");
    }

    #[tokio::test]
    async fn test_create_config_storage_file_without_config_path() {
        let result =
            StorageFactory::create_config_storage(StorageBackend::File, None, None, None).await;

        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert!(err_msg.contains("config_path"));
    }

    #[tokio::test]
    async fn test_create_config_storage_file_with_paths() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("bot-app.v2.toml");

        // 创建一个简单的配置文件
        tokio::fs::write(
            &config_path,
            r#"
[info]
name = "test"
version = "1.0.0"
"#,
        )
        .await
        .unwrap();

        let result = StorageFactory::create_config_storage(
            StorageBackend::File,
            Some(config_path.clone()),
            None,
            None,
        )
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_config_storage_postgres_without_url() {
        let result =
            StorageFactory::create_config_storage(StorageBackend::Postgres, None, None, None).await;

        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert!(err_msg.contains("database_url"));
    }

    #[tokio::test]
    async fn test_create_prompt_storage_file_without_prompts_dir() {
        let result = StorageFactory::create_prompt_storage(StorageBackend::File, None, None).await;

        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert!(err_msg.contains("prompts_dir"));
    }

    #[tokio::test]
    async fn test_create_prompt_storage_file_with_prompts_dir() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let prompts_dir = temp_dir.path().join("prompts");
        tokio::fs::create_dir_all(&prompts_dir).await.unwrap();

        // 创建必要的父目录结构
        let config_path = temp_dir.path().join("bot-app.v2.toml");
        tokio::fs::write(&config_path, "").await.unwrap();

        let result =
            StorageFactory::create_prompt_storage(StorageBackend::File, Some(prompts_dir), None)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_prompt_storage_postgres_without_url() {
        let result =
            StorageFactory::create_prompt_storage(StorageBackend::Postgres, None, None).await;

        assert!(result.is_err());
        let err_msg = result.err().unwrap();
        assert!(err_msg.contains("database_url"));
    }
}
