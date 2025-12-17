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
