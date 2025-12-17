# 规则配置示例合集

以下示例基于 `config/bot-app.toml` 的 `bots.rules`，按“场景 + 少量条件 + 动作”配置，无需了解底层回调字段。

## 1) 文本：昵称等于 CLAsh，内容等于“测试”→ 回复“测试成功”
```toml
[[bots.rules]]
kind = "text"
from = { nick = "CLAsh" }
match = { equals = "测试" }
action = { reply_text = "测试成功" }
```

## 2) 文本：群聊里内容包含“报名”→ 固定回复
```toml
[[bots.rules]]
kind = "text"
chat = "group"
match = { contains = "报名" }
action = { reply_text = "收到报名，请稍后处理" }
```

## 3) 图片：任何图片 → 落盘到 data/images/{new_msg_id}.jpg
```toml
[[bots.rules]]
kind = "image"
action = { save = { dir = "data/images", filename = "{new_msg_id}.jpg" } }
```

## 4) 链接：收到链接消息 → 只记录，不动作
```toml
[[bots.rules]]
kind = "link"
action = { log = true }
```

## 5) Emoji：收到表情 → 转发原文本到指定 wxid，并记录
```toml
[[bots.rules]]
kind = "emoji"
action = { forward = ["wxid_friend"], log = true }
```

## 6) 文件通知（MsgType=49 type=74）：保存到 data/files/{new_msg_id}.bin
```toml
[[bots.rules]]
kind = "file_notice"
action = { save = { dir = "data/files", filename = "{new_msg_id}.bin" } }
```

## 7) 语音：保存语音原文件
```toml
[[bots.rules]]
kind = "voice"
action = { save = { dir = "data/voices", filename = "{new_msg_id}.amr" } }
```

## 8) 群聊任意文本 → 仅记录
```toml
[[bots.rules]]
kind = "text"
chat = "group"
action = { log = true }
```

## 9) 兜底：任何消息 → 仅记录
```toml
[[bots.rules]]
kind = "any"
action = { log = true }
```

## 10) 文本：关键词“查询” → 调用内置命令获取 Claude 变更日志
```toml
[[bots.rules]]
kind = "text"
match = { equals = "查询" }
action = { command = { program = "claude_changelog", timeout_secs = 10, max_output = 2000 } }
```

提示：
- `kind` 支持 `text/image/voice/video/emoji/link/file_notice/contact_event/any`。
- `from` 可填 `nick` 或 `wxid`，可选。
- `match` 支持 `equals`/`contains`/`regex`，可选。
- `chat` 可选 `private`/`group`。
- `action` 支持 `reply_text`、`save`（`dir`+`filename`，可用占位 `{new_msg_id}/{from_wxid}/{app_id}`）、`forward`（wxid 列表）、`log`、`ignore`、`command`（`program` 必填，`args`/`timeout_secs`/`max_output` 可选；内置 `claude_changelog` 直接可用，外置命令需设置 `GEWE_ALLOW_COMMAND=1`）。
