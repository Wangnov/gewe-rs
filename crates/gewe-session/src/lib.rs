use async_trait::async_trait;
use gewe_core::{AppId, BotContext};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

pub type BotRegistry = Arc<RwLock<HashMap<AppId, BotContext>>>;

#[async_trait]
pub trait SessionStore: Send + Sync {
    async fn get_session(&self, app_id: &AppId) -> Option<BotContext>;
    async fn put_session(&self, context: BotContext);
    /// returns true if this message id is first seen
    async fn mark_message_seen(&self, app_id: &AppId, new_msg_id: i64) -> bool;
}

#[derive(Clone, Default)]
pub struct InMemorySessionStore {
    inner: Arc<RwLock<HashMap<AppId, StoredEntry>>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct StoredEntry {
    context: BotContext,
    #[serde(default)]
    seen: VecDeque<i64>,
}

#[async_trait]
impl SessionStore for InMemorySessionStore {
    async fn get_session(&self, app_id: &AppId) -> Option<BotContext> {
        let map: tokio::sync::RwLockReadGuard<'_, HashMap<AppId, StoredEntry>> =
            self.inner.read().await;
        map.get(app_id).map(|entry| entry.context.clone())
    }

    async fn put_session(&self, context: BotContext) {
        let mut map: tokio::sync::RwLockWriteGuard<'_, HashMap<AppId, StoredEntry>> =
            self.inner.write().await;
        map.insert(
            context.app_id.clone(),
            StoredEntry {
                context,
                seen: VecDeque::new(),
            },
        );
    }

    async fn mark_message_seen(&self, app_id: &AppId, new_msg_id: i64) -> bool {
        let mut map: tokio::sync::RwLockWriteGuard<'_, HashMap<AppId, StoredEntry>> =
            self.inner.write().await;
        let entry = match map.get_mut(app_id) {
            Some(entry) => entry,
            None => return true,
        };
        if entry.seen.contains(&new_msg_id) {
            return false;
        }

        entry.seen.push_back(new_msg_id);
        // 防止无限增长，简单裁剪
        const MAX_SEEN: usize = 1024;
        if entry.seen.len() > MAX_SEEN {
            entry.seen.pop_front();
        }
        true
    }
}

#[cfg(feature = "sqlite")]
pub mod sqlite_store {
    use super::{AppId, BotContext, SessionStore, StoredEntry};
    use async_trait::async_trait;
    use serde_json;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
    use std::collections::VecDeque;
    use std::time::Duration;

    #[derive(Clone)]
    pub struct SqliteSessionStore {
        pool: SqlitePool,
    }

    impl SqliteSessionStore {
        pub async fn connect(database_url: &str) -> sqlx::Result<Self> {
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .acquire_timeout(Duration::from_secs(5))
                .connect(database_url)
                .await?;
            sqlx::query(
                r#"
CREATE TABLE IF NOT EXISTS sessions (
    app_id TEXT PRIMARY KEY,
    payload TEXT NOT NULL
);
"#,
            )
            .execute(&pool)
            .await?;
            Ok(Self { pool })
        }

        async fn load_entry(&self, app_id: &AppId) -> Option<StoredEntry> {
            let row: Option<(String,)> =
                sqlx::query_as("SELECT payload FROM sessions WHERE app_id = ?")
                    .bind(&app_id.0)
                    .fetch_optional(&self.pool)
                    .await
                    .ok()?;
            row.and_then(|(payload,)| serde_json::from_str::<StoredEntry>(&payload).ok())
        }
    }

    #[async_trait]
    impl SessionStore for SqliteSessionStore {
        async fn get_session(&self, app_id: &AppId) -> Option<BotContext> {
            self.load_entry(app_id).await.map(|entry| entry.context)
        }

        async fn put_session(&self, context: BotContext) {
            let entry = StoredEntry {
                context,
                seen: VecDeque::new(),
            };
            let payload = match serde_json::to_string(&entry) {
                Ok(p) => p,
                Err(err) => {
                    tracing::warn!(?err, "failed to serialize session");
                    return;
                }
            };
            let _ = sqlx::query("INSERT OR REPLACE INTO sessions (app_id, payload) VALUES (?, ?)")
                .bind(&entry.context.app_id.0)
                .bind(payload)
                .execute(&self.pool)
                .await;
        }

        async fn mark_message_seen(&self, app_id: &AppId, new_msg_id: i64) -> bool {
            let mut entry = match self.load_entry(app_id).await {
                Some(entry) => entry,
                None => return true,
            };
            if entry.seen.contains(&new_msg_id) {
                return false;
            }
            entry.seen.push_back(new_msg_id);
            const MAX_SEEN: usize = 1024;
            if entry.seen.len() > MAX_SEEN {
                entry.seen.pop_front();
            }
            let payload = match serde_json::to_string(&entry) {
                Ok(p) => p,
                Err(_) => return true,
            };
            let _ = sqlx::query("INSERT OR REPLACE INTO sessions (app_id, payload) VALUES (?, ?)")
                .bind(&entry.context.app_id.0)
                .bind(payload)
                .execute(&self.pool)
                .await;
            true
        }
    }
}

#[cfg(feature = "redis-store")]
pub mod redis_store {
    use super::{AppId, BotContext, SessionStore, StoredEntry};
    use async_trait::async_trait;
    use redis::{AsyncCommands, Client};
    use serde_json;
    use std::collections::VecDeque;

    #[derive(Clone)]
    pub struct RedisSessionStore {
        client: Client,
        prefix: String,
    }

    impl RedisSessionStore {
        pub fn new(url: &str, prefix: impl Into<String>) -> redis::RedisResult<Self> {
            Ok(Self {
                client: Client::open(url)?,
                prefix: prefix.into(),
            })
        }

        fn key(&self, app_id: &AppId) -> String {
            format!("{}:{}", self.prefix, app_id.0)
        }

        async fn load_entry(&self, app_id: &AppId) -> Option<StoredEntry> {
            let mut conn = self.client.get_multiplexed_async_connection().await.ok()?;
            let payload: Option<String> = conn.get(self.key(app_id)).await.ok()?;
            payload.and_then(|p| serde_json::from_str::<StoredEntry>(&p).ok())
        }
    }

    #[async_trait]
    impl SessionStore for RedisSessionStore {
        async fn get_session(&self, app_id: &AppId) -> Option<BotContext> {
            self.load_entry(app_id).await.map(|entry| entry.context)
        }

        async fn put_session(&self, context: BotContext) {
            let entry = StoredEntry {
                context,
                seen: VecDeque::new(),
            };
            if let Ok(payload) = serde_json::to_string(&entry) {
                if let Ok(mut conn) = self.client.get_multiplexed_async_connection().await {
                    let _: redis::RedisResult<()> =
                        conn.set(self.key(&entry.context.app_id), payload).await;
                }
            }
        }

        async fn mark_message_seen(&self, app_id: &AppId, new_msg_id: i64) -> bool {
            // Fetch and update atomically best-effort; simple get/set for now.
            let mut entry = match self.load_entry(app_id).await {
                Some(entry) => entry,
                None => return true,
            };
            if entry.seen.contains(&new_msg_id) {
                return false;
            }
            entry.seen.push_back(new_msg_id);
            const MAX_SEEN: usize = 1024;
            if entry.seen.len() > MAX_SEEN {
                entry.seen.pop_front();
            }

            if let Ok(payload) = serde_json::to_string(&entry) {
                if let Ok(mut conn) = self.client.get_multiplexed_async_connection().await {
                    let _: redis::RedisResult<()> = conn.set(self.key(app_id), payload).await;
                }
            }
            true
        }
    }
}
