# Gewe Bot 配置管理前端

基于 htmx + Alpine.js + DaisyUI 的配置管理界面

## 快速开始

### 1. 设置环境变量

```bash
export GEWE_BOT_TOKEN_MAIN="your_bot_token"
export GEMINI_API_KEY="your_gemini_api_key"
```

或使用启动脚本（需先编辑脚本填入实际值）：

```bash
./crates/gewe-bot-app/start-dev.sh
```

### 2. 启动服务

```bash
cargo run -p gewe-bot-app -- config/bot-app.v2.toml
```

### 3. 访问界面

浏览器打开：`http://localhost:4399/`

## 功能说明

### Dashboard 概览页
- 配置统计（Bots、AI Profiles、Tools、Rules 数量）
- 配置状态（版本、ETag、草稿状态）
- 备份历史（最近 5 个版本，支持回滚）

### Bots 管理
- 查看所有 Bot 配置
- 添加/编辑 Bot（App ID、Token 环境变量、Base URL、Tags）
- 支持环境变量配置（如 `GEWE_BOT_TOKEN_MAIN`）

### AI Profiles 管理
- 查看所有 AI 配置
- 添加/编辑 Profile（Provider、Model、API Key、System Prompt）
- 关联工具（多选 checkbox）

### 工具管理
- 查看所有工具
- 添加/编辑工具（ID、类型、程序路径、超时时间）
- 配置预回复消息

### 规则管理
- **规则模板**：定义匹配条件和动作（any、equals、contains、regex）
- **规则实例**：绑定模板到具体频道（私聊/群聊）、设置优先级、过滤条件

### Prompts 管理
- 查看所有 Prompt 文件
- 点击文件名编辑内容
- 新建/删除 Prompt 文件

### 规则模拟器
- 输入模拟参数（App ID、消息类型、频道、内容）
- 查看匹配的规则列表
- 显示最终执行动作

## 技术栈

- **htmx 2.0.4**：无刷新页面交互
- **Alpine.js 3.x**：响应式状态管理
- **DaisyUI 5.x**：UI 组件库
- **Tailwind CSS**：CSS 框架
- **axum-htmx**：Rust 后端 htmx 集成

## API 端点

### HTML 片段端点（用于 htmx）
- `GET /pages/dashboard` - Dashboard 页面
- `GET /pages/bots` - Bots 列表
- `POST /pages/bots/save` - 保存 Bot
- `GET /pages/ai-profiles` - AI Profiles 列表
- `POST /pages/ai-profiles/save` - 保存 Profile
- ... 以及其他页面端点

### JSON API 端点（用于数据操作）
- `GET /api/config` - 获取配置
- `POST /api/config/save` - 保存配置
- `POST /api/config/publish` - 发布配置
- `POST /api/config/rollback` - 回滚配置
- `POST /api/config/simulate` - 模拟匹配
- `GET /api/prompts` - 列出 Prompts
- `PUT /api/prompts/{name}` - 更新 Prompt

## 目录结构

```
crates/gewe-bot-app/
├── static/
│   └── index.html          # 主页面（通过 CDN 加载前端库）
├── src/
│   ├── api/
│   │   ├── mod.rs          # 路由定义
│   │   ├── pages.rs        # HTML 渲染端点
│   │   ├── config.rs       # JSON API（Phase 1）
│   │   ├── prompts.rs      # Prompts API（Phase 1）
│   │   └── state.rs        # 共享状态
│   ├── config.rs           # 配置结构定义
│   ├── main.rs             # 主程序入口
│   └── ...
└── start-dev.sh            # 开发启动脚本
```

## 开发说明

### 修改 HTML 模板

编辑 `src/api/pages.rs`，修改相应的函数返回的 HTML 字符串。

注意：使用 `r##"..."##` 双井号原始字符串，避免 CSS 选择器 `#` 与字符串结束符冲突。

### 修改样式

编辑 `static/index.html` 的 `<style>` 标签，或直接使用 Tailwind 类名。

### 添加新页面

1. 在 `pages.rs` 添加处理函数
2. 在 `mod.rs` 的 `pages_router()` 添加路由
3. 在 `index.html` 导航栏添加链接

## 故障排查

### 服务无法启动
- 检查环境变量是否设置（`GEWE_BOT_TOKEN_MAIN`、`GEMINI_API_KEY`）
- 检查配置文件路径是否正确
- 查看错误日志

### 页面无法加载
- 检查浏览器控制台是否有 JavaScript 错误
- 检查 Network 面板查看请求是否成功
- 确认服务在 4399 端口正常运行

### 模态框无法打开
- 检查 DaisyUI 是否正确加载
- 检查 `openModal()` 函数是否定义

## 相关文档

- Phase 1 完成记录：`docs/tasks/01-backend-api-phase1.md`
- Phase 2 开发计划：`docs/tasks/02-frontend-htmx-plan.md`
- Phase 2 完成记录：`docs/tasks/03-phase2-completion.md`
- 项目指南：`docs/tasks/00-project-guide.md`
