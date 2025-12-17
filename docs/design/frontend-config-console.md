# 配置前端 & 后端设计（V2 配置 + Postgres 预留）

## 目标
- 提供一个前端配置台，直观管理 bot 配置、AI 配置、工具和规则（模板+实例）。
- 兼容当前 V2 文件真源，后续可切换到 Postgres 作为真源。
- 支持校验、草稿/发布、模拟命中、审计简要信息。

## 当前配置结构（V2）
- 顶层：`server`、`storage`、`defaults`、`defaults.ai`。
- `bots`：app_id/token_env/base_url/webhook_secret_env/tags。
- `ai_profiles`：provider/model/base_url/api_key_env/system_prompt_file/user_prefix/tool_ids。
- `tools`：program/args/timeout/max_output/pre/post_reply/description/parameters。
- `rule_templates`：kind/match/action/defaults（require_mention）。
- `rule_instances`：template/channel/from/priority/overrides(enabled)。
- 提示词文件：`prompts/ai_system.txt`。

## 后端接口规划
- `GET /api/config`：读取当前配置（JSON），支持 `source=file/postgres` 参数，默认 file。
- `POST /api/config/lint`：提交配置 JSON，返回校验结果（serde/JSON Schema）。
- `POST /api/config/save`：保存草稿（文件写入或 DB upsert），带版本/etag 防并发覆盖。
- `POST /api/config/publish`：将草稿标记为生效版本，写 releases 记录（文件模式可简单备份/打 tag）。
- `POST /api/config/rollback`：切回指定版本（DB 用 releases 表；文件模式从备份恢复）。
- `POST /api/config/simulate`：输入样例消息（app_id/msg_type/chat/content/from_wxid），返回命中规则链路与动作。
- `GET /api/config/meta`：返回当前版本/最新发布/最近 reload 结果。
- 提示词文件：支持 `GET/PUT /api/prompts/:name` 读取/写入（限制目录 `prompts/`）。

## 数据存储
- 现阶段：文件模式，真源 `config/bot-app.v2.toml` + `prompts/`，写入需加文件锁或乐观锁（etag/hash）。
- 预留 Postgres：
  - 表建议：`bots`、`ai_profiles`、`tools`、`rule_templates`、`rule_instances`、`releases`、`prompts`（可选）。
  - releases 保存 snapshot JSON + 备注 + created_at。
  - API 层保持同一 JSON 结构，切换存储时不改前端。
- 迁移策略：提供 `export/import`，文件 <-> DB 的互转脚本。

## 校验与约束
- serde + JSON Schema 校验：kind/channel 枚举，必填项，超长截断提示，参数类型检查。
- 工具 parameters 默认填 `{ "type": "object" }` 避免空 schema（已修复）。
- 引用 system_prompt_file 需验证路径落在 `prompts/` 下，防任意读写。

## 前端功能
- 概览：当前版本/草稿状态，Reload 结果。
- 全局设置：server/storage/defaults/defaults.ai。
- Bot 管理：app_id/token_env/webhook_secret_env/base_url/tags。
- AI 配置：模型/代理/提示词（文件编辑/上传）、工具引用。
- 工具管理：program/参数/超时/输出上限/描述。
- 规则模板：kind/match/action/defaults。
- 规则实例：模板引用、channel、from.w xid、priority、overrides、enabled。
- 草稿/发布：编辑 -> 保存草稿 -> 校验 -> diff -> 发布；回滚按钮。
- 模拟：输入消息，展示命中规则和最终动作。
- 密钥显示：仅显示 env 名，不回显值。

## Postgres 设计（预留）
- 表（简化版）：
  - `bots(id serial, app_id text unique, token_env text, base_url text, webhook_secret_env text, tags jsonb, updated_at timestamptz)`.
  - `ai_profiles(id text pk, provider text, model text, base_url text, api_key_env text, system_prompt text, system_prompt_file text, user_prefix text, tool_ids jsonb, updated_at timestamptz)`.
  - `tools(id text pk, program text, args jsonb, timeout_secs int, max_output int, pre_reply text, post_reply text, description text, parameters jsonb, updated_at timestamptz)`.
  - `rule_templates(id text pk, name text, kind text, match jsonb, action jsonb, defaults jsonb, updated_at timestamptz)`.
  - `rule_instances(id text pk, template text, channel text, from_wxid text, priority int, overrides jsonb, enabled bool, updated_at timestamptz)`.
  - `releases(id serial, version text, snapshot jsonb, status text, created_at timestamptz, remark text)`.
  - `prompts(name text pk, content text, updated_at timestamptz)`（可选）。
- 索引：app_id 唯一，rule_instances(priority, enabled)，releases(version)。
- API 层按 JSON 模型读写；发布时写 releases + 推送 reload（未来可用 RabbitMQ）。

## 安全与并发
- 文件模式：写前比对 etag/hash，避免覆盖；路径白名单（只允许 `config/`、`prompts/`）。
- DB 模式：事务写入，releases 单调版本号；操作需鉴权（最简 token/basic-auth）。

## 日志与观测
- 提供 `GET /api/healthz`、`/api/config/meta`。
- 日志可选彩色输出（stdout）、JSON/文件输出控制（环境变量）。

## 后续扩展
- 加入 RabbitMQ/Redis 推送 reload 通知。
- UI 增加 diff/版本列表、批量启停规则、灰度（按 bot tag/百分比）。

