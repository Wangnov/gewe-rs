# Webhook 集成示例

以下代码片段演示如何通过 `router_with_channel_and_state` 创建 webhook 路由，并在启动阶段将机器人上下文写入 SessionStore，使签名校验与消息去重能立即生效。

```rust
use axum::Router;
use gewe_core::{AppId, BotContext};
use gewe_session::InMemorySessionStore;
use gewe_webhook::{router_with_channel_and_state, WebhookBuilderOptions};

#[tokio::main]
async fn main() {
    let (router, mut rx, store) =
        router_with_channel_and_state::<InMemorySessionStore>(WebhookBuilderOptions {
            queue_size: 2048,
        });

    // 在启动阶段或登录成功后注册 BotContext，供签名校验/去重逻辑使用。
    store
        .put_session(BotContext {
            app_id: AppId("appid_xxx".into()),
            token: "bot-token".into(),
            webhook_secret: Some("secret-if-any".into()),
            description: Some("demo bot".into()),
        })
        .await;

    // 业务线程消费 webhook 事件
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            println!("received webhook: {:?}", event.type_name);
        }
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
```

> 说明：当前上游回调未携带签名头，库默认不验签。如需在有签名头的环境启用验签，可设置环境变量 `GEWE_WEBHOOK_REQUIRE_SIGNATURE=1`。

如果需要复用自定义的 `SessionStore`（例如 SQLite/Redis 版本），可以手动构造 `Arc<S>` 后传给 `router_with_channel_and_store`，本示例的思路同样适用：保留 `Arc<S>` 句柄并在登录后调用 `put_session` 即可。
