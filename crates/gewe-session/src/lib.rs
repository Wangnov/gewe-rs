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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_context(app_id: &str) -> BotContext {
        BotContext {
            app_id: AppId(app_id.to_string()),
            token: format!("token_{}", app_id),
            webhook_secret: None,
            description: None,
        }
    }

    #[tokio::test]
    async fn test_in_memory_store_default() {
        let store = InMemorySessionStore::default();
        let app_id = AppId("test_app".to_string());
        let result = store.get_session(&app_id).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_in_memory_store_put_and_get_session() {
        let store = InMemorySessionStore::default();
        let ctx = create_test_context("app123");

        store.put_session(ctx.clone()).await;

        let result = store.get_session(&ctx.app_id).await;
        assert!(result.is_some());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.app_id.0, "app123");
        assert_eq!(retrieved.token, "token_app123");
    }

    #[tokio::test]
    async fn test_in_memory_store_overwrite_session() {
        let store = InMemorySessionStore::default();
        let ctx1 = BotContext {
            app_id: AppId("app123".to_string()),
            token: "token1".to_string(),
            webhook_secret: None,
            description: None,
        };
        let ctx2 = BotContext {
            app_id: AppId("app123".to_string()),
            token: "token2".to_string(),
            webhook_secret: Some("secret".to_string()),
            description: Some("updated".to_string()),
        };

        store.put_session(ctx1).await;
        store.put_session(ctx2).await;

        let result = store.get_session(&AppId("app123".to_string())).await;
        assert!(result.is_some());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.token, "token2");
        assert_eq!(retrieved.webhook_secret, Some("secret".to_string()));
    }

    #[tokio::test]
    async fn test_in_memory_store_multiple_sessions() {
        let store = InMemorySessionStore::default();
        let ctx1 = create_test_context("app1");
        let ctx2 = create_test_context("app2");
        let ctx3 = create_test_context("app3");

        store.put_session(ctx1).await;
        store.put_session(ctx2).await;
        store.put_session(ctx3).await;

        assert!(store
            .get_session(&AppId("app1".to_string()))
            .await
            .is_some());
        assert!(store
            .get_session(&AppId("app2".to_string()))
            .await
            .is_some());
        assert!(store
            .get_session(&AppId("app3".to_string()))
            .await
            .is_some());
        assert!(store
            .get_session(&AppId("app4".to_string()))
            .await
            .is_none());
    }

    #[tokio::test]
    async fn test_mark_message_seen_no_session() {
        let store = InMemorySessionStore::default();
        let app_id = AppId("nonexistent".to_string());

        // Should return true for unknown app_id
        let result = store.mark_message_seen(&app_id, 12345).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_mark_message_seen_first_time() {
        let store = InMemorySessionStore::default();
        let ctx = create_test_context("app123");
        store.put_session(ctx.clone()).await;

        // First time seeing this message
        let result = store.mark_message_seen(&ctx.app_id, 12345).await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_mark_message_seen_duplicate() {
        let store = InMemorySessionStore::default();
        let ctx = create_test_context("app123");
        store.put_session(ctx.clone()).await;

        // First time - should return true
        let result1 = store.mark_message_seen(&ctx.app_id, 12345).await;
        assert!(result1);

        // Second time - should return false (duplicate)
        let result2 = store.mark_message_seen(&ctx.app_id, 12345).await;
        assert!(!result2);
    }

    #[tokio::test]
    async fn test_mark_message_seen_different_messages() {
        let store = InMemorySessionStore::default();
        let ctx = create_test_context("app123");
        store.put_session(ctx.clone()).await;

        let result1 = store.mark_message_seen(&ctx.app_id, 1).await;
        let result2 = store.mark_message_seen(&ctx.app_id, 2).await;
        let result3 = store.mark_message_seen(&ctx.app_id, 3).await;

        assert!(result1);
        assert!(result2);
        assert!(result3);

        // Check duplicates
        assert!(!store.mark_message_seen(&ctx.app_id, 1).await);
        assert!(!store.mark_message_seen(&ctx.app_id, 2).await);
        assert!(!store.mark_message_seen(&ctx.app_id, 3).await);
    }

    #[tokio::test]
    async fn test_mark_message_seen_max_capacity() {
        let store = InMemorySessionStore::default();
        let ctx = create_test_context("app123");
        store.put_session(ctx.clone()).await;

        // Add more than MAX_SEEN (1024) messages
        for i in 0..1030 {
            store.mark_message_seen(&ctx.app_id, i).await;
        }

        // The oldest messages should have been evicted
        // Message 0-5 should be gone (evicted to make room)
        // Message 6+ should still be there
        let result_old = store.mark_message_seen(&ctx.app_id, 0).await;
        assert!(result_old); // Should return true as if first seen (was evicted)

        let result_recent = store.mark_message_seen(&ctx.app_id, 1029).await;
        assert!(!result_recent); // Should return false (still in cache)
    }

    #[tokio::test]
    async fn test_mark_message_seen_different_apps() {
        let store = InMemorySessionStore::default();
        let ctx1 = create_test_context("app1");
        let ctx2 = create_test_context("app2");
        store.put_session(ctx1.clone()).await;
        store.put_session(ctx2.clone()).await;

        // Same message ID for different apps should both be first seen
        let result1 = store.mark_message_seen(&ctx1.app_id, 12345).await;
        let result2 = store.mark_message_seen(&ctx2.app_id, 12345).await;

        assert!(result1);
        assert!(result2);

        // But duplicates within same app should be caught
        assert!(!store.mark_message_seen(&ctx1.app_id, 12345).await);
        assert!(!store.mark_message_seen(&ctx2.app_id, 12345).await);
    }

    #[tokio::test]
    async fn test_in_memory_store_clone() {
        let store1 = InMemorySessionStore::default();
        let ctx = create_test_context("app123");
        store1.put_session(ctx.clone()).await;

        let store2 = store1.clone();

        // Both stores should share the same data
        let result1 = store1.get_session(&ctx.app_id).await;
        let result2 = store2.get_session(&ctx.app_id).await;

        assert!(result1.is_some());
        assert!(result2.is_some());
        assert_eq!(result1.unwrap().token, result2.unwrap().token);
    }

    #[tokio::test]
    async fn test_stored_entry_serialize_deserialize() {
        let entry = StoredEntry {
            context: create_test_context("app123"),
            seen: VecDeque::from([1, 2, 3]),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: StoredEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.context.app_id.0, "app123");
        assert_eq!(deserialized.seen.len(), 3);
        assert!(deserialized.seen.contains(&1));
        assert!(deserialized.seen.contains(&2));
        assert!(deserialized.seen.contains(&3));
    }

    #[tokio::test]
    async fn test_stored_entry_default_seen() {
        // Test that seen defaults to empty when deserializing without it
        let json = r#"{"context":{"appId":"app123","token":"token"}}"#;
        let entry: StoredEntry = serde_json::from_str(json).unwrap();
        assert!(entry.seen.is_empty());
    }

    #[test]
    fn test_bot_registry_type() {
        // Test that BotRegistry can be created
        let registry: BotRegistry = Arc::new(RwLock::new(HashMap::new()));
        assert!(Arc::strong_count(&registry) == 1);
    }

    #[tokio::test]
    async fn test_bot_registry_operations() {
        let registry: BotRegistry = Arc::new(RwLock::new(HashMap::new()));
        let ctx = create_test_context("app123");

        // Insert
        {
            let mut map = registry.write().await;
            map.insert(ctx.app_id.clone(), ctx.clone());
        }

        // Read
        {
            let map = registry.read().await;
            let retrieved = map.get(&ctx.app_id);
            assert!(retrieved.is_some());
            assert_eq!(retrieved.unwrap().token, "token_app123");
        }
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        use std::sync::Arc;

        let store = Arc::new(InMemorySessionStore::default());
        let ctx = create_test_context("app123");
        store.put_session(ctx.clone()).await;

        let store1 = Arc::clone(&store);
        let store2 = Arc::clone(&store);
        let app_id1 = ctx.app_id.clone();
        let app_id2 = ctx.app_id.clone();

        let handle1 = tokio::spawn(async move {
            for i in 0..100 {
                store1.mark_message_seen(&app_id1, i).await;
            }
        });

        let handle2 = tokio::spawn(async move {
            for i in 100..200 {
                store2.mark_message_seen(&app_id2, i).await;
            }
        });

        handle1.await.unwrap();
        handle2.await.unwrap();

        // Both ranges should have been processed
        assert!(!store.mark_message_seen(&ctx.app_id, 50).await);
        assert!(!store.mark_message_seen(&ctx.app_id, 150).await);
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
