# 任务分解：配置前端/后端（V2 -> Postgres 预留）

## 阶段 0：准备
- 确认运行方式：先文件真源（config/bot-app.v2.toml + prompts/），后续切 Postgres。
- 环境变量约定：`GEWE_BOT_CONFIG` 指向 V2 文件；未来 DB URL 使用 `POSTGRES_URL`。

## 阶段 1：后端 API 雏形（文件存储）
- [ ] `GET /api/config`：返回当前配置 JSON。
- [ ] `POST /api/config/lint`：校验提交的 JSON（serde + schema）。
- [ ] `POST /api/config/save`：保存草稿到文件（带 etag/hash 乐观锁）。
- [ ] `POST /api/config/publish`：发布版本（文件模式可做备份/打 tag）。
- [ ] `POST /api/config/rollback`：从备份/历史版本恢复。
- [ ] `POST /api/config/simulate`：输入样例消息，返回匹配规则与动作。
- [ ] `GET/PUT /api/prompts/:name`：读取/写入 prompts 目录下的提示词文件。
- [ ] `GET /api/config/meta`：当前版本/最近 reload 结果。

## 阶段 2：前端（最小可用）
- [ ] 基础布局：概览（版本、状态、Reload 结果）。
- [ ] 全局设置表单：server/storage/defaults/defaults.ai。
- [ ] Bots 管理：列表、编辑 token_env/webhook_secret_env/base_url/tags。
- [ ] AI Profiles：模型/代理/prompt（多行编辑、保存到文件）、工具选择。
- [ ] Tools：program/args/timeout/max_output/pre/post_reply/description/parameters。
- [ ] 规则模板：kind/match/action/defaults（require_mention）。
- [ ] 规则实例：模板引用、channel/from/priority/overrides/enabled。
- [ ] 草稿/发布/回滚按钮；校验提示；diff 展示（可简化为 before/after JSON）。
- [ ] 模拟器：输入 app_id/chat/content/from_wxid，展示命中规则链路。
- [ ] 密钥显示：仅显示 env 名，不回显值。

## 阶段 3：Postgres 适配
- [ ] 建表脚本（见 design 文档），添加 `POSTGRES_URL` 配置。
- [ ] API 层增加存储抽象：file vs postgres；配置切换参数。
- [ ] 发布/回滚使用 releases 表；prompts 可选入库。
- [ ] 导入/导出：文件 <-> DB 的迁移脚本。
- [ ]（可选）reload 推送接口，为 RabbitMQ/Redis 预留。

## 阶段 4：加固与观测
- [ ] 文件写入加锁/乐观锁（etag/hash）。
- [ ] 路径白名单（仅允许 config/prompts）。
- [ ] 简单鉴权（token/basic-auth）。
- [ ] 健康检查 `/api/healthz`。
- [ ] 日志格式开关：彩色 stdout / JSON / 文件。

## 参考运行
```bash
# 文件模式
GEWE_BOT_CONFIG=config/bot-app.v2.toml \
GEWE_BOT_TOKEN_MAIN=... \
GEWE_WEBHOOK_SECRET_MAIN=... \
GEMINI_API_KEY=... \
cargo run -p gewe-bot-app
```
切 Postgres 时，增加 `POSTGRES_URL=postgresql://user:pass@host:5432/db`，并在配置/API 中切换存储后端。

