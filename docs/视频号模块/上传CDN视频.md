# 上传CDN视频

## OpenAPI Specification

```yaml
openapi: 3.0.1
info:
  title: ''
  description: ''
  version: 1.0.0
paths:
  /gewe/v2/api/finder/uploadFinderVideo:
    post:
      summary: 上传CDN视频
      deprecated: false
      description: >-
        和[发布视频接口](http://doc.geweapi.com/endpoint-144557553)实现的功能一样，此接口建议多个号批量发布时使用，某个号调用1次[上传CDN视频](http://doc.geweapi.com/endpoint-144561904)，其余号直接调用[CDN发布](http://doc.geweapi.com/endpoint-144563194)，无需重复上传。
      tags:
        - 基础API/视频号模块
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
                videoUrl:
                  type: string
                  description: 视频链接地址
                coverImgUrl:
                  type: string
                  description: 封面链接地址
              required:
                - appId
                - videoUrl
                - coverImgUrl
              x-apifox-orders:
                - appId
                - videoUrl
                - coverImgUrl
            example:
              appId: '{{appid}}'
              proxyIp: ''
              videoUrl: >-
                https://cos.ap-shanghai.myqcloud.com/pkg/436fa030-18a45a6e917.mp4
              coverImgUrl: http://dummyimage.com/400x400
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
                        description: 视频文件链接
                      thumbUrl:
                        type: string
                        description: 封面图链接
                      mp4Identify:
                        type: string
                        description: 文件ID
                      fileSize:
                        type: integer
                        description: 文件大小
                      thumbMD5:
                        type: string
                        description: 封面图md5
                      fileKey:
                        type: string
                        description: 文件的key
                    required:
                      - fileUrl
                      - thumbUrl
                      - mp4Identify
                      - fileSize
                      - thumbMD5
                      - fileKey
                    x-apifox-orders:
                      - fileUrl
                      - thumbUrl
                      - mp4Identify
                      - fileSize
                      - thumbMD5
                      - fileKey
                    description: 可通过如下参数调用cdn发布视频接口
                required:
                  - ret
                  - msg
                  - data
                x-apifox-orders:
                  - ret
                  - msg
                  - data
              examples:
                '1':
                  summary: 成功示例
                  value:
                    ret: 200
                    msg: 操作成功
                    data:
                      fileUrl: >-
                        http://wxapp.tc.qq.com/251/20302/stodownload?a=1&bizid=1023&dotrans=0&encfilekey=Cvvj5Ix3eexKX1zo1IZZBrQomawdVfSQH1uu2U31EqFrUA5xctbdDlGGkhM5r9b4e7lDdgzBiaffgFRzukh66M2lXMjLCibKxwU0PWibofftsXd4MHJfNM3VHq2dvmoibcEWE363ibcKI0eTQEIjluPstxRwNxUlPI0iamxHoIKIbaxVM&hy=SH&idx=1&m=6e95f9d79588843ac259b780f0cbf20f&token=cztXnd9GyrEsWrS4eJynZnXPAO12gKrhygeBeB1Zic0orX2aeKcU6ZCsuRHVNiaicw7CQ9M5VgFq8Wut9uMm1QQPA&upid=500210
                      thumbUrl: >-
                        http://wxapp.tc.qq.com/251/20350/stodownload?bizid=1023&dotrans=0&encfilekey=okgXGMsUNLEibHKtCw1bRNicxw6C1zsevQuNo2sjfLcsBDAAjgT6M9OY6Z9VcUKoBHpJsck5dZqOdbCEY7gZhWCHqXLHudqbTQQa6KnvfbM2Ria6riace9QG1zPYAcKc12vS4EicdspqvoxNYs8zKX8EfERXEoEcLdwLZ&hy=SH&idx=1&m=704de7ebbc107a51a4f0986253a6d3b6&token=cztXnd9GyrEsWrS4eJynZhicYicwhU5cChkbUOWNwn6llc25ba051o3j5lhJUGZgv4nzSxYfuDf7q3Xiat145wgtQ
                      mp4Identify: ed39cc64d1dbe68dbc4e43127f2bbd37
                      fileSize: 1315979
                      thumbMD5: 704de7ebbc107a51a4f0986253a6d3b6
                      fileKey: '-finder_upload_7212269489_wxid_0xsqb3o0tsvz22'
                '2':
                  summary: 异常示例
                  value:
                    ret: 500
                    msg: 发布视频失败
                    data:
                      code: '-4013'
                      msg: null
          headers: {}
          x-apifox-name: 成功
        '500':
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
                      code:
                        type: string
                      msg:
                        type: 'null'
                    required:
                      - code
                      - msg
                required:
                  - ret
                  - msg
                  - data
          headers: {}
          x-apifox-name: 服务器错误
      security: []
      x-apifox-folder: 基础API/视频号模块
      x-apifox-status: released
      x-run-in-apifox: https://app.apifox.com/web/project/3475559/apis/api-144561904-run
components:
  schemas: {}
  securitySchemes: {}
servers:
  - url: http://api.geweapi.com
    description: 测试环境
security: []

```
