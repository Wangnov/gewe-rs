# Fetch Contacts List

1. 设备上线后可直接触发：`gewe fetch-contacts-list --bot-alias <alias|wxid|appId>`，命令会自动按 alias > bot_app_id > app_id 的顺序解析 Bot，并回退到配置/默认 base_url。
2. 如果接口因好友量大而超时，循环执行 `gewe fetch-contacts-list-cache --bot-alias <alias>`（建议 10s 间隔）即可读取最近一次通讯录缓存。
3. 查询结果会 JSON Debug 形式输出到 stdout，可配合 `jq`/重定向做持久化，例如 `gewe fetch-contacts-list ... > contacts.json`。
