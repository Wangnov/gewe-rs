mod api;
mod config;
mod dispatcher;
mod storage;
mod tools;

use crate::api::{api_router, auth, pages_router, ApiState};
use crate::config::AppConfig;
use crate::dispatcher::Dispatcher;
use axum::{middleware, response::Html, routing::get, Router};
use gewe_core::{AppId, BotContext};
use gewe_session::{InMemorySessionStore, SessionStore};
use gewe_webhook::{router_with_channel_and_state, WebhookBuilderOptions};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tower::make::Shared;
use tower_http::services::ServeDir;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args().nth(1);
    let app_config = AppConfig::load(config_path.as_deref())?;
    init_tracing();

    // 确保图片目录存在
    tokio::fs::create_dir_all(&app_config.image_dir).await?;
    tracing::info!(image_dir = %app_config.image_dir, "图片存储目录已就绪");

    // 初始化 API 状态
    let config_file_path = config_path
        .map(PathBuf::from)
        .or_else(|| std::env::var("GEWE_BOT_CONFIG").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("config/bot-app.v2.toml"));

    let prompts_dir = config_file_path
        .parent()
        .unwrap_or(Path::new("."))
        .join("prompts");

    let backup_dir = config_file_path
        .parent()
        .unwrap_or(Path::new("."))
        .join("backups");

    let api_state = ApiState::new(config_file_path.clone(), prompts_dir, backup_dir);
    if let Err(e) = api_state.initialize().await {
        tracing::warn!(error = ?e, "API 状态初始化失败，部分功能可能不可用");
    }

    let (webhook_router, rx, store) =
        router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
            queue_size: app_config.queue_size,
        });

    for bot in &app_config.bots {
        store
            .put_session(BotContext {
                app_id: AppId(bot.app_id.clone()),
                token: bot.token.clone(),
                webhook_secret: bot.webhook_secret.clone(),
                description: Some("gewe-bot-app bot".to_string()),
            })
            .await;
    }

    // 合并 webhook 路由、API 路由、Pages 路由和静态文件路由
    let image_url_prefix = app_config.image_url_prefix.trim_start_matches('/');

    // 可选的鉴权中间件
    let api_router = if std::env::var("GEWE_API_TOKEN").is_ok() {
        tracing::info!("API Token 鉴权已启用");
        api_router(api_state.clone()).route_layer(middleware::from_fn(auth::auth_middleware))
    } else if std::env::var("GEWE_API_USERNAME").is_ok() {
        tracing::info!("Basic Auth 鉴权已启用");
        api_router(api_state.clone()).route_layer(middleware::from_fn(auth::basic_auth_middleware))
    } else {
        tracing::warn!("API 鉴权未启用，生产环境建议设置 GEWE_API_TOKEN 或 GEWE_API_USERNAME/GEWE_API_PASSWORD");
        api_router(api_state.clone())
    };

    let router: Router = webhook_router
        .route("/", get(index_page))
        .nest("/api", api_router)
        .nest("/pages", pages_router(api_state))
        .nest_service(
            &format!("/{}", image_url_prefix),
            ServeDir::new(&app_config.image_dir),
        );

    let dispatcher = Dispatcher::new(&app_config)?;
    let shared = std::sync::Arc::new(dispatcher);
    let mut event_rx = rx;
    let concurrency = std::sync::Arc::new(tokio::sync::Semaphore::new(
        app_config.max_concurrency.max(1),
    ));
    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            let permit = concurrency.clone().acquire_owned().await;
            let shared = shared.clone();
            tokio::spawn(async move {
                let _permit = permit;
                if let Err(err) = shared.handle(event).await {
                    tracing::warn!(?err, "事件处理失败");
                }
            });
        }
    });

    let listener = tokio::net::TcpListener::bind(&app_config.listen_addr).await?;
    tracing::info!(
        "服务监听: {}, 图片访问路径: /{}, API 路径: /api, 前端路径: /",
        app_config.listen_addr,
        image_url_prefix
    );
    let service = router.into_service::<axum::body::Body>();
    let make_service = Shared::new(service);
    axum::serve(listener, make_service).await?;

    Ok(())
}

/// 返回前端主页面（占位）
async fn index_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Gewe Bot</title>
    <style>
        body { font-family: system-ui, sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background: #f5f5f5; }
        .container { text-align: center; padding: 2rem; }
        h1 { color: #333; }
        p { color: #666; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Gewe Bot</h1>
        <p>前端尚未构建，请先运行 <code>cd frontend && npm install && npm run build</code></p>
        <p>API 服务正常运行中</p>
    </div>
</body>
</html>"#,
    )
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,gewe_bot_app=debug"));

    let use_json = env_flag("GEWE_LOG_JSON");
    let log_file = std::env::var("GEWE_LOG_FILE").ok();
    let rolling = std::env::var("GEWE_LOG_ROLLING").unwrap_or_else(|_| "daily".to_string());

    if let Some(path) = log_file {
        let writer = make_file_writer(&path, &rolling);
        let builder = tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_ansi(false)
            .with_writer(writer);
        if use_json {
            builder.json().flatten_event(true).finish().init();
        } else {
            builder.finish().init();
        }
    } else {
        let builder = tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_ansi(!use_json);
        if use_json {
            builder.json().flatten_event(true).finish().init();
        } else {
            builder.finish().init();
        }
    }
}

fn make_file_writer(path: &str, rolling: &str) -> tracing_appender::non_blocking::NonBlocking {
    let (dir, file_name) = split_path(path);
    let rotation = match rolling {
        "hourly" => tracing_appender::rolling::Rotation::HOURLY,
        "never" => tracing_appender::rolling::Rotation::NEVER,
        _ => tracing_appender::rolling::Rotation::DAILY,
    };
    let appender = tracing_appender::rolling::RollingFileAppender::new(rotation, dir, file_name);
    let (nb, guard) = tracing_appender::non_blocking(appender);
    FILE_GUARD.get_or_init(|| guard);
    nb
}

fn split_path(path: &str) -> (&Path, String) {
    let p = Path::new(path);
    let dir = p.parent().unwrap_or_else(|| Path::new("."));
    let file = p
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "gewe-bot-app.log".to_string());
    (dir, file)
}

fn env_flag(key: &str) -> bool {
    matches!(
        std::env::var(key).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("True")
    )
}

static FILE_GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();
