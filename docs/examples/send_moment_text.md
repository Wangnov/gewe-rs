# Send Moment Text

1. 新设备上线 3 天后，可直接发布纯文字朋友圈：`gewe send-moment-text --bot-alias <alias|wxid|appId> --content "新年快乐"`。如需限制可见范围，可追加 `--allow-wxids`/`--disable-wxids`/`--privacy true` 等参数（逗号分隔多值）。
2. 图片、视频朋友圈需先上传素材：`gewe upload-moment-image --img-url https://example.com/a.jpg --img-url https://example.com/b.jpg`，再把返回的 `fileUrl,thumbUrl,fileMd5,length,width,height` 通过 `--img-info` 传给 `gewe send-moment-image`。
3. CLI 会打印朋友圈 ID、作者及发布时间，可结合 `gewe get-moment-detail --sns-id <id>` 查看或用于后续删除、点赞、评论等操作。
