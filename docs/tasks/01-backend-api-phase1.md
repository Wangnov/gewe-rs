# 阶段 1 完成记录：后端 API 实现

> 完成时间：2024-12-04
> 状态：✅ 已完成

## 概述

阶段 1 实现了配置管理的后端 API，基于文件存储模式，为后续前端开发提供数据接口。

## 完成的功能

### 1. API 模块结构

新增文件：
```
crates/gewe-bot-app/src/api/
├── mod.rs      # API 路由定义
├── config.rs   # 配置相关 API (420 行)
├── prompts.rs  # Prompts API (250 行)
└── state.rs    # 共享状态管理 (290 行)
```

### 2. 配置 API 端点

#### GET /api/config
- **功能**：获取当前 V2 配置
- **响应**：
```json
{
  "success": true,
  "data": {
    "config": { /* AppConfigV2 JSON */ },
    "etag": "sha256_hash"
  }
}
```

#### POST /api/config/lint
- **功能**：校验配置 JSON
- **请求**：
```json
{
  "config": { /* AppConfigV2 JSON */ }
}
```
- **响应**：
```json
{
  "success": true,
  "data": {
    "valid": false,
    "errors": ["bots[0]: app_id 不能为空", "..."]
  }
}
```

#### GET /api/config/meta
- **功能**：获取配置元信息
- **响应**：
```json
{
  "success": true,
  "data": {
    "version": 1,
    "etag": "sha256_hash",
    "has_draft": false,
    "last_published_at": "2024-12-04T12:00:00Z",
    "last_saved_at": null,
    "last_reload_at": "2024-12-04T12:00:00Z",
    "last_reload_result": "ok",
    "available_backups": [
      {
        "version": 1,
        "filename": "bot-app.v2.toml.v1.20241204120000",
        "created_at": "2024-12-04T12:00:00Z",
        "remark": null
      }
    ]
  }
}
```

#### POST /api/config/save
- **功能**：保存配置草稿（支持乐观锁）
- **请求**：
```json
{
  "config": { /* AppConfigV2 JSON */ },
  "expected_etag": "previous_etag_for_optimistic_lock"  // 可选
}
```
- **响应**：
```json
{
  "success": true,
  "data": {
    "etag": "new_sha256_hash",
    "saved_at": "2024-12-04T12:00:00Z"
  }
}
```
- **错误**：
  - 409 Conflict - ETag 不匹配（配置已被修改）
  - 400 Bad Request - 配置校验失败

#### POST /api/config/publish
- **功能**：发布配置（创建备份）
- **请求**：
```json
{
  "remark": "版本说明"  // 可选
}
```
- **响应**：
```json
{
  "success": true,
  "data": {
    "version": 2,
    "published_at": "2024-12-04T12:00:00Z",
    "backup_filename": "bot-app.v2.toml.v2.20241204120000"
  }
}
```

#### POST /api/config/rollback
- **功能**：回滚到指定版本
- **请求**：
```json
{
  "version": 1
}
```
- **响应**：
```json
{
  "success": true,
  "data": {
    "version": 1,
    "rolled_back_at": "2024-12-04T12:00:00Z"
  }
}
```

#### POST /api/config/simulate
- **功能**：模拟消息匹配
- **请求**：
```json
{
  "app_id": "wx_xxx",
  "msg_kind": "text",       // text/image/voice/video/emoji/link/file_notice
  "chat": "private",        // private/group
  "content": "hello",
  "from_wxid": "wxid_xxx",  // 可选
  "mentioned": false        // 是否被 @ 了机器人
}
```
- **响应**：
```json
{
  "success": true,
  "data": {
    "matched": true,
    "rules": [
      {
        "instance_id": "ai_qa_private",
        "template_id": "ai_qa",
        "priority": 100,
        "action_summary": "ai(gemini_default), log"
      }
    ],
    "final_action": "ai(gemini_default), log"
  }
}
```

### 3. Prompts API 端点

#### GET /api/prompts
- **功能**：列出所有 prompt 文件
- **响应**：
```json
{
  "success": true,
  "data": {
    "prompts": [
      {
        "name": "ai_system.txt",
        "size": 1234,
        "modified_at": "2024-12-04 12:00:00"
      }
    ]
  }
}
```

#### GET /api/prompts/{name}
- **功能**：获取指定 prompt 内容
- **安全限制**：文件名必须是 `.txt` 或 `.md` 后缀，不能包含路径
- **响应**：
```json
{
  "success": true,
  "data": {
    "name": "ai_system.txt",
    "content": "You are a helpful assistant..."
  }
}
```

#### PUT /api/prompts/{name}
- **功能**：写入 prompt 内容
- **请求**：
```json
{
  "content": "New prompt content..."
}
```
- **响应**：
```json
{
  "success": true,
  "data": {
    "name": "ai_system.txt",
    "size": 1234,
    "saved_at": "2024-12-04 12:00:00"
  }
}
```

### 4. 配置结构序列化支持

修改 `config.rs`，为所有 V2 配置结构添加 `Serialize` 派生：

- `AppConfigV2`
- `ServerConfigV2`
- `StorageConfigV2`
- `DefaultsV2`
- `DefaultsAiV2`
- `BotConfigV2`
- `AiProfileV2`
- `ToolConfigV2`
- `RuleTemplateV2`
- `TemplateDefaultsV2`
- `MatchConfigV2`
- `TemplateActionV2`
- `InstanceOverridesV2`
- `RuleInstanceV2`
- `RuleKind` (枚举)
- `ReplyMode` (枚举)
- `FromConfig`

### 5. 配置校验功能

`AppConfigV2::validate()` 方法实现以下校验：

- `config_version` 必须为 2
- `bots`:
  - `app_id` 不能为空
  - `base_url` 不能为空
  - `token` 或 `token_env` 必须设置其一
  - id/app_id 不能重复
- `ai_profiles`:
  - `id` 不能为空
  - `model` 不能为空
  - id 不能重复
  - `tool_ids` 引用的工具必须存在
- `tools`:
  - `id` 不能为空
  - `program` 不能为空
  - id 不能重复
- `rule_templates`:
  - `id` 不能为空
  - id 不能重复
  - `action.ai_profile` 引用必须存在
- `rule_instances`:
  - `id` 不能为空
  - `template` 不能为空且必须存在
  - id 不能重复
  - `channel` 必须是 private/group/both
  - `overrides.ai_profile` 引用必须存在

### 6. 状态管理

`ApiState` 实现：

- **线程安全**：使用 `Arc<RwLock>` 共享状态
- **ETag 计算**：SHA256 哈希防止并发覆盖
- **备份管理**：
  - 自动扫描备份目录
  - 创建备份时自动递增版本号
  - 备份文件格式：`bot-app.v2.toml.v{version}.{timestamp}`

### 7. 主程序集成

修改 `main.rs`：
- 初始化 `ApiState`
- 将 API 路由挂载到 `/api` 路径
- 日志输出包含 API 路径信息

## 依赖变更

`Cargo.toml` 新增：
```toml
sha2 = { workspace = true }
hex = { workspace = true }
chrono = { version = "0.4", features = ["serde"] }
```

## 测试结果

```
running 8 tests
test api::prompts::tests::test_is_safe_filename ... ok
test api::state::tests::test_compute_etag ... ok
test api::state::tests::test_parse_backup_filename ... ok
test tools::claude_changelog::tests::test_parse_query ... ok
test tools::claude_changelog::tests::test_parse_changelog ... ok
test tools::tool_versions::tests::test_normalize_tool_id ... ok
test tools::gemini_image::tests::test_parse_query ... ok
test tools::tool_versions::tests::test_parse_query ... ok

test result: ok. 8 passed; 0 failed
```

## 文件变更汇总

| 文件 | 变更类型 | 说明 |
|------|----------|------|
| `src/api/mod.rs` | 新增 | API 路由定义 |
| `src/api/config.rs` | 新增 | 配置 API 处理函数 |
| `src/api/prompts.rs` | 新增 | Prompts API 处理函数 |
| `src/api/state.rs` | 新增 | 共享状态管理 |
| `src/config.rs` | 修改 | 添加 Serialize、validate()、parse()、to_toml() |
| `src/main.rs` | 修改 | 集成 API 路由 |
| `Cargo.toml` | 修改 | 添加 sha2、hex、chrono 依赖 |

## 后续任务

阶段 1 完成后，下一步是 **阶段 2：前端实现（htmx 方案）**。

详见：`docs/tasks/02-frontend-htmx-plan.md`
