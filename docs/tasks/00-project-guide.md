# 项目指南：gewe-rs 配置管理系统

> 本文档面向后续参与开发的 AI 助手，帮助快速理解项目结构和关键代码位置。

## 项目概述

**gewe-rs** 是一个基于 Gewe API 的微信机器人框架，使用 Rust 编写。当前正在开发**配置前端/后端系统**，目标是提供一个 Web 界面来管理机器人配置。

### 核心目标
1. 提供前端配置台，直观管理 bot、AI 配置、工具和规则
2. 兼容 V2 文件模式（TOML），后续预留 Postgres 支持
3. 支持配置校验、草稿/发布、版本回滚、模拟命中

## Workspace 结构

```
gewe-rs/
├── crates/
│   ├── gewe-core/       # 核心 API 类型定义
│   ├── gewe-http/       # HTTP 客户端封装
│   ├── gewe-webhook/    # Webhook 接收处理
│   ├── gewe-session/    # 会话管理
│   ├── gewe-bot-app/    # ★ 机器人应用主体（重点关注）
│   ├── gewe-cli/        # 命令行工具
│   ├── gewe-tauri/      # 桌面应用（暂未使用）
│   └── gewe-grpc/       # gRPC 接口（预留）
├── config/              # 配置文件目录
│   ├── bot-app.v2.toml  # V2 配置文件
│   └── prompts/         # Prompt 文件目录
├── docs/
│   ├── design/          # 设计文档
│   └── tasks/           # 任务文档（本目录）
└── Cargo.toml           # Workspace 根配置
```

## 关键代码位置

### 1. 配置系统 (`gewe-bot-app/src/config.rs`)

这是配置系统的核心文件，包含：

**V2 配置结构定义（行 305-516）**
```rust
pub struct AppConfigV2          // 根配置
pub struct ServerConfigV2       // 服务器配置
pub struct StorageConfigV2      // 存储配置
pub struct DefaultsV2           // 默认配置
pub struct BotConfigV2          // Bot 配置
pub struct AiProfileV2          // AI Profile 配置
pub struct ToolConfigV2         // 工具配置
pub struct RuleTemplateV2       // 规则模板
pub struct RuleInstanceV2       // 规则实例
```

**配置方法（行 518-680）**
- `AppConfigV2::parse()` - 从 TOML 解析
- `AppConfigV2::to_toml()` - 序列化为 TOML
- `AppConfigV2::validate()` - 配置校验，返回错误列表

**V2 → V1 转换（行 682-800）**
- `into_v1()` - 将 V2 配置转换为运行时 V1 结构

### 2. API 模块 (`gewe-bot-app/src/api/`)

**模块入口 (`mod.rs`)**
- `api_router()` - 创建所有 API 路由

**状态管理 (`state.rs`)**
- `ApiState` - 共享状态，包含配置路径、元信息
- `ConfigMeta` - 配置元信息（版本、ETag、备份列表）
- `compute_etag()` - 计算内容哈希

**配置 API (`config.rs`)**
- `get_config` - GET /api/config
- `lint_config` - POST /api/config/lint
- `get_meta` - GET /api/config/meta
- `save_config` - POST /api/config/save
- `publish_config` - POST /api/config/publish
- `rollback_config` - POST /api/config/rollback
- `simulate_config` - POST /api/config/simulate

**Prompts API (`prompts.rs`)**
- `list_prompts` - GET /api/prompts
- `get_prompt` - GET /api/prompts/{name}
- `put_prompt` - PUT /api/prompts/{name}

### 3. 主程序入口 (`gewe-bot-app/src/main.rs`)

**关键部分（行 30-74）**
- API 状态初始化
- 路由合并：webhook + API + 静态文件

### 4. 示例配置文件

**V2 配置示例 (`docs/config/bot-config-v2.example.toml`)**
- 完整的 V2 配置格式参考

## API 端点汇总

| 端点 | 方法 | 功能 | 实现位置 |
|------|------|------|----------|
| `/api/config` | GET | 获取配置 JSON | `api/config.rs:54` |
| `/api/config/lint` | POST | 校验配置 | `api/config.rs:88` |
| `/api/config/meta` | GET | 获取元信息 | `api/config.rs:107` |
| `/api/config/save` | POST | 保存草稿 | `api/config.rs:128` |
| `/api/config/publish` | POST | 发布版本 | `api/config.rs:199` |
| `/api/config/rollback` | POST | 回滚版本 | `api/config.rs:260` |
| `/api/config/simulate` | POST | 模拟匹配 | `api/config.rs:300` |
| `/api/prompts` | GET | 列出 prompts | `api/prompts.rs:85` |
| `/api/prompts/{name}` | GET | 获取 prompt | `api/prompts.rs:120` |
| `/api/prompts/{name}` | PUT | 写入 prompt | `api/prompts.rs:157` |

## 配置结构关系

```
AppConfigV2
├── server: ServerConfigV2
├── storage: StorageConfigV2
├── defaults: DefaultsV2
│   └── ai: DefaultsAiV2
├── bots: Vec<BotConfigV2>
├── ai_profiles: Vec<AiProfileV2>
│   └── tool_ids → 引用 tools[].id
├── tools: Vec<ToolConfigV2>
├── rule_templates: Vec<RuleTemplateV2>
│   └── action.ai_profile → 引用 ai_profiles[].id
└── rule_instances: Vec<RuleInstanceV2>
    ├── template → 引用 rule_templates[].id
    └── overrides.ai_profile → 引用 ai_profiles[].id
```

## 开发建议

### 读代码顺序
1. 先看 `docs/config/bot-config-v2.example.toml` 理解配置结构
2. 再看 `config.rs` 中的 V2 结构定义（约 200 行）
3. 然后看 `api/` 目录了解 API 实现
4. 最后看 `main.rs` 了解如何集成

### 避免读取的文件
- `dispatcher.rs` - 业务逻辑，与配置管理无关（2000+ 行）
- `tools/*.rs` - 内置工具实现，与配置管理无关
- `gewe-core/`, `gewe-http/` 等其他 crate - 底层 API，无需关注

### 测试命令
```bash
# 编译检查
cargo check -p gewe-bot-app

# 运行测试
cargo test -p gewe-bot-app

# 运行服务（需要配置文件）
cargo run -p gewe-bot-app -- config/bot-app.v2.toml
```

### 测试 API
```bash
# 获取配置
curl http://localhost:4399/api/config

# 校验配置
curl -X POST http://localhost:4399/api/config/lint \
  -H "Content-Type: application/json" \
  -d '{"config": {"config_version": 2, "bots": []}}'

# 获取元信息
curl http://localhost:4399/api/config/meta
```

## 相关文档

- `docs/design/frontend-config-console.md` - 整体设计文档
- `docs/tasks/frontend-config-console.md` - 任务分解
- `docs/tasks/01-backend-api-phase1.md` - 阶段 1 完成记录
- `docs/tasks/02-frontend-htmx-plan.md` - 阶段 2 开发计划
