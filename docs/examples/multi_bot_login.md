# Multi-Bot Login Workflow

1. Run `gewe config --token <token>` to store token.
2. For each bot:
   - `gewe get-login-qr --device-type ipad --region-id 320000` (appId 可为空，默认写入配置)
   - Poll `gewe check-login --uuid <uuid>` until `status=2`；配置将存储 appId 与 wxid。
3. 发送消息：
   - `gewe send-text --bot-alias <appId|wxid|alias> --to-wxid <target> --content <text>`
   - 或显式 `--bot-app-id`/`--app-id`。
4. 查看配置与 Bot 列表：`gewe config --list-bots`。
