# wait-reply 命令设计文档

## 概述

`wait-reply` 命令用于发送消息后等待特定用户的回复。支持私聊和群聊场景，支持多种消息类型混合发送。

## 使用场景

1. **自动化脚本**：发送任务完成通知，等待用户确认
2. **交互式 CLI**：发送消息后获取用户回复作为输入
3. **监控告警**：发送告警后等待处理确认

## 命令格式

```bash
gewe-cli wait-reply [OPTIONS] [--message <TYPE:CONTENT>]...
```

## 参数说明

### 必填参数

| 参数 | 说明 | 示例 |
|------|------|------|
| `--to-wxid <WXID>` | 发送目标用户 wxid | `--to-wxid wxid_xxx` |
| `--listen <ADDR>` | webhook 监听地址 | `--listen 0.0.0.0:4399` |

### 可选参数

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `--group-wxid <WXID>` | 群 ID（群消息场景） | 无 |
| `--filter-wxid <WXID>` | 过滤发送者 wxid | 同 `--to-wxid` |
| `--match <REGEX>` | 正则匹配回复内容 | 无（任意回复） |
| `--timeout <SECONDS>` | 超时秒数 | 无（无限等待） |
| `--output-format <FORMAT>` | 输出格式：`text` / `json` | `text` |
| `--message <TYPE:CONTENT>` | 发送的消息（可多次指定） | 无（仅监听） |

### 消息类型格式

```bash
--message "text:消息内容"
--message "image:/path/to/file.png"
--message "voice:/path/to/file.mp3"
--message "video:/path/to/file.mp4"
--message "link:标题|描述|URL|缩略图URL"
```

## 使用示例

### 示例 1：私聊发送消息并等待回复

```bash
gewe-cli wait-reply \
  --to-wxid wxid_mly499mvz23o21 \
  --listen 0.0.0.0:4399 \
  --message "text:任务已完成，请回复确认或取消"
```

### 示例 2：私聊发送多条消息并等待特定回复

```bash
gewe-cli wait-reply \
  --to-wxid wxid_mly499mvz23o21 \
  --listen 0.0.0.0:4399 \
  --match "确认|取消|done" \
  --message "text:【通知】任务执行完成" \
  --message "image:/tmp/result.png" \
  --message "text:请回复「确认」继续或「取消」终止"
```

### 示例 3：群聊场景

```bash
gewe-cli wait-reply \
  --to-wxid wxid_mly499mvz23o21 \
  --group-wxid 12345678@chatroom \
  --listen 0.0.0.0:4399 \
  --filter-wxid wxid_mly499mvz23o21 \
  --match "收到|OK" \
  --message "text:@张三 请确认收到"
```

### 示例 4：仅监听模式（不发送消息）

```bash
gewe-cli wait-reply \
  --to-wxid wxid_mly499mvz23o21 \
  --listen 0.0.0.0:4399 \
  --timeout 60
```

### 示例 5：JSON 格式输出

```bash
gewe-cli wait-reply \
  --to-wxid wxid_mly499mvz23o21 \
  --listen 0.0.0.0:4399 \
  --output-format json \
  --message "text:请回复任意内容"
```

## 执行流程

```
┌─────────────────────────────────────────────────────────────┐
│                      wait-reply 执行流程                      │
└─────────────────────────────────────────────────────────────┘

    ┌──────────────┐
    │   开始执行    │
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐
    │ 启动 webhook │
    │   监听服务    │
    └──────┬───────┘
           │
           ▼
    ┌──────────────┐     否
    │ 有 --message │─────────┐
    │    参数？     │         │
    └──────┬───────┘         │
           │ 是              │
           ▼                 │
    ┌──────────────┐         │
    │  按顺序发送   │         │
    │   所有消息    │         │
    └──────┬───────┘         │
           │                 │
           ▼                 ▼
    ┌─────────────────────────┐
    │      等待消息回复        │
    │  ┌─────────────────┐    │
    │  │ 过滤条件：        │    │
    │  │ 1. filter-wxid  │    │
    │  │ 2. group-wxid   │    │
    │  │ 3. match 正则   │    │
    │  └─────────────────┘    │
    └──────────┬──────────────┘
               │
         ┌─────┴─────┐
         │           │
         ▼           ▼
    ┌────────┐  ┌────────┐
    │ 超时    │  │ 收到匹 │
    │ 退出    │  │ 配消息 │
    └────────┘  └───┬────┘
                    │
                    ▼
             ┌─────────────┐
             │ 输出所有已收 │
             │ 到的消息内容 │
             │ (含匹配消息) │
             └──────┬──────┘
                    │
                    ▼
             ┌─────────────┐
             │   退出程序   │
             └─────────────┘
```

## 私聊 vs 群聊

### 私聊模式

- 不指定 `--group-wxid`
- `--to-wxid` 为目标用户
- `--filter-wxid` 默认等于 `--to-wxid`
- 消息直接发送给用户

### 群聊模式

- 指定 `--group-wxid` 为群 ID（以 `@chatroom` 结尾）
- `--to-wxid` 为要 @ 的用户
- `--filter-wxid` 过滤群内发送者（从消息 content 提取）
- 消息发送到群聊

### 群消息发送者提取逻辑

群消息的 `Content` 字段格式为：

```
sender_wxid:\n
实际消息内容
```

提取逻辑（参考 `gewe-bot-app/src/dispatcher.rs`）：

```rust
fn extract_group_sender(content: &str) -> Option<String> {
    let trimmed = content.trim_start();
    if let Some((head, _)) = trimmed.split_once(':') {
        let sender = head.trim();
        if !sender.is_empty() {
            return Some(sender.to_string());
        }
    }
    None
}
```

## 输出格式

### text 格式（默认）

只输出回复的文本内容，每条消息一行：

```
收到
好的，已确认
```

### json 格式

输出完整的消息 JSON 数组：

```json
[
  {
    "from_wxid": "wxid_xxx",
    "group_wxid": null,
    "content": "收到",
    "timestamp": "2025-01-01T12:00:00Z"
  },
  {
    "from_wxid": "wxid_xxx",
    "group_wxid": null,
    "content": "好的，已确认",
    "timestamp": "2025-01-01T12:00:05Z"
  }
]
```

## 匹配逻辑

当指定 `--match` 参数时：

1. 收到的每条消息都会与正则表达式进行匹配
2. 匹配成功时，输出从第 1 条到当前条的所有消息
3. 匹配失败的消息会被缓存，等待后续匹配
4. 未指定 `--match` 时，收到第一条消息即返回

## 退出码

| 退出码 | 说明 |
|--------|------|
| 0 | 成功收到匹配的回复 |
| 1 | 超时未收到回复 |
| 2 | 发送消息失败 |
| 3 | webhook 启动失败 |

## 多进程共享机制

当多个 `wait-reply` 进程同时运行时，支持共享同一个 webhook 端口接收的消息。

### 场景说明

```
场景：第一个 wait-reply 一直未收到回复，用户启动第二个 wait-reply

T1: wait-reply A 启动 → 监听 :4399，等待用户 X 回复
T2: wait-reply B 启动 → 发现 :4399 被占用 → 连接 A 的广播 socket
T3: 用户 Y 回复 → A 广播消息 → B 收到并匹配成功 → B 退出
T4: 用户 X 回复 → A 收到并匹配成功 → A 退出
```

### 架构设计

```
                    GeWe 平台
                        │
                        ▼ webhook 推送
              ┌─────────────────────┐
              │  wait-reply A (主)   │
              │  - 监听 :4399       │
              │  - 广播 Unix socket │
              └──────────┬──────────┘
                         │ 广播消息
           ┌─────────────┼─────────────┐
           ▼             ▼             ▼
    ┌────────────┐ ┌────────────┐ ┌────────────┐
    │ wait-reply │ │ wait-reply │ │ wait-reply │
    │     A      │ │     B      │ │     C      │
    │ (自己处理)  │ │ (订阅者)   │ │ (订阅者)   │
    │ filter: X  │ │ filter: Y  │ │ filter: Z  │
    └────────────┘ └────────────┘ └────────────┘
```

### 约定

| 项目 | 说明 |
|------|------|
| 广播 socket 路径 | `/tmp/gewe-wait-reply-{port}.sock` |
| 通信协议 | JSON Lines（每条消息一行 JSON） |
| 消息类型 | `message`（普通消息）、`shutdown`（主进程退出） |

### 启动流程

```
┌──────────────┐
│   启动进程    │
└──────┬───────┘
       │
       ▼
┌──────────────────┐
│ 尝试绑定 --listen │
│      端口         │
└──────┬───────────┘
       │
   ┌───┴───┐
   │       │
成功      失败
   │       │
   ▼       ▼
┌──────┐  ┌────────────────────┐
│ 主进程│  │ 检查广播 socket    │
│ 模式  │  │ 是否存在           │
└──┬───┘  └──────┬─────────────┘
   │             │
   │         ┌───┴───┐
   │         │       │
   │       存在    不存在
   │         │       │
   │         ▼       ▼
   │    ┌────────┐ ┌──────────┐
   │    │ 订阅者 │ │ 报错退出  │
   │    │ 模式   │ │(非wait-  │
   │    └────────┘ │ reply占用)│
   │               └──────────┘
   ▼
┌─────────────────┐
│ 创建广播 socket  │
│ 开始接收/广播   │
└─────────────────┘
```

### 主进程职责

1. 绑定 webhook 监听端口
2. 创建广播 Unix socket：`/tmp/gewe-wait-reply-{port}.sock`
3. 接收 webhook 消息
4. 将消息广播给所有订阅者
5. 自己也处理消息（根据 filter 条件）
6. 退出时发送 `shutdown` 消息并清理 socket 文件

### 订阅者职责

1. 连接主进程的广播 socket
2. 从 socket 接收广播消息
3. 根据自己的 filter 条件处理消息
4. 检测主进程断开时尝试升级

### 广播消息格式

普通消息：
```json
{"type": "message", "data": {"from_wxid": "wxid_xxx", "content": "hello", "timestamp": "2025-01-01T12:00:00Z"}}
```

主进程退出：
```json
{"type": "shutdown", "reason": "primary_exit"}
```

## 故障转移机制

当主进程意外退出时，订阅者会自动竞争升级为新的主进程。

### 故障转移流程

```
┌─────────────────────────────────────────────────────────────┐
│                   主进程退出 → 自动升级                       │
└─────────────────────────────────────────────────────────────┘

T1: A(主) ─────┬───── B(订阅) ───── C(订阅)
              │
T2: A 意外退出 ┘
              ↓
T3: B、C 检测到 socket 断开（EOF 或 shutdown 消息）
              ↓
T4: B、C 同时尝试绑定 :4399 端口
              ↓
T5: B 成功绑定 → 升级为主进程，创建新 socket
    C 绑定失败 → 连接 B 的 socket，成为 B 的订阅者
              ↓
T6: 系统恢复正常运行
```

### 订阅者状态机

```rust
enum Role {
    Primary,    // 主进程
    Subscriber, // 订阅者
}

loop {
    match role {
        Role::Primary => {
            // 接收 webhook，广播给订阅者，处理自己的消息
            select! {
                msg = webhook_rx.recv() => broadcast_and_handle(msg),
                _ = shutdown_signal => {
                    broadcast_shutdown();
                    cleanup_socket();
                    break;
                }
            }
        }
        Role::Subscriber => {
            match socket.recv() {
                Ok(BroadcastMsg::Message(msg)) => {
                    handle_message(msg);
                }
                Ok(BroadcastMsg::Shutdown) | Err(Disconnected) => {
                    // 主进程断开，尝试升级
                    match try_bind_port() {
                        Ok(listener) => {
                            role = Role::Primary;
                            create_broadcast_socket();
                        }
                        Err(_) => {
                            // 别人抢先升级了，重新连接
                            sleep(random_backoff());
                            connect_to_socket()?;
                        }
                    }
                }
            }
        }
    }
}
```

### 竞争策略

- 使用端口绑定作为"选举"机制，先成功绑定的进程成为新主
- 失败的进程等待随机退避时间后重新连接 socket
- 避免惊群效应：退避时间 = 基础时间 + 随机抖动

### 边界情况处理

| 场景 | 处理方式 |
|------|----------|
| 主进程正常退出 | 发送 shutdown 消息，订阅者立即竞争升级 |
| 主进程崩溃 | 订阅者检测到 EOF，竞争升级 |
| 所有订阅者同时竞争 | 先绑定成功的成为新主，其他重连 |
| 新主创建 socket 前有进程连接 | 连接失败，重试直到成功 |
| socket 文件残留 | 启动时检测并清理无效的 socket 文件 |

### Socket 文件清理

主进程启动时：
```rust
fn cleanup_stale_socket(path: &Path) -> Result<()> {
    if path.exists() {
        // 尝试连接，如果失败说明是残留文件
        match UnixStream::connect(path) {
            Ok(_) => Err("socket is active, port conflict"),
            Err(_) => {
                fs::remove_file(path)?;
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}
```

## 注意事项

1. **回调地址配置**：使用前需在 GeWe 平台设置回调 URL 指向监听地址
2. **端口共享**：多个 wait-reply 可共享同一端口，自动协调主/订阅角色
3. **网络可达**：GeWe 平台需能访问到监听地址（公网 IP 或内网穿透）
4. **回复类型**：当前仅支持接收文本类型回复
5. **消息顺序**：多条 `--message` 按指定顺序依次发送
6. **故障恢复**：主进程退出后订阅者自动升级，无需人工干预

## 数据结构

### 内部消息结构

```rust
pub struct WaitReplyMessage {
    pub msg_type: MessageType,
    pub content: String,
}

pub enum MessageType {
    Text,
    Image,
    Voice,
    Video,
    Link { title: String, desc: String, url: String, thumb_url: String },
}
```

### 回复消息结构

```rust
pub struct ReceivedReply {
    pub from_wxid: String,
    pub group_wxid: Option<String>,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
```

## 实现要点

1. **复用 gewe-webhook**：使用 `gewe-webhook` crate 处理 webhook 接收
2. **复用消息发送**：使用 `gewe-http` crate 发送各类消息
3. **群消息处理**：参考 `gewe-bot-app` 的 `extract_group_sender` 和 `strip_sender_prefix`
4. **异步处理**：webhook 监听和消息发送并行处理
