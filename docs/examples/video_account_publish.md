# 视频号作品发布示例

1. **上传素材至 CDN**

   ```bash
   gewe video-account upload-finder-video \
     --token $GEWE_TOKEN \
     --bot-alias demo-bot \
     --video-url https://example.com/video.mp4 \
     --cover-img-url https://example.com/cover.jpg
   ```

   记录输出中的 `fileUrl/thumbUrl/mp4Identify/fileKey` 等字段，供后续发布复用。

2. **调用视频号 CDN 发布**

   ```bash
   gewe video-account publish-finder-cdn \
     --token $GEWE_TOKEN \
     --bot-alias demo-bot \
     --my-user-name v2_xxx@finder \
     --my-role-type 3 \
     --topic "#新品,#测试" \
     --description "首发 Demo 视频" \
     --file-url "<上传返回的 fileUrl>" \
     --thumb-url "<上传返回的 thumbUrl>" \
     --mp4-identify "<上传返回的 mp4Identify>" \
     --file-size 1315979 \
     --thumb-md5 "<上传返回的 thumbMD5>" \
     --file-key "<上传返回的 fileKey>"
   ```

   成功后 CLI 会打印接口响应，可用 `id`、`code` 等字段排查发布状态。
