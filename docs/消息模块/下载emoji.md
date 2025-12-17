# 下载emoji

## OpenAPI Specification

```yaml
openapi: 3.0.1
info:
  title: ''
  description: ''
  version: 1.0.0
paths:
  /gewe/v2/api/message/downloadEmojiMd5:
    post:
      summary: 下载emoji
      deprecated: false
      description: ''
      tags:
        - 基础API/消息模块/下载
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
                emojiMd5:
                  type: string
                  description: emoji图片的md5
              x-apifox-orders:
                - appId
                - emojiMd5
              required:
                - appId
                - emojiMd5
            example:
              appId: '{{appid}}'
              emojiMd5: cc56728d56c730ddae52baffe941ed86
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
                      url:
                        type: string
                        description: emoji表情链接地址，7天有效
                    required:
                      - url
                    x-apifox-orders:
                      - url
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
                  url: >-
                    http://wxapp.tc.qq.com/262/20304/stodownload?m=cc56728d56c730ddae52baffe941ed86&filekey=30350201010421301f02020106040253480410cc56728d56c730ddae52baffe941ed860203033b55040d00000004627466730000000132&hy=SH&storeid=2631f5928000984ff000000000000010600004f50534801c67b40b77857716&bizid=1023
          headers: {}
          x-apifox-name: 成功
      security: []
      x-apifox-folder: 基础API/消息模块/下载
      x-apifox-status: released
      x-run-in-apifox: https://app.apifox.com/web/project/3475559/apis/api-139908332-run
components:
  schemas: {}
  securitySchemes: {}
servers:
  - url: http://api.geweapi.com
    description: 测试环境
security: []

```
