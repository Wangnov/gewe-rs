# 发送emoji消息

## OpenAPI Specification

```yaml
openapi: 3.0.1
info:
  title: ''
  description: ''
  version: 1.0.0
paths:
  /gewe/v2/api/message/postEmoji:
    post:
      summary: 发送emoji消息
      deprecated: false
      description: ''
      tags:
        - 基础API/消息模块
      parameters:
        - name: X-GEWE-TOKEN
          in: header
          description: ''
          required: true
          example: '{{gewe-token}}'
          schema:
            type: string
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                appId:
                  type: string
                  description: 设备ID
                  additionalProperties: false
                toWxid:
                  type: string
                  description: 好友/群的ID
                emojiMd5:
                  type: string
                  description: emoji图片的md5
                emojiSize:
                  type: integer
                  description: emoji的文件大小
              x-apifox-orders:
                - appId
                - toWxid
                - emojiMd5
                - emojiSize
              required:
                - appId
                - toWxid
                - emojiMd5
                - emojiSize
            example:
              appId: '{{appid}}'
              toWxid: 34757816141@chatroom
              emojiMd5: 4cc7540a85b5b6cf4ba14e9f4ae08b7c
              emojiSize: 102357
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: object
                properties:
                  ret:
                    type: integer
                  msg:
                    type: string
                  data:
                    type: object
                    properties:
                      toWxid:
                        type: string
                        description: 接收人的wxid
                      createTime:
                        type: integer
                        description: 发送时间
                      msgId:
                        type: integer
                        description: 消息ID
                      newMsgId:
                        type: integer
                        description: 消息ID
                      type:
                        type: integer
                        description: 消息类型
                    required:
                      - toWxid
                      - createTime
                      - msgId
                      - newMsgId
                      - type
                    x-apifox-orders:
                      - toWxid
                      - createTime
                      - msgId
                      - newMsgId
                      - type
                required:
                  - ret
                  - msg
                  - data
                x-apifox-orders:
                  - ret
                  - msg
                  - data
              example:
                ret: 200
                msg: 操作成功
                data:
                  toWxid: 34757816141@chatroom
                  createTime: null
                  msgId: 769523643
                  newMsgId: 891398861855787000
                  type: null
          headers: {}
          x-apifox-name: 成功
      security: []
      x-apifox-folder: 基础API/消息模块
      x-apifox-status: released
      x-run-in-apifox: https://app.apifox.com/web/project/3475559/apis/api-139908320-run
components:
  schemas: {}
  securitySchemes: {}
servers:
  - url: http://api.geweapi.com
    description: 测试环境
security: []

```
