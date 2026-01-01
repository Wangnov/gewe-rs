# 快速参考卡

## 启动命令

### 开发模式（文件存储）
```bash
./crates/gewe-bot-app/start-dev.sh
```

### 生产模式（Postgres）
```bash
export POSTGRES_URL="postgresql://user:pass@localhost/gewebot"
export GEWE_API_TOKEN="$(openssl rand -hex 32)"
export GEWE_LOG_JSON=1
export GEWE_LOG_FILE=/var/log/gewe-bot.log

cargo run --release -p gewe-bot-app -- config/bot-app.v2.toml
```

## 环境变量速查

| 变量 | 必需 | 默认值 | 说明 |
|------|------|--------|------|
| `GEWE_BOT_TOKEN_MAIN` | ✅ | - | Bot Token |
| `GEMINI_API_KEY` | ✅ | - | Gemini API Key |
| `POSTGRES_URL` | ❌ | - | Postgres 连接（未设置使用文件） |
| `GEWE_API_TOKEN` | ❌ | - | API Token 鉴权 |
| `GEWE_API_USERNAME` | ❌ | - | Basic Auth 用户名 |
| `GEWE_API_PASSWORD` | ❌ | - | Basic Auth 密码 |
| `GEWE_LOG_JSON` | ❌ | 0 | JSON 日志格式 |
| `GEWE_LOG_FILE` | ❌ | - | 日志文件路径 |
| `GEWE_LOG_ROLLING` | ❌ | daily | 滚动策略（daily/hourly/never） |
| `RUST_LOG` | ❌ | info | 日志级别 |

## API 端点速查

### 配置管理
- `GET /api/config` - 获取配置
- `POST /api/config/save` - 保存草稿
- `POST /api/config/publish` - 发布版本
- `POST /api/config/rollback` - 回滚
- `GET /api/config/export` - 导出 TOML
- `POST /api/config/import` - 导入 TOML
- `POST /api/config/simulate` - 模拟匹配
- `GET /api/healthz` - 健康检查

### Prompts
- `GET /api/prompts` - 列表
- `GET /api/prompts/{name}` - 获取
- `PUT /api/prompts/{name}` - 更新
- `DELETE /api/prompts/{name}` - 删除

## 前端页面速查

| 页面 | URL | 功能 |
|------|-----|------|
| Dashboard | `/pages/dashboard` | 概览、导入/导出 |
| Bots | `/pages/bots` | Bot 管理 |
| AI | `/pages/ai-profiles` | AI Profile 管理 |
| 工具 | `/pages/tools` | 工具管理 |
| 规则 | `/pages/rules` | 规则模板/实例 |
| Prompts | `/pages/prompts` | Prompt 编辑 |
| 模拟器 | `/pages/simulator` | 规则模拟测试 |
| 设置 | `/pages/settings` | 全局设置 |

## 测试命令

```bash
# 健康检查
curl http://localhost:4399/api/healthz

# 获取配置（需要鉴权）
curl -H "Authorization: Bearer $GEWE_API_TOKEN" \
  http://localhost:4399/api/config

# 导出配置
curl -O http://localhost:4399/api/config/export

# 模拟匹配
curl -X POST http://localhost:4399/api/config/simulate \
  -H "Content-Type: application/json" \
  -d '{"app_id":"wx_xxx","msg_kind":"text","chat":"private","content":"hello"}'
```

## 数据库操作

### 查看当前配置
```sql
SELECT current_version, etag, last_published_at
FROM config_current;
```

### 查看所有版本
```sql
SELECT version, remark, created_at
FROM config_releases
ORDER BY version DESC;
```

### 查看 Prompts
```sql
SELECT name, size, updated_at
FROM prompts
ORDER BY name;
```

## 故障排查

### 编译失败
```bash
cargo clean
cargo build -p gewe-bot-app
```

### 服务无法启动
```bash
# 检查端口占用
lsof -i :4399

# 查看详细错误
RUST_LOG=debug cargo run -p gewe-bot-app -- config/bot-app.v2.toml
```

### Postgres 连接失败
```bash
# 测试连接
psql $POSTGRES_URL -c "SELECT 1"

# 手动运行迁移
sqlx migrate run --database-url $POSTGRES_URL
```

## 目录结构

```
gewe-rs/
├── config/
│   ├── bot-app.v2.toml      # 配置文件
│   ├── prompts/             # Prompt 文件目录
│   └── backups/             # 备份目录
├── crates/gewe-bot-app/
│   ├── static/              # 前端文件
│   ├── migrations/          # 数据库迁移
│   └── src/
│       ├── api/             # API 层
│       ├── storage/         # 存储抽象层
│       └── ...
└── docs/
    ├── tasks/               # 任务文档
    └── USAGE.md             # 使用指南
```

## 下一步

1. ✅ 编辑 `config/bot-app.v2.toml` 配置 Bot 信息
2. ✅ 设置必需的环境变量
3. ✅ 启动服务
4. ✅ 浏览器访问 `http://localhost:4399/`
5. ✅ 在界面中管理配置
6. ✅ 测试模拟器功能
7. ✅ 发布版本
8. ✅ （可选）迁移到 Postgres

## 完成状态

- [x] 阶段 0：准备
- [x] 阶段 1：后端 API
- [x] 阶段 2：前端界面
- [x] 阶段 3：Postgres 适配
- [x] 阶段 4：加固与观测

**系统已就绪！** 🎉
