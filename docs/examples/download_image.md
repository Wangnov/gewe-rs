# 下载图片（CLI）

```bash
# xml 来自 webhook 回调，type=2 表示常规图片
cargo run -p gewe-cli -- download-image \
  --xml "<msg><img ... /></msg>" \
  --image-type 2
```

```bash
# 指定 bot alias + 自定义 base_url
cargo run -p gewe-cli -- download-image \
  --bot-alias workbot \
  --xml "<msg><img ... /></msg>" \
  --image-type 1 \
  --base-url http://api.geweapi.com
```
