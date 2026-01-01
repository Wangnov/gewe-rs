use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use gewe_core::{AppId, BotContext};
use gewe_session::SessionStore;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::sync::mpsc;
use tracing::instrument;

#[derive(Clone)]
pub struct WebhookState<S> {
    pub store: Arc<S>,
    pub tx: mpsc::Sender<WebhookEvent>,
}

// tokio mpsc Sender is Send + Sync when message is Send; expose bounds on state type for axum.
unsafe impl<S: Send + Sync> Send for WebhookState<S> {}
unsafe impl<S: Send + Sync> Sync for WebhookState<S> {}

pub struct WebhookBuilderOptions {
    pub queue_size: usize,
}

impl Default for WebhookBuilderOptions {
    fn default() -> Self {
        Self { queue_size: 1024 }
    }
}

#[derive(Debug, Clone)]
pub struct WebhookEvent {
    pub app_id: AppId,
    pub type_name: Option<String>,
    pub data: serde_json::Value,
}

pub fn router_with_channel<S>(opts: WebhookBuilderOptions) -> (Router, mpsc::Receiver<WebhookEvent>)
where
    S: SessionStore + Default + Send + Sync + Clone + 'static,
{
    router_with_channel_and_store::<S>(opts, Arc::new(Default::default()))
}

/// 返回 Router、事件接收端及可写的 SessionStore 句柄，方便在应用启动时注册机器人上下文。
pub fn router_with_channel_and_state<S>(
    opts: WebhookBuilderOptions,
) -> (Router, mpsc::Receiver<WebhookEvent>, Arc<S>)
where
    S: SessionStore + Default + Send + Sync + Clone + 'static,
{
    let store = Arc::new(Default::default());
    let (router, rx) = router_with_channel_and_store(opts, Arc::clone(&store));
    (router, rx, store)
}

/// 构建携带自定义 `SessionStore` 的 webhook，便于外部提前写入 BotContext。
pub fn router_with_channel_and_store<S>(
    opts: WebhookBuilderOptions,
    store: Arc<S>,
) -> (Router, mpsc::Receiver<WebhookEvent>)
where
    S: SessionStore + Send + Sync + Clone + 'static,
{
    let (tx, rx) = mpsc::channel(opts.queue_size);
    let state = WebhookState { store, tx };
    let router: Router<()> = Router::new()
        .route(
            "/webhook",
            post(
                |State(state): State<WebhookState<S>>, headers: HeaderMap, body: Bytes| async move {
                    handle_webhook::<S>(state, headers, body).await
                },
            ),
        )
        .with_state(state);
    (router, rx)
}

// 兼容旧接口：默认队列大小 1024，丢弃接收端。
pub fn router<S>() -> Router
where
    S: SessionStore + Default + Send + Sync + Clone + 'static,
{
    let (router, _rx) = router_with_channel::<S>(WebhookBuilderOptions::default());
    router
}

#[derive(Debug, Deserialize)]
struct WebhookBody {
    #[serde(rename = "Appid")]
    appid: String,
    #[serde(rename = "Data")]
    data: serde_json::Value,
    #[serde(rename = "TypeName")]
    type_name: Option<String>,
}

#[instrument(skip(state, headers, raw_body))]
async fn handle_webhook<S>(
    state: WebhookState<S>,
    headers: HeaderMap,
    raw_body: Bytes,
) -> impl IntoResponse
where
    S: SessionStore + Send + Sync + 'static,
{
    log_request_pre_parse(&headers, &raw_body);
    if capture_only() {
        return StatusCode::OK;
    }

    if is_ping(&raw_body) {
        tracing::info!("webhook ping: {}", String::from_utf8_lossy(&raw_body));
        return StatusCode::OK;
    }

    let body: WebhookBody = match serde_json::from_slice(&raw_body) {
        Ok(v) => v,
        Err(err) => {
            log_raw_invalid_body(&raw_body);
            tracing::warn!(?err, "invalid webhook body");
            return StatusCode::BAD_REQUEST;
        }
    };
    maybe_dump_raw(&body.appid, &raw_body).await;

    let app_id = AppId(body.appid.clone());
    let Some(ctx) = state.store.get_session(&app_id).await else {
        tracing::warn!("unknown app_id for webhook");
        return StatusCode::UNAUTHORIZED;
    };

    if require_signature() {
        if let Err(err) = verify_signature(&headers, &ctx, &raw_body) {
            log_headers_on_verify_fail(&headers);
            log_raw_on_verify_fail(&raw_body);
            tracing::warn!(?err, "webhook signature verify failed");
            return StatusCode::UNAUTHORIZED;
        }
    }

    if let Some(mid) = extract_new_msg_id(&body.data) {
        if !state.store.mark_message_seen(&app_id, mid).await {
            return StatusCode::OK;
        }
    }

    // 投递到异步队列，避免阻塞 3s SLA
    if let Err(err) = state.tx.try_send(WebhookEvent {
        app_id,
        type_name: body.type_name,
        data: body.data,
    }) {
        tracing::warn!(?err, "webhook queue full; dropping event");
    }

    StatusCode::OK
}

fn dump_dir() -> Option<String> {
    static DUMP_DIR: OnceLock<Option<String>> = OnceLock::new();
    DUMP_DIR
        .get_or_init(|| match std::env::var("GEWE_WEBHOOK_DUMP_DIR") {
            Ok(v) if !v.trim().is_empty() => Some(v),
            _ => None,
        })
        .clone()
}

async fn maybe_dump_raw(appid: &str, raw_body: &[u8]) {
    let Some(dir) = dump_dir() else {
        return;
    };
    if let Err(err) = fs::create_dir_all(&dir).await {
        tracing::warn!(?err, %dir, "create dump dir failed");
        return;
    }
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let path = format!("{}/{}_{}.json", dir.trim_end_matches('/'), ts, appid);
    if let Err(err) = fs::write(&path, raw_body).await {
        tracing::warn!(?err, %path, "write webhook dump failed");
    } else {
        tracing::info!(%path, %appid, "webhook raw dumped");
    }
}

fn log_raw_invalid_body(raw_body: &[u8]) {
    if debug_raw_enabled() {
        let body_str = String::from_utf8_lossy(raw_body);
        tracing::warn!(%body_str, "webhook raw body (invalid)");
    }
}

fn log_raw_on_verify_fail(raw_body: &[u8]) {
    if debug_raw_enabled() {
        let body_str = String::from_utf8_lossy(raw_body);
        tracing::warn!(%body_str, "webhook raw body (signature failed)");
    }
}

fn is_ping(raw_body: &[u8]) -> bool {
    if !raw_body.contains(&b't') {
        return false;
    }
    match serde_json::from_slice::<serde_json::Value>(raw_body) {
        Ok(val) => val.get("testMsg").is_some(),
        Err(_) => false,
    }
}

fn debug_raw_enabled() -> bool {
    static LOG_RAW: OnceLock<bool> = OnceLock::new();
    *LOG_RAW.get_or_init(|| match std::env::var("GEWE_WEBHOOK_DEBUG_RAW") {
        Ok(v) => matches!(v.as_str(), "1" | "true" | "TRUE" | "True"),
        Err(_) => false,
    })
}

fn capture_only() -> bool {
    static CAPTURE_ONLY: OnceLock<bool> = OnceLock::new();
    *CAPTURE_ONLY.get_or_init(|| match std::env::var("GEWE_WEBHOOK_CAPTURE_ONLY") {
        Ok(v) => matches!(v.as_str(), "1" | "true" | "TRUE" | "True"),
        Err(_) => false,
    })
}

fn require_signature() -> bool {
    static REQUIRE: OnceLock<bool> = OnceLock::new();
    *REQUIRE.get_or_init(|| match std::env::var("GEWE_WEBHOOK_REQUIRE_SIGNATURE") {
        Ok(v) => matches!(v.as_str(), "1" | "true" | "TRUE" | "True"),
        Err(_) => false,
    })
}

fn log_request_pre_parse(headers: &HeaderMap, raw_body: &[u8]) {
    if !(debug_raw_enabled() || capture_only()) {
        return;
    }
    let body_str = String::from_utf8_lossy(raw_body);
    tracing::info!(?headers, %body_str, "webhook request (pre-parse)");
}

fn log_headers_on_verify_fail(headers: &HeaderMap) {
    static LOG_HEADERS: OnceLock<bool> = OnceLock::new();
    let enabled = *LOG_HEADERS.get_or_init(|| match std::env::var("GEWE_WEBHOOK_DEBUG_HEADERS") {
        Ok(v) => matches!(v.as_str(), "1" | "true" | "TRUE" | "True"),
        Err(_) => false,
    });
    if !enabled {
        return;
    }
    let token = headers
        .get("X-GEWE-TOKEN")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("<missing>");
    let timestamp = headers
        .get("X-GEWE-TIMESTAMP")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("<missing>");
    let sign_present = headers.contains_key("X-GEWE-SIGN");
    tracing::warn!(
        %token,
        %timestamp,
        sign_present,
        "webhook signature headers (debug; no body logged)"
    );
}

#[derive(Debug)]
enum SignatureError {
    MissingHeader,
    InvalidTimestamp,
    Stale,
    InvalidHex,
    VerifyFailed,
}

fn verify_signature(
    headers: &HeaderMap,
    ctx: &BotContext,
    body: &[u8],
) -> Result<(), SignatureError> {
    let ts_header = headers
        .get("X-GEWE-TIMESTAMP")
        .ok_or(SignatureError::MissingHeader)?
        .to_str()
        .map_err(|_| SignatureError::InvalidTimestamp)?;
    let ts: i64 = ts_header
        .parse()
        .map_err(|_| SignatureError::InvalidTimestamp)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SignatureError::InvalidTimestamp)?
        .as_secs() as i64;
    const MAX_SKEW: i64 = 300;
    if (now - ts).abs() > MAX_SKEW {
        return Err(SignatureError::Stale);
    }

    let provided = headers
        .get("X-GEWE-SIGN")
        .ok_or(SignatureError::MissingHeader)?
        .to_str()
        .map_err(|_| SignatureError::VerifyFailed)?;

    let token_header = headers
        .get("X-GEWE-TOKEN")
        .ok_or(SignatureError::MissingHeader)?
        .to_str()
        .map_err(|_| SignatureError::VerifyFailed)?;
    if token_header != ctx.token {
        return Err(SignatureError::VerifyFailed);
    }

    let secret = ctx.webhook_secret.as_ref().unwrap_or(&ctx.token);

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| SignatureError::VerifyFailed)?;
    mac.update(ts_header.as_bytes());
    mac.update(b":");
    mac.update(body);
    let expected_bytes = mac.finalize().into_bytes();
    let provided_bytes = hex::decode(provided).map_err(|_| SignatureError::InvalidHex)?;
    if expected_bytes.as_slice() == provided_bytes.as_slice() {
        Ok(())
    } else {
        Err(SignatureError::VerifyFailed)
    }
}

fn extract_new_msg_id(data: &serde_json::Value) -> Option<i64> {
    data.get("NewMsgId").and_then(|v| v.as_i64()).or_else(|| {
        data.get("Data")
            .and_then(|inner| inner.get("NewMsgId"))
            .and_then(|v| v.as_i64())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gewe_session::InMemorySessionStore;

    fn create_test_context(app_id: &str, token: &str) -> BotContext {
        BotContext {
            app_id: AppId(app_id.to_string()),
            token: token.to_string(),
            webhook_secret: None,
            description: None,
        }
    }

    fn create_test_context_with_secret(app_id: &str, token: &str, secret: &str) -> BotContext {
        BotContext {
            app_id: AppId(app_id.to_string()),
            token: token.to_string(),
            webhook_secret: Some(secret.to_string()),
            description: None,
        }
    }

    // ===== WebhookBuilderOptions tests =====
    #[test]
    fn test_webhook_builder_options_default() {
        let opts = WebhookBuilderOptions::default();
        assert_eq!(opts.queue_size, 1024);
    }

    #[test]
    fn test_webhook_builder_options_custom() {
        let opts = WebhookBuilderOptions { queue_size: 512 };
        assert_eq!(opts.queue_size, 512);
    }

    // ===== WebhookEvent tests =====
    #[test]
    fn test_webhook_event_debug() {
        let event = WebhookEvent {
            app_id: AppId("app123".to_string()),
            type_name: Some("message".to_string()),
            data: serde_json::json!({"test": "data"}),
        };
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("app123"));
        assert!(debug_str.contains("message"));
    }

    #[test]
    fn test_webhook_event_clone() {
        let event = WebhookEvent {
            app_id: AppId("app123".to_string()),
            type_name: Some("message".to_string()),
            data: serde_json::json!({"key": "value"}),
        };
        let cloned = event.clone();
        assert_eq!(event.app_id.0, cloned.app_id.0);
        assert_eq!(event.type_name, cloned.type_name);
    }

    // ===== WebhookBody tests =====
    #[test]
    fn test_webhook_body_deserialize() {
        let json = r#"{"Appid":"app123","Data":{"test":"data"},"TypeName":"message"}"#;
        let body: WebhookBody = serde_json::from_str(json).unwrap();
        assert_eq!(body.appid, "app123");
        assert_eq!(body.type_name, Some("message".to_string()));
        assert!(body.data.get("test").is_some());
    }

    #[test]
    fn test_webhook_body_deserialize_without_typename() {
        let json = r#"{"Appid":"app123","Data":{"test":"data"}}"#;
        let body: WebhookBody = serde_json::from_str(json).unwrap();
        assert_eq!(body.appid, "app123");
        assert_eq!(body.type_name, None);
    }

    // ===== is_ping tests =====
    #[test]
    fn test_is_ping_with_test_msg() {
        let body = br#"{"testMsg":"hello"}"#;
        assert!(is_ping(body));
    }

    #[test]
    fn test_is_ping_without_test_msg() {
        let body = br#"{"Appid":"app123","Data":{}}"#;
        assert!(!is_ping(body));
    }

    #[test]
    fn test_is_ping_invalid_json() {
        let body = br#"not json"#;
        assert!(!is_ping(body));
    }

    #[test]
    fn test_is_ping_no_t_character() {
        // Optimization: if no 't' character, skip JSON parsing
        let body = br#"{"Appid":"app123"}"#;
        assert!(!is_ping(body));
    }

    // ===== extract_new_msg_id tests =====
    #[test]
    fn test_extract_new_msg_id_direct() {
        let data = serde_json::json!({"NewMsgId": 12345});
        assert_eq!(extract_new_msg_id(&data), Some(12345));
    }

    #[test]
    fn test_extract_new_msg_id_nested() {
        let data = serde_json::json!({"Data": {"NewMsgId": 67890}});
        assert_eq!(extract_new_msg_id(&data), Some(67890));
    }

    #[test]
    fn test_extract_new_msg_id_none() {
        let data = serde_json::json!({"other": "field"});
        assert_eq!(extract_new_msg_id(&data), None);
    }

    #[test]
    fn test_extract_new_msg_id_prefers_direct() {
        let data = serde_json::json!({
            "NewMsgId": 111,
            "Data": {"NewMsgId": 222}
        });
        assert_eq!(extract_new_msg_id(&data), Some(111));
    }

    // ===== verify_signature tests =====
    #[test]
    fn test_verify_signature_missing_timestamp() {
        let headers = HeaderMap::new();
        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::MissingHeader)));
    }

    #[test]
    fn test_verify_signature_invalid_timestamp() {
        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", "not_a_number".parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        headers.insert("X-GEWE-SIGN", "somesign".parse().unwrap());

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::InvalidTimestamp)));
    }

    #[test]
    fn test_verify_signature_stale_timestamp() {
        let mut headers = HeaderMap::new();
        // Very old timestamp
        headers.insert("X-GEWE-TIMESTAMP", "1000000000".parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        headers.insert("X-GEWE-SIGN", "somesign".parse().unwrap());

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::Stale)));
    }

    #[test]
    fn test_verify_signature_missing_sign() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::MissingHeader)));
    }

    #[test]
    fn test_verify_signature_token_mismatch() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "wrong_token".parse().unwrap());
        headers.insert("X-GEWE-SIGN", "somesign".parse().unwrap());

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::VerifyFailed)));
    }

    #[test]
    fn test_verify_signature_invalid_hex() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        headers.insert("X-GEWE-SIGN", "not_valid_hex_zzz".parse().unwrap());

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::InvalidHex)));
    }

    #[test]
    fn test_verify_signature_valid() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_str = now.to_string();
        let body = b"test body";
        let token = "token123";

        // Calculate expected signature
        let mut mac = Hmac::<Sha256>::new_from_slice(token.as_bytes()).unwrap();
        mac.update(now_str.as_bytes());
        mac.update(b":");
        mac.update(body);
        let signature = hex::encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now_str.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", token.parse().unwrap());
        headers.insert("X-GEWE-SIGN", signature.parse().unwrap());

        let ctx = create_test_context("app123", token);

        let result = verify_signature(&headers, &ctx, body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_signature_with_webhook_secret() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_str = now.to_string();
        let body = b"test body";
        let token = "token123";
        let secret = "webhook_secret";

        // Calculate expected signature using webhook_secret
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(now_str.as_bytes());
        mac.update(b":");
        mac.update(body);
        let signature = hex::encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now_str.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", token.parse().unwrap());
        headers.insert("X-GEWE-SIGN", signature.parse().unwrap());

        let ctx = create_test_context_with_secret("app123", token, secret);

        let result = verify_signature(&headers, &ctx, body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_signature_wrong_signature() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_str = now.to_string();
        let body = b"test body";
        let token = "token123";

        // Wrong signature (valid hex but incorrect value)
        let wrong_signature = hex::encode([0u8; 32]);

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now_str.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", token.parse().unwrap());
        headers.insert("X-GEWE-SIGN", wrong_signature.parse().unwrap());

        let ctx = create_test_context("app123", token);

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::VerifyFailed)));
    }

    // ===== SignatureError Debug test =====
    #[test]
    fn test_signature_error_debug() {
        let err = SignatureError::MissingHeader;
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("MissingHeader"));

        let err2 = SignatureError::Stale;
        let debug_str2 = format!("{:?}", err2);
        assert!(debug_str2.contains("Stale"));
    }

    // ===== Router construction tests =====
    #[tokio::test]
    async fn test_router_with_channel() {
        let opts = WebhookBuilderOptions { queue_size: 100 };
        let (_router, _rx) = router_with_channel::<InMemorySessionStore>(opts);
        // Just verify it compiles and creates without panic
    }

    #[tokio::test]
    async fn test_router_with_channel_and_state() {
        let opts = WebhookBuilderOptions { queue_size: 100 };
        let (_router, _rx, store) = router_with_channel_and_state::<InMemorySessionStore>(opts);

        // Verify store is accessible and works
        let ctx = create_test_context("app123", "token123");
        store.put_session(ctx.clone()).await;

        let retrieved = store.get_session(&ctx.app_id).await;
        assert!(retrieved.is_some());
    }

    #[tokio::test]
    async fn test_router_with_channel_and_store() {
        let store = Arc::new(InMemorySessionStore::default());
        let ctx = create_test_context("app123", "token123");
        store.put_session(ctx).await;

        let opts = WebhookBuilderOptions { queue_size: 100 };
        let (_router, _rx) = router_with_channel_and_store(opts, store);
    }

    #[test]
    fn test_router_default() {
        let _router = router::<InMemorySessionStore>();
    }

    // ===== WebhookState Clone test =====
    #[tokio::test]
    async fn test_webhook_state_clone() {
        let store = Arc::new(InMemorySessionStore::default());
        let (tx, _rx) = mpsc::channel(100);
        let state1 = WebhookState {
            store: Arc::clone(&store),
            tx: tx.clone(),
        };
        let state2 = state1.clone();

        // Both states should share the same store
        let ctx = create_test_context("app123", "token123");
        state1.store.put_session(ctx.clone()).await;

        let retrieved = state2.store.get_session(&ctx.app_id).await;
        assert!(retrieved.is_some());
    }

    // ===== Environment variable tests =====
    #[test]
    fn test_dump_dir_not_set() {
        // When GEWE_WEBHOOK_DUMP_DIR is not set, should return None
        std::env::remove_var("GEWE_WEBHOOK_DUMP_DIR");
        // Since OnceLock caches, we can't retest in same process, but we verify the function exists
        let _ = dump_dir();
    }

    #[test]
    fn test_debug_raw_enabled_default() {
        std::env::remove_var("GEWE_WEBHOOK_DEBUG_RAW");
        let _ = debug_raw_enabled();
    }

    #[test]
    fn test_capture_only_default() {
        std::env::remove_var("GEWE_WEBHOOK_CAPTURE_ONLY");
        let _ = capture_only();
    }

    #[test]
    fn test_require_signature_default() {
        std::env::remove_var("GEWE_WEBHOOK_REQUIRE_SIGNATURE");
        let _ = require_signature();
    }

    // ===== Logging functions tests =====
    #[test]
    fn test_log_raw_invalid_body_with_debug() {
        let body = b"invalid json body";
        // Should not panic
        log_raw_invalid_body(body);
    }

    #[test]
    fn test_log_raw_on_verify_fail_with_debug() {
        let body = b"test body";
        // Should not panic
        log_raw_on_verify_fail(body);
    }

    #[test]
    fn test_log_request_pre_parse() {
        let headers = HeaderMap::new();
        let body = b"test body";
        // Should not panic
        log_request_pre_parse(&headers, body);
    }

    #[test]
    fn test_log_headers_on_verify_fail_missing_headers() {
        let headers = HeaderMap::new();
        // Should not panic when headers are missing
        log_headers_on_verify_fail(&headers);
    }

    #[test]
    fn test_log_headers_on_verify_fail_with_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        headers.insert("X-GEWE-TIMESTAMP", "1234567890".parse().unwrap());
        headers.insert("X-GEWE-SIGN", "somesign".parse().unwrap());
        // Should not panic
        log_headers_on_verify_fail(&headers);
    }

    #[test]
    fn test_log_headers_on_verify_fail_partial_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        // Missing timestamp and sign
        log_headers_on_verify_fail(&headers);
    }

    // ===== Integration tests for handle_webhook =====
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_handle_webhook_ping() {
        let (router, _rx) =
            router_with_channel::<InMemorySessionStore>(WebhookBuilderOptions::default());

        let ping_body = r#"{"testMsg":"hello"}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(ping_body))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_handle_webhook_invalid_json() {
        let (router, _rx) =
            router_with_channel::<InMemorySessionStore>(WebhookBuilderOptions::default());

        let invalid_body = "not valid json";
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(invalid_body))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_handle_webhook_unknown_app_id() {
        let (router, _rx) =
            router_with_channel::<InMemorySessionStore>(WebhookBuilderOptions::default());

        let body = r#"{"Appid":"unknown_app","Data":{"test":"data"}}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_handle_webhook_success() {
        let (router, mut rx, store) =
            router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
                queue_size: 10,
            });

        let ctx = create_test_context("app123", "token123");
        store.put_session(ctx).await;

        let body = r#"{"Appid":"app123","Data":{"test":"data"},"TypeName":"message"}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Verify event was sent to channel
        let event = rx.try_recv().ok();
        assert!(event.is_some());
        let event = event.unwrap();
        assert_eq!(event.app_id.0, "app123");
        assert_eq!(event.type_name, Some("message".to_string()));
    }

    #[tokio::test]
    async fn test_handle_webhook_with_new_msg_id() {
        let (router, mut rx, store) =
            router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
                queue_size: 10,
            });

        let ctx = create_test_context("app123", "token123");
        store.put_session(ctx).await;

        // First message with NewMsgId
        let body = r#"{"Appid":"app123","Data":{"NewMsgId":12345}}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(rx.try_recv().is_ok());

        // Duplicate message should be filtered
        let body = r#"{"Appid":"app123","Data":{"NewMsgId":12345}}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        // Channel should be empty (message filtered)
        assert!(rx.try_recv().is_err());
    }

    #[tokio::test]
    async fn test_handle_webhook_nested_new_msg_id() {
        let (router, mut rx, store) =
            router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
                queue_size: 10,
            });

        let ctx = create_test_context("app123", "token123");
        store.put_session(ctx).await;

        // Message with nested NewMsgId in Data.Data
        let body = r#"{"Appid":"app123","Data":{"Data":{"NewMsgId":67890}}}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert!(rx.try_recv().is_ok());
    }

    #[tokio::test]
    async fn test_handle_webhook_queue_full() {
        // Very small queue to test full scenario
        let (router, _rx, store) =
            router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
                queue_size: 1,
            });

        let ctx = create_test_context("app123", "token123");
        store.put_session(ctx).await;

        // Fill the queue
        let body = r#"{"Appid":"app123","Data":{"test":"data1"}}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();
        let response = router.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Second message should overflow (but still return OK)
        let body = r#"{"Appid":"app123","Data":{"test":"data2"}}"#;
        let request = Request::builder()
            .uri("/webhook")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();
        let response = router.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        // Note: _rx is not consumed, so queue should be full
    }

    // ===== Edge cases for extract_new_msg_id =====
    #[test]
    fn test_extract_new_msg_id_wrong_type() {
        // NewMsgId is a string, not a number
        let data = serde_json::json!({"NewMsgId": "not_a_number"});
        assert_eq!(extract_new_msg_id(&data), None);
    }

    #[test]
    fn test_extract_new_msg_id_nested_wrong_type() {
        let data = serde_json::json!({"Data": {"NewMsgId": "not_a_number"}});
        assert_eq!(extract_new_msg_id(&data), None);
    }

    #[test]
    fn test_extract_new_msg_id_empty_data() {
        let data = serde_json::json!({});
        assert_eq!(extract_new_msg_id(&data), None);
    }

    #[test]
    fn test_extract_new_msg_id_data_not_object() {
        let data = serde_json::json!({"Data": "not_an_object"});
        assert_eq!(extract_new_msg_id(&data), None);
    }

    // ===== Edge cases for is_ping =====
    #[test]
    fn test_is_ping_empty_body() {
        let body = b"";
        assert!(!is_ping(body));
    }

    #[test]
    fn test_is_ping_body_with_t_but_no_testmsg() {
        let body = br#"{"otherField":"test"}"#;
        assert!(!is_ping(body));
    }

    #[test]
    fn test_is_ping_testmsg_null() {
        let body = br#"{"testMsg":null}"#;
        // Should still be considered ping if testMsg key exists
        assert!(is_ping(body));
    }

    // ===== Additional verify_signature edge cases =====
    #[test]
    fn test_verify_signature_missing_token() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now.parse().unwrap());
        headers.insert("X-GEWE-SIGN", "somesign".parse().unwrap());
        // Missing X-GEWE-TOKEN

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::MissingHeader)));
    }

    #[test]
    fn test_verify_signature_future_timestamp() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Timestamp 400 seconds in the future (beyond MAX_SKEW of 300)
        let future_ts = (now + 400).to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", future_ts.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        headers.insert("X-GEWE-SIGN", "somesign".parse().unwrap());

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::Stale)));
    }

    #[test]
    fn test_verify_signature_within_max_skew() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Timestamp 200 seconds in the past (within MAX_SKEW of 300)
        let ts = now - 200;
        let ts_str = ts.to_string();
        let body = b"test body";
        let token = "token123";

        // Calculate correct signature
        let mut mac = Hmac::<Sha256>::new_from_slice(token.as_bytes()).unwrap();
        mac.update(ts_str.as_bytes());
        mac.update(b":");
        mac.update(body);
        let signature = hex::encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", ts_str.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", token.parse().unwrap());
        headers.insert("X-GEWE-SIGN", signature.parse().unwrap());

        let ctx = create_test_context("app123", token);

        let result = verify_signature(&headers, &ctx, body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_signature_non_utf8_header_values() {
        use axum::http::HeaderValue;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", now.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        // Insert a header value that is valid HTTP but we'll test error paths
        headers.insert(
            "X-GEWE-SIGN",
            HeaderValue::from_bytes(&[0xFF, 0xFE]).unwrap(),
        );

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        // Should fail when trying to convert to str
        assert!(matches!(result, Err(SignatureError::VerifyFailed)));
    }

    // ===== Test WebhookBody edge cases =====
    #[test]
    fn test_webhook_body_deserialize_minimal() {
        // Minimal valid webhook body
        let json = r#"{"Appid":"app123","Data":{}}"#;
        let body: WebhookBody = serde_json::from_str(json).unwrap();
        assert_eq!(body.appid, "app123");
        assert_eq!(body.type_name, None);
        assert!(body.data.is_object());
    }

    #[test]
    fn test_webhook_body_deserialize_with_null_typename() {
        let json = r#"{"Appid":"app123","Data":{},"TypeName":null}"#;
        let body: WebhookBody = serde_json::from_str(json).unwrap();
        assert_eq!(body.type_name, None);
    }

    #[test]
    fn test_webhook_body_deserialize_complex_data() {
        let json = r#"{"Appid":"app123","Data":{"nested":{"deep":"value"}},"TypeName":"complex"}"#;
        let body: WebhookBody = serde_json::from_str(json).unwrap();
        assert_eq!(body.appid, "app123");
        assert!(body.data.get("nested").is_some());
    }

    #[test]
    fn test_webhook_body_missing_required_fields() {
        // Missing Appid
        let json = r#"{"Data":{}}"#;
        let result: Result<WebhookBody, _> = serde_json::from_str(json);
        assert!(result.is_err());

        // Missing Data
        let json = r#"{"Appid":"app123"}"#;
        let result: Result<WebhookBody, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    // ===== Test unsafe Send/Sync impls =====
    #[test]
    fn test_webhook_state_is_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<WebhookState<InMemorySessionStore>>();
        assert_sync::<WebhookState<InMemorySessionStore>>();
    }

    // ===== Test all SignatureError variants =====
    #[test]
    fn test_signature_error_all_variants() {
        let errors = vec![
            SignatureError::MissingHeader,
            SignatureError::InvalidTimestamp,
            SignatureError::Stale,
            SignatureError::InvalidHex,
            SignatureError::VerifyFailed,
        ];

        for err in errors {
            let debug_str = format!("{:?}", err);
            assert!(!debug_str.is_empty());
        }
    }

    // ===== Test maybe_dump_raw (error paths) =====
    #[tokio::test]
    async fn test_maybe_dump_raw_no_dump_dir() {
        std::env::remove_var("GEWE_WEBHOOK_DUMP_DIR");
        // Should return early without error
        maybe_dump_raw("app123", b"test body").await;
    }

    // ===== Test empty and whitespace values for environment variables =====
    #[test]
    fn test_dump_dir_empty_string() {
        // Empty string should be treated as not set
        std::env::set_var("GEWE_WEBHOOK_DUMP_DIR", "");
        // OnceLock caches the value, so we just verify function doesn't panic
        let _ = dump_dir();
        std::env::remove_var("GEWE_WEBHOOK_DUMP_DIR");
    }

    #[test]
    fn test_dump_dir_whitespace_only() {
        std::env::set_var("GEWE_WEBHOOK_DUMP_DIR", "   ");
        let _ = dump_dir();
        std::env::remove_var("GEWE_WEBHOOK_DUMP_DIR");
    }

    // ===== Additional tests for maybe_dump_raw error paths =====
    #[tokio::test]
    async fn test_maybe_dump_raw_create_dir_error() {
        // Test with an invalid path that can't be created
        std::env::set_var("GEWE_WEBHOOK_DUMP_DIR", "/invalid/nonexistent/path");
        // Should handle error gracefully
        maybe_dump_raw("app123", b"test body").await;
        std::env::remove_var("GEWE_WEBHOOK_DUMP_DIR");
    }

    // ===== Test timestamp edge cases =====
    #[test]
    fn test_verify_signature_timestamp_exactly_at_max_skew() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Timestamp exactly 300 seconds in the past (at MAX_SKEW boundary)
        let ts = now - 300;
        let ts_str = ts.to_string();
        let body = b"test body";
        let token = "token123";

        // Calculate correct signature
        let mut mac = Hmac::<Sha256>::new_from_slice(token.as_bytes()).unwrap();
        mac.update(ts_str.as_bytes());
        mac.update(b":");
        mac.update(body);
        let signature = hex::encode(mac.finalize().into_bytes());

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", ts_str.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", token.parse().unwrap());
        headers.insert("X-GEWE-SIGN", signature.parse().unwrap());

        let ctx = create_test_context("app123", token);

        let result = verify_signature(&headers, &ctx, body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_signature_timestamp_just_beyond_max_skew() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Timestamp 301 seconds in the past (just beyond MAX_SKEW)
        let ts_str = (now - 301).to_string();

        let mut headers = HeaderMap::new();
        headers.insert("X-GEWE-TIMESTAMP", ts_str.parse().unwrap());
        headers.insert("X-GEWE-TOKEN", "token123".parse().unwrap());
        headers.insert("X-GEWE-SIGN", "somesign".parse().unwrap());

        let ctx = create_test_context("app123", "token123");
        let body = b"test body";

        let result = verify_signature(&headers, &ctx, body);
        assert!(matches!(result, Err(SignatureError::Stale)));
    }
}
