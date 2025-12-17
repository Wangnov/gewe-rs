# 发送小程序消息（CLI）

```bash
cargo run -p gewe-cli -- send-mini-app \
  --to-wxid filehelper \
  --mini-app-id wx1f9ea355b47256dd \
  --display-name "百果园+" \
  --page-path pages/homeDelivery/index.html \
  --cover-img-url https://download-1308498490.cos.ap-guangzhou.myqcloud.com/cover.jpg \
  --title "最快29分钟 好吃水果送到家" \
  --user-name gh_690acf47ea05@app
```

```bash
# 使用 bot alias 指定 appId
cargo run -p gewe-cli -- send-mini-app \
  --bot-alias workbot \
  --to-wxid 34757816141@chatroom \
  --mini-app-id wx1f9ea355b47256dd \
  --display-name "示例小程序" \
  --page-path pages/index.html \
  --cover-img-url https://example.com/cover.png \
  --title "示例标题" \
  --user-name gh_foo@app
```
