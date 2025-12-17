# 发送语音消息（CLI）

```bash
# 从配置读取 token/base_url/app_id，发送 silk 语音，单位毫秒
cargo run -p gewe-cli -- send-voice \
  --to-wxid 34757816141@chatroom \
  --voice-url "https://example.com/demo.silk" \
  --voice-duration 2000
```
```
# 自定义 base_url 与 bot alias
cargo run -p gewe-cli -- send-voice \
  --bot-alias workbot \
  --voice-url "https://example.com/demo.silk" \
  --voice-duration 3500 \
  --base-url http://api.geweapi.com
```
