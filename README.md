# gewe-rs

基于 Gewe API 的微信自动化 Rust 生态系统

[中文](#介绍) | [English](#introduction)

---

# 介绍

**gewe-rs** 是一个完整的微信自动化生态系统，基于 [Gewe](https://www.geweapi.com/) API 构建。提供从底层 SDK 到上层应用的全套解决方案。

## 组件

```
┌─────────────────────────────────────────────────────┐
│                    gewe-rs                          │
├─────────────────────────────────────────────────────┤
│  应用层                                              │
│  ├─ gewe-cli       命令行工具 (终端操作微信)          │
│  ├─ gewe-bot-app   机器人框架 (自动化/AI对话)         │
│  ├─ gewe-tauri     桌面管理面板 (GUI) [占位]         │
│  └─ gewe-web       Web 管理面板 [占位]               │
├─────────────────────────────────────────────────────┤
│  SDK 层                                             │
│  ├─ gewe-http      HTTP 客户端 (API 封装)            │
│  ├─ gewe-webhook   Webhook 处理 (消息接收)           │
│  ├─ gewe-session   会话管理 (状态存储)               │
│  └─ gewe-grpc      gRPC 支持 [占位]                 │
├─────────────────────────────────────────────────────┤
│  核心层                                              │
│  └─ gewe-core      核心类型定义                      │
└─────────────────────────────────────────────────────┘
```

### CLI 命令行工具

通过终端直接操作微信，支持 50+ 命令：

- 消息：发送/转发/撤回 文字、图片、视频、文件、语音、链接等
- 联系人：添加、删除、备注、黑名单、标签管理
- 群组：创建、邀请、踢人、公告、解散
- 朋友圈：发布、点赞、评论、隐私设置
- 登录：二维码登录、设备切换

### Bot 机器人框架

完整的机器人服务框架，支持：

- 规则引擎：灵活的消息匹配和处理规则
- AI 对话：集成多种 AI 模型 (Gemini, Claude 等)
- Webhook：实时接收微信消息
- 配置管理：热更新配置

### Rust SDK

为 Rust 开发者提供类型安全的 API 封装：

```rust
use gewe_http::GeweClient;

let client = GeweClient::new("http://api.example.com", "your-token");
let contacts = client.contact().get_contact_list().await?;
```

## 安装

### CLI 工具

```bash
# 从 crates.io 安装
cargo install gewe-cli

# 从源码构建
git clone https://github.com/wangnov/gewe-rs.git
cd gewe-rs
cargo build --release -p gewe-cli
```

### SDK

```toml
# Cargo.toml
[dependencies]
gewe-core = "0.1"
gewe-http = "0.1"
```

## 快速开始

### CLI

```bash
# 配置 API 地址和 Token
gewe-cli config set api-url http://your-gewe-api.com
gewe-cli config set token your-token

# 获取登录二维码
gewe-cli login qrcode

# 发送消息
gewe-cli message send-text --to wxid_xxx --content "Hello!"

# 查看帮助
gewe-cli --help
```

### SDK

```rust
use gewe_http::GeweClient;
use gewe_core::message::TextMessage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = GeweClient::new("http://api.example.com", "token");
    
    // 发送文字消息
    client.message()
        .send_text("wxid_xxx", "Hello from Rust!")
        .await?;
    
    Ok(())
}
```

## 功能特性

| 功能 | CLI | SDK | Bot |
|------|-----|-----|-----|
| 发送消息 | ✅ | ✅ | ✅ |
| 接收消息 | ❌ | ✅ | ✅ |
| 联系人管理 | ✅ | ✅ | ✅ |
| 群组管理 | ✅ | ✅ | ✅ |
| 朋友圈 | ✅ | ✅ | ⚠️ |
| AI 对话 | ❌ | ❌ | ✅ |
| 规则引擎 | ❌ | ❌ | ✅ |

## 许可证

本项目采用 [AGPL-3.0](LICENSE) 许可证。

---

# Introduction

**gewe-rs** is a complete WeChat automation ecosystem built on the [Gewe](https://www.geweapi.com/) API. It provides a full suite of solutions from low-level SDK to high-level applications.

## Components

```
┌─────────────────────────────────────────────────────┐
│                    gewe-rs                          │
├─────────────────────────────────────────────────────┤
│  Application Layer                                  │
│  ├─ gewe-cli       CLI tool                        │
│  ├─ gewe-bot-app   Bot framework (AI chat)         │
│  ├─ gewe-tauri     Desktop panel [placeholder]     │
│  └─ gewe-web       Web panel [placeholder]         │
├─────────────────────────────────────────────────────┤
│  SDK Layer                                          │
│  ├─ gewe-http      HTTP client                     │
│  ├─ gewe-webhook   Webhook handler                 │
│  ├─ gewe-session   Session management              │
│  └─ gewe-grpc      gRPC support [placeholder]      │
├─────────────────────────────────────────────────────┤
│  Core Layer                                         │
│  └─ gewe-core      Core types                      │
└─────────────────────────────────────────────────────┘
```

### CLI Tool

Operate WeChat directly from terminal with 50+ commands:

- Messages: send/forward/revoke text, images, videos, files, voice, etc.
- Contacts: add, delete, remark, blacklist, tag management
- Groups: create, invite, kick, announcement, dissolve
- Moments: post, like, comment, privacy settings
- Login: QR code login, device switching

### Bot Framework

Complete bot service framework supporting:

- Rule Engine: flexible message matching and processing
- AI Chat: integration with multiple AI models (Gemini, Claude, etc.)
- Webhook: real-time WeChat message reception
- Config Management: hot-reload configuration

### Rust SDK

Type-safe API wrapper for Rust developers:

```rust
use gewe_http::GeweClient;

let client = GeweClient::new("http://api.example.com", "your-token");
let contacts = client.contact().get_contact_list().await?;
```

## Installation

### CLI Tool

```bash
# Install from crates.io
cargo install gewe-cli

# Build from source
git clone https://github.com/wangnov/gewe-rs.git
cd gewe-rs
cargo build --release -p gewe-cli
```

### SDK

```toml
# Cargo.toml
[dependencies]
gewe-core = "0.1"
gewe-http = "0.1"
```

## Quick Start

### CLI

```bash
# Configure API URL and Token
gewe-cli config set api-url http://your-gewe-api.com
gewe-cli config set token your-token

# Get login QR code
gewe-cli login qrcode

# Send message
gewe-cli message send-text --to wxid_xxx --content "Hello!"

# View help
gewe-cli --help
```

### SDK

```rust
use gewe_http::GeweClient;
use gewe_core::message::TextMessage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = GeweClient::new("http://api.example.com", "token");
    
    // Send text message
    client.message()
        .send_text("wxid_xxx", "Hello from Rust!")
        .await?;
    
    Ok(())
}
```

## Features

| Feature | CLI | SDK | Bot |
|---------|-----|-----|-----|
| Send Messages | ✅ | ✅ | ✅ |
| Receive Messages | ❌ | ✅ | ✅ |
| Contact Management | ✅ | ✅ | ✅ |
| Group Management | ✅ | ✅ | ✅ |
| Moments | ✅ | ✅ | ⚠️ |
| AI Chat | ❌ | ❌ | ✅ |
| Rule Engine | ❌ | ❌ | ✅ |

## License

This project is licensed under [AGPL-3.0](LICENSE).
