# Send Text via Bot Alias

1. Ensure config has bots (appId/wxid) and optionally alias via `gewe config --alias <alias> --alias-target-app-id <appId>`.
2. Send message using alias:
   `gewe send-text --bot-alias <alias|appId|wxid> --to-wxid <target> --content "hello"`
3. Order of resolution: `--bot-alias` > `--bot-app-id` > `--app-id` > config default appId.
