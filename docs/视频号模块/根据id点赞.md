# 根据id点赞

## OpenAPI Specification

```yaml
openapi: 3.0.1
info:
  title: ''
  description: ''
  version: 1.0.0
paths:
  /gewe/v2/api/finder/idFav:
    post:
      summary: 根据id点赞
      deprecated: false
      description: ''
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
                myUserName:
                  type: string
                  description: 自己的username
                opType:
                  type: integer
                  description: 1点赞 2取消点赞
                objectNonceId:
                  type: string
                  description: 视频号的objectNonceId
                sessionBuffer:
                  type: string
                  description: 视频号的sessionBuffer
                objectId:
                  type: integer
                  description: 视频号的ID
                toUserName:
                  type: string
                  description: 视频所有者userName
                myRoleType:
                  type: integer
                  description: 自己的roletype
              required:
                - appId
                - myUserName
                - opType
                - objectNonceId
                - sessionBuffer
                - objectId
                - toUserName
                - myRoleType
              x-apifox-orders:
                - appId
                - objectId
                - sessionBuffer
                - objectNonceId
                - opType
                - myUserName
                - myRoleType
                - toUserName
            example:
              appId: ''
              proxyIp: ''
              myUserName: ''
              opType: 1
              objectNonceId: '8507486792812551167_0_0_2_2_1719545315208098'
              sessionBuffer: >-
                eyJyZWNhbGxfdHlwZXMiOltdLCJkZWxpdmVyeV9zY2VuZSI6MiwiZGVsaXZlcnlfdGltZSI6MTcxOTU0NTMxNSwic2V0X2NvbmRpdGlvbl9mbGFnIjo5LCJyZWNhbGxfaW5kZXgiOltdLCJyZXF1ZXN0X2lkIjoxNzE5NTQ1MzE1MjA4MDk4LCJtZWRpYV90eXBlIjo0LCJ2aWRfbGVuIjoyLCJjcmVhdGVfdGltZSI6MTcxODMzNDg5MywicmVjYWxsX2luZm8iOltdLCJvZmxhZyI6MTY4MTgxOTIsImlkYyI6MywiZGV2aWNlX3R5cGVfaWQiOjEzLCJkZXZpY2VfcGxhdGZvcm0iOiJpUGFkMTEsMyIsImZlZWRfcG9zIjowLCJjbGllbnRfcmVwb3J0X2J1ZmYiOiJ7XCJpZl9zcGxpdF9zY3JlZW5faXBhZFwiOjAsXCJlbnRlclNvdXJjZUluZm9cIjpcIntcXFwiZmluZGVydXNlcm5hbWVcXFwiOlxcXCJcXFwiLFxcXCJmZWVkaWRcXFwiOlxcXCJcXFwifVwiLFwiZXh0cmFpbmZvXCI6XCJ7XFxcInJlZ2NvdW50cnlcXFwiOlxcXCJDTlxcXCJ9XCIsXCJzZXNzaW9uSWRcIjpcIlNwbGl0Vmlld0VtcHR5Vmlld0NvbnRyb2xsZXJfMTcxOTU0NTMwNjU5NiMkMF8xNzE5NTQ1MjkzOTYwI1wiLFwianVtcElkXCI6e1widHJhY2VpZFwiOlwiXCIsXCJzb3VyY2VpZFwiOlwiXCJ9fSIsIm9iamVjdF9pZCI6MTQ0MTQ0Mzc4MzUwMTE1MjkwMDcsImZpbmRlcl91aW4iOjEzMTA0ODA1MzY5MjE2NzMyLCJnZW9oYXNoIjozMzc3Njk5NzIwNTI3ODcyLCJlbnRyYW5jZV9zY2VuZSI6MiwiY2FyZF90eXBlIjozLCJleHB0X2ZsYWciOjg4Nzg3OTU1LCJ1c2VyX21vZGVsX2ZsYWciOjgsImlzX2ZyaWVuZCI6dHJ1ZSwiY3R4X2lkIjoiMi0zLTMyLWYxNjU5NWU2YjhlYmVjZjVhNDRhZGMzZWY1NGQzYzdhMTcxOTU0NTMxMTcyMiIsImFkX2ZsYWciOjQsImVyaWwiOltdLCJwZ2tleXMiOltdLCJzY2lkIjoiODA2MmY0NTQtMzRmZS0xMWVmLTkxOWUtZGYyYjg4ZGI2N2M5IiwiY29tbWVudF92ZXIiOjE3MTk0ODE2NDJ9
              objectId: 14414437835011529000
              toUserName: >-
                v2_060000231003b20faec8c7e08f10c1d4c803ef36b077bc0b9fb41ae2efc82c20ba5fb68f838a@finder
              myRoleType: 3
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
                required:
                  - ret
                  - msg
              example:
                ret: 200
                msg: 操作成功
          headers: {}
          x-apifox-name: 成功
      security: []
      x-apifox-folder: 基础API/视频号模块
      x-apifox-status: released
      x-run-in-apifox: https://app.apifox.com/web/project/3475559/apis/api-189454588-run
components:
  schemas: {}
  securitySchemes: {}
servers:
  - url: http://api.geweapi.com
    description: 测试环境
security: []

```
