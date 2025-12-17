# 阶段 2 开发启动提示词

## 任务：实现 gewe-rs 配置管理前端（htmx 方案）

我需要你帮我实现配置管理系统的前端界面。这是一个 Rust 项目，后端 API 已经完成，现在需要用 htmx + Alpine.js + DaisyUI 实现前端。

### 第一步：阅读文档

请按顺序阅读以下文档，了解项目背景和开发计划：

1. `docs/tasks/00-project-guide.md` - 项目指南，了解代码结构和关键文件位置
2. `docs/tasks/01-backend-api-phase1.md` - 已完成的后端 API，了解可用接口
3. `docs/tasks/02-frontend-htmx-plan.md` - **重点阅读**，包含完整的前端开发计划和代码示例

### 第二步：了解现有代码（仅需快速浏览）

- `crates/gewe-bot-app/src/api/mod.rs` - API 路由定义
- `crates/gewe-bot-app/src/main.rs` - 主程序入口（行 30-74 的 API 集成部分）
- `docs/config/bot-config-v2.example.toml` - V2 配置格式参考

**不需要阅读**：`dispatcher.rs`、`tools/*.rs`、其他 crate 的代码

### 第三步：开始实现

按照 `02-frontend-htmx-plan.md` 中的任务清单，从 Phase 2.1 开始：

1. 添加 `axum-htmx` 依赖到 `Cargo.toml`
2. 创建 `static/index.html` 主框架
3. 创建 `src/api/pages.rs` 模块
4. 在 `src/api/mod.rs` 中添加 pages 路由
5. 修改 `main.rs` 集成静态文件服务

### 技术栈

- htmx 2.0.4 - 无刷新页面交互
- Alpine.js 3.x - 轻量级响应式
- DaisyUI 5.x - UI 组件库（基于 Tailwind）
- axum-htmx - Rust htmx 请求头提取器

### 可用的后端 API

| 端点 | 方法 | 功能 |
|------|------|------|
| `/api/config` | GET | 获取配置 JSON |
| `/api/config/lint` | POST | 校验配置 |
| `/api/config/meta` | GET | 获取元信息 |
| `/api/config/save` | POST | 保存草稿 |
| `/api/config/publish` | POST | 发布版本 |
| `/api/config/rollback` | POST | 回滚版本 |
| `/api/config/simulate` | POST | 模拟匹配 |
| `/api/prompts` | GET | 列出 prompts |
| `/api/prompts/{name}` | GET/PUT | 读写 prompt |

### 注意事项

- 所有 CDN 资源使用文档中指定的版本
- 密钥字段只显示环境变量名（`_env` 后缀）
- 保存时使用 ETag 实现乐观锁
- 422 响应需要特殊处理显示校验错误

请先阅读文档，然后告诉我你的理解和实现计划。
