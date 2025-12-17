# CLI 邀请成员示例

`gewe invite-member` 现在要求显式提供 `--reason`，该字段会透传给官方 API 的 `reason` 参数。下面示例在群聊 `123@chatroom` 中邀请两个好友并注明用途：

```bash
gewe invite-member \
  --token "$GEWE_TOKEN" \
  --app-id "$GEWE_APP_ID" \
  --chatroom-id 123@chatroom \
  --wxids wxid_a,wxid_b \
  --reason "新人入职欢迎"
```

如需通过机器人别名发送，可额外指定 `--bot-alias dev-bot`；`--reason` 同样必填，可配合自动化脚本根据不同场景生成描述，方便后台审计。
