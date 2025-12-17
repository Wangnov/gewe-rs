# AGENT 开发指导备忘（gewe-rs）

## 质控

- `make check`：fmt + clippy 严格。
- `make check-fix`：fmt-fix + clippy-fix（允许 dirty/staged/no-vcs）。
- 单独：`make fmt`/`fmt-fix`，`make lint`/`lint-fix`。

## 依赖与惯例

- base_url 仅写协议+域名（如 `https://www.geweapi.com`）；SDK 自动拼接 `/gewe/v2/api/...`。
- 根 `Cargo.toml` 管理依赖；特性：`sqlite`、`redis-store` 对应 SessionStore 后端。

## 模块现状

- `gewe-core`：按文档目录拆分模块（login / message::send|download|forward|revoke / group::manage|member|settings|admin / contact::info|manage|wecom / personal::profile|safety|settings / moments::publish|media|timeline|settings|interact|manage / video_account::publish|profile|follow|message|interact|scan|search / tag / favorite）。消息模型覆盖 `postText/postImage/postVoice/postVideo/postFile/postLink/postEmoji/postAppMsg/postMiniApp/postNameCard`、`downloadImage/downloadVideo/downloadFile/downloadVoice/downloadEmojiMd5/downloadCdn`、`forwardImage/forwardVideo/forwardFile/forwardMiniApp/forwardUrl`、`revokeMsg`；群模型覆盖 docs/群模块 19 个接口；联系人模型覆盖 docs/联系人模块 16 个接口；个人资料模型覆盖 docs/个人资料模块（获取/更新资料、更新头像、获取二维码、隐私设置、设备记录）；朋友圈模型覆盖 docs/朋友圈模块（上传图片/视频、发图文/视频/链接、转发、列表/详情、下载视频、删除、点赞/评论、隐私设置、陌生人可见）；视频号模型覆盖 docs/视频号模块全量接口（素材上传/发布/朋友圈/消息、关注/列表/搜索、扫码系列、点赞收藏/评论/浏览、消息列表/私信同步、账号建模/主页/二维码/数据查询）；标签模块覆盖添加/删除/列表/成员更新；收藏夹模块覆盖同步/详情/删除；登录模型覆盖取码、扫码检查、弹框登录、账号密码登录、设置回调、Mac 转 iPad、在线检测/断线重连/退出等接口；`BotId/AppId` newtype。
- `gewe-http`：GeweHttpClient + 按目录拆分方法。消息/群/联系人/个人资料/朋友圈/视频号/标签/收藏夹 SDK 覆盖上述接口；登录接口 `get_login_qr_code`、`check_login`（reqwest）。Decode 失败会回显原始 body 便于排查。
- `gewe-webhook`：Axum `/webhook`，去重（Appid+NewMsgId），mpsc 队列，签名校验（X-GEWE-TOKEN、X-GEWE-TIMESTAMP±300s、X-GEWE-SIGN=HMAC-SHA256(secret, "{timestamp}:{body}"), secret 优先 webhook_secret 否则 token）。
- `gewe-session`：内存去重（VecDeque 1024）；可选 SQLite/Redis 简易实现（JSON 序列化）。
- `gewe-cli`：配置 `~/.config/gewe/config.toml` 或 ProjectDirs；子命令覆盖消息 + 群 19 个接口 + 联系人 16 个接口 + 个人资料 6 个接口 + 朋友圈 17 个接口 + 标签/收藏夹命令 + 视频号命令集（统一 `video-account` 子命令）及登录/配置；全局 `-v/--verbose`（-v=debug，-vv=trace）。Bot 路由：alias > bot_app_id > app_id > 配置；base_url 取 flag/配置/默认 `http://api.geweapi.com`；持久化 bots(appId/wxid/alias)。
- 文档示例：`docs/examples/multi_bot_login.md`，`docs/examples/send_text_alias.md`，`docs/examples/send_voice.md`，`docs/examples/send_mini_app.md`，`docs/examples/download_image.md`，`docs/examples/fetch_contacts_list.md`，`docs/examples/get_profile.md`，`docs/examples/send_moment_text.md`，`docs/examples/video_account_publish.md`，`docs/examples/manage_labels.md`，`docs/examples/favorite_sync.md`，`docs/examples/webhook_setup.md`，`docs/examples/invite_members_with_reason.md`（可继续补充）。
  - 任务指南：`TASK_API_EXPANSION.md` 描述扩充消息/群类 API 的操作步骤（阅读顺序、context7 用法、质控要求）。

## Webhook 关键点

- 3 秒快速 200，事件入 mpsc 队列。
- 去重：Appid + NewMsgId（支持嵌套 Data.NewMsgId）。
- 签名：校验 `X-GEWE-TOKEN/TIMESTAMP/SIGN`，时间窗 ±300s。

## CLI 提示

- 运行：`cargo run -p gewe-cli -- <cmd>` 或 `target/debug/gewe-cli`，可加 `-v/--verbose`。
- 配置命令：`gewe config --token ... --base-url ... --app-id ...`；`--list-bots`/`--alias` 管理 bots。
- 消息/群命令均支持 `--bot-alias` / `--bot-app-id` / `--app-id`；不写 base_url 时回退 config。

## 待办（提醒）

- SessionStore TTL/原子写；Webhook 队列持久化示例；CLI Bot 路由/alias 交互优化与帮助文档；群/联系人/视频号模块示例补充；gRPC/Tauri 落地。
