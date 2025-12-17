# 管理好友标签

1. **新增标签**

   ```bash
   gewe add-label \
     --token $GEWE_TOKEN \
     --bot-alias demo-bot \
     --label-name "核心客户"
   ```

   输出包含 `labelId/labelName`，用于后续绑定或删除。

2. **查看标签列表**

   ```bash
   gewe list-labels --bot-alias demo-bot
   ```

   CLI 会打印全部标签 JSON，可确认最新 ID。

3. **批量更新某些好友的标签**

   ```bash
   gewe modify-label-members \
     --bot-alias demo-bot \
     --label-ids 12,18 \
     --wx-id wxid_a123,wxid_b456
   ```

   传入标签 ID 需为好友当前最终集合（新增/删除都需要提供全量列表）。

4. **删除无用标签**

   ```bash
   gewe delete-label \
     --bot-alias demo-bot \
     --label-ids 18
   ```

   支持传入逗号分隔的多个 ID。
