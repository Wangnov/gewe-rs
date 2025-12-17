# 同步与查看收藏夹

1. **同步收藏夹索引**

   ```bash
   gewe sync-favorites \
     --token $GEWE_TOKEN \
     --bot-alias demo-bot \
     --sync-key ""
   ```

   首次 `syncKey` 传空；后续将响应里的 `syncKey` 作为下一次 `--sync-key` 继续翻页。

2. **获取收藏具体内容**

   ```bash
   gewe get-favorite-content \
     --bot-alias demo-bot \
     --fav-id 179
   ```

   输出将包含收藏的 XML 内容以及状态、更新时间等信息。

3. **删除收藏夹记录**

   ```bash
   gewe delete-favorite \
     --bot-alias demo-bot \
     --fav-id 179
   ```

   删除成功会返回日志提示，可配合同步接口 flag 字段确认。
