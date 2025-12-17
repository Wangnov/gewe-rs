# Get Profile

1. 读取当前设备资料：`gewe get-profile --bot-alias <alias|wxid|appId>`；命令按 alias > bot_app_id > app_id 解析 Bot，未指定 token/base_url 时回退配置/默认。
2. 输出包含 wxid/微信号/昵称/头像/地区/签名等字段，命令会以 Debug JSON 打印，可配合 `jq` 过滤：`gewe get-profile ... | jq '.nick_name, .wxid'`。
