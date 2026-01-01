# Gewe Bot 配置管理系统 - 完整使用指南

## 快速开始

### 1. 文件模式（推荐开发环境）

```bash
# 编辑启动脚本
nano crates/gewe-bot-app/start-dev.sh

# 设置环境变量
export GEWE_BOT_TOKEN_MAIN="your_bot_token"
export GEMINI_API_KEY="your_gemini_key"

# 启动服务
chmod +x crates/gewe-bot-app/start-dev.sh
./crates/gewe-bot-app/start-dev.sh
```

访问：`http://localhost:4399/`

### 2. Postgres 模式（推荐生产环境）

```bash
# 准备 Postgres 数据库
createdb gewebot

# 设置环境变量
export POSTGRES_URL="postgresql://user:password@localhost:5432/gewebot"
export GEWE_BOT_TOKEN_MAIN="your_bot_token"
export GEMINI_API_KEY="your_gemini_key"

# 可选：启用 API 鉴权
export GEWE_API_TOKEN="admin_secret_token"

# 启动服务（首次会自动运行迁移）
cargo run -p gewe-bot-app -- config/bot-app.v2.toml
```

---

## 功能使用

### Dashboard 概览
- 查看配置统计（Bots、Profiles、Tools、Rules 数量）
- 查看配置状态（版本、ETag、草稿状态）
- 查看备份历史，一键回滚
- 导出/导入配置文件

### Bot 管理
1. 点击"Bots"导航
2. 点击"添加 Bot"
3. 填写表单：
   - ID（可选，默认使用 App ID）
   - App ID（必填）
   - Base URL（默认 https://www.geweapi.com）
   - Token 环境变量名（如 `GEWE_BOT_TOKEN_XXX`）
   - Webhook Secret 环境变量名
   - Tags（逗号分隔）
4. 保存后自动刷新列表

### AI Profile 管理
1. 点击"AI"导航
2. 点击"添加 Profile"
3. 填写：
   - ID（唯一标识）
   - Provider（OpenAI/Gemini/Anthropic/DeepSeek）
   - Model（如 gemini-2.5-flash）
   - Base URL（可选，用于代理）
   - API Key 环境变量名
   - System Prompt 文件路径
   - 勾选需要的工具
4. 保存

### 规则配置
1. 点击"规则"导航
2. **规则模板**标签：
   - 定义匹配条件和动作
   - 不绑定具体频道
3. **规则实例**标签：
   - 选择模板
   - 设置频道（private/group/both）
   - 设置优先级（数字越小越优先）
   - 可选：过滤发送者 wxid
   - 可选：覆盖 AI Profile 或 require_mention

### 模拟器测试
1. 点击"模拟器"导航
2. 填写模拟参数：
   - App ID
   - 消息类型（text/image/voice 等）
   - 频道（private/group）
   - 消息内容
   - 可选：发送者 wxid、是否 @机器人
3. 点击"模拟匹配"
4. 查看匹配结果（哪些规则被命中，最终执行什么动作）

### 全局设置
1. 点击"设置"导航
2. 配置：
   - 服务器（监听地址、队列容量）
   - 存储（图片目录、URL 前缀）
   - 默认行为（回复模式、日志）
   - AI 默认配置（默认 Profile、是否需要 @）
3. 保存后重启服务生效

---

## API 鉴权

### Token 鉴权

```bash
# 设置环境变量
export GEWE_API_TOKEN="my_secret_token"

# 请求示例
curl -H "Authorization: Bearer my_secret_token" \
  http://localhost:4399/api/config
```

### Basic Auth

```bash
# 设置环境变量
export GEWE_API_USERNAME="admin"
export GEWE_API_PASSWORD="password"

# 请求示例
curl -u admin:password \
  http://localhost:4399/api/config
```

### 说明
- 未设置环境变量时，鉴权自动禁用
- 鉴权仅对 `/api/*` 生效
- `/api/healthz` 无需鉴权
- `/pages/*` 和静态文件无需鉴权

---

## 导入/导出配置

### 导出
1. Dashboard 页面点击"导出配置"按钮
2. 浏览器自动下载 `bot-app.v2.toml`

或使用 API：
```bash
curl -O http://localhost:4399/api/config/export
```

### 导入
1. Dashboard 页面点击"导入配置"按钮
2. 选择本地 `.toml` 文件
3. 自动校验并保存

或使用 API：
```bash
curl -X POST http://localhost:4399/api/config/import \
  -H "Content-Type: text/plain" \
  --data-binary @config.toml
```

---

## 数据库迁移

### 文件 -> Postgres 迁移

```bash
# 1. 导出当前配置
curl -O http://localhost:4399/api/config/export

# 2. 设置 Postgres URL
export POSTGRES_URL="postgresql://..."

# 3. 重启服务（自动运行迁移）
cargo run -p gewe-bot-app -- config/bot-app.v2.toml

# 4. 导入配置
curl -X POST http://localhost:4399/api/config/import \
  -H "Content-Type: text/plain" \
  --data-binary @bot-app.v2.toml

# 5. 发布版本
curl -X POST http://localhost:4399/api/config/publish
```

### Postgres -> 文件迁移

```bash
# 1. 导出配置
curl -O http://localhost:4399/api/config/export

# 2. 移除 POSTGRES_URL
unset POSTGRES_URL

# 3. 重启服务
cargo run -p gewe-bot-app -- bot-app.v2.toml
```

---

## 健康检查

```bash
curl http://localhost:4399/api/healthz

# 响应
{
  "status": "ok",
  "timestamp": "2024-12-04T12:00:00Z"
}
```

可用于：
- Kubernetes liveness/readiness probe
- 负载均衡器健康检查
- 监控系统集成

---

## 生产部署建议

### 1. 使用 Postgres 存储
- 更可靠的版本管理
- 支持多实例部署（共享数据库）
- 便于备份和恢复

### 2. 启用 API 鉴权
```bash
export GEWE_API_TOKEN="$(openssl rand -hex 32)"
```

### 3. 配置日志输出
```bash
export GEWE_LOG_JSON=1
export GEWE_LOG_FILE=/var/log/gewe-bot-app/app.log
export GEWE_LOG_ROLLING=daily
```

### 4. 反向代理（Nginx）
```nginx
location /api {
    proxy_pass http://localhost:4399;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
}

location / {
    proxy_pass http://localhost:4399;
}
```

### 5. 使用 systemd 服务
```ini
[Unit]
Description=Gewe Bot Application
After=network.target postgresql.service

[Service]
Type=simple
User=gewebot
WorkingDirectory=/opt/gewe-bot
Environment="POSTGRES_URL=postgresql://..."
Environment="GEWE_API_TOKEN=..."
Environment="GEWE_LOG_JSON=1"
ExecStart=/opt/gewe-bot/gewe-bot-app config/bot-app.v2.toml
Restart=always

[Install]
WantedBy=multi-user.target
```

---

## 故障排查

### 服务无法启动
- 检查环境变量是否设置
- 检查端口 4399 是否被占用
- 检查配置文件路径
- 查看错误日志

### Postgres 连接失败
- 检查 `POSTGRES_URL` 格式
- 确认数据库已创建
- 确认用户权限
- 检查网络连接

### 前端页面空白
- 检查浏览器控制台错误
- 检查 Network 面板请求
- 确认服务正常运行
- 尝试清除浏览器缓存

### 鉴权失败
- 确认环境变量已设置
- 检查 Token 格式（Bearer xxx）
- 检查 Basic Auth 编码
- 查看服务日志

---

## 相关文档

- 项目指南：`docs/tasks/00-project-guide.md`
- 阶段 1 完成：`docs/tasks/01-backend-api-phase1.md`
- 阶段 2 计划：`docs/tasks/02-frontend-htmx-plan.md`
- 阶段 2 完成：`docs/tasks/03-phase2-completion.md`
- 全部完成：`docs/tasks/04-all-phases-completion.md`
- README：`crates/gewe-bot-app/README.md`

---

## 技术支持

遇到问题？
1. 查看日志输出
2. 检查配置文件格式
3. 使用 `/api/config/lint` 校验配置
4. 使用模拟器测试规则匹配
5. 参考示例配置：`docs/config/bot-config-v2.example.toml`
