# CLAsh「测试」自动回复示例

本示例演示如何用新建的 `gewe-bot-app` 自动处理 webhook 回调，并在某个昵称发送「测试」时回复「测试成功」。

## 准备配置

1) 复制示例配置：
```bash
cp config/bot-app.example.toml config/bot-app.toml
```
2) 编辑 `config/bot-app.toml`：
- `app_id`、`token`：来自 geweapi 后台。
- `base_url`：你的公网域名（只写协议+域名，SDK 会自动拼接 `/gewe/v2/api/...`）。
- `webhook_secret`：如后台设置了签名密钥则填写，否则可删掉/注释。
- `bots.rules`：示例包含多条规则，可按需增删：
  - `kind`：消息场景，支持 `text/image/voice/video/emoji/link/file_notice/contact_event/any`。
  - `from`：来源过滤（如 `nick`、`wxid`），可选。
  - `match`：文本匹配（`equals`/`contains`/`regex`），可选。
  - `chat`：`private`/`group`，可选。
  - `action`：`reply_text`、`save`（`dir`+`filename`）、`forward`（wxid 列表）、`log`、`ignore`、`command`（`program` 必填，`args`/`timeout_secs`/`max_output` 可选；内置命令直接可用，外置命令需 `GEWE_ALLOW_COMMAND=1` 以降低误触风险）。

## 启动 webhook 服务

```bash
cargo run -p gewe-bot-app -- config/bot-app.toml
```

- 默认监听 `0.0.0.0:3000`，可在配置修改。
- 启动后将本机端口通过 frp 暴露到公网，把公网地址填到 geweapi 回调地址。

## 工作流程

Webhook 进来后会做：去重 → 入队 → 规则匹配（按 kind/from/chat/match）→ 执行动作（如 reply/save/forward/log）。新的规则只需在配置的 `bots.rules` 增加对应条件即可，无需改代码。
