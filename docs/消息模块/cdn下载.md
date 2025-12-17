# cdn下载

## OpenAPI Specification

```yaml
openapi: 3.0.1
info:
  title: ''
  description: ''
  version: 1.0.0
paths:
  /gewe/v2/api/message/downloadCdn:
    post:
      summary: cdn下载
      deprecated: false
      description: '**注意** 如果是下载图片失败，可尝试下载另外两种图片类型，并非所有图片都会有高清、常规图片'
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
                aesKey:
                  type: string
                  description: cdn的aeskey
                fileId:
                  type: string
                  description: cdn的fileid
                type:
                  type: string
                  description: 下载的文件类型 1：高清图片 2：常规图片 3：缩略图 4：视频 5：文件
                totalSize:
                  type: string
                  description: 文件大小
                suffix:
                  type: string
                  description: 下载类型为文件时，传文件的后缀（例：doc）
              x-apifox-orders:
                - appId
                - aesKey
                - fileId
                - type
                - totalSize
                - suffix
              required:
                - appId
                - aesKey
                - totalSize
                - type
                - fileId
                - suffix
            example:
              appId: '{{appid}}'
              aesKey: f46be643aa0dc009ae5fb63bbc73335d
              totalSize: '63'
              type: '5'
              fileId: >-
                3057020100044b304902010002043904752002032f7d6d02046bb5bade02046593760c042433653765306131612d646138622d346662322d383239362d3964343665623766323061370204051400050201000405004c53d900
              suffix: json
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
                      fileUrl:
                        type: string
                        description: 文件链接地址，7天有效
                    required:
                      - fileUrl
                    x-apifox-orders:
                      - fileUrl
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
                  fileUrl: >-
                    http://wxapii.oos-sccd.ctyunapi.cn/20240102/wx_HJDZOY09-ucKMhzu8jQ2Z/f6eee8b1-fd10-451e-a8eb-d64d5eed1274.json?AWSAccessKeyId=9e882e7187c38b431303&Expires=1704768509&Signature=4ZHUE7xfJziBpAcYIsvlKTpWJDI%3D
          headers: {}
          x-apifox-name: 成功
      security: []
      x-apifox-folder: 基础API/消息模块/下载
      x-apifox-status: released
      x-run-in-apifox: https://app.apifox.com/web/project/3475559/apis/api-139908333-run
components:
  schemas: {}
  securitySchemes: {}
servers:
  - url: http://api.geweapi.com
    description: 测试环境
security: []

```
