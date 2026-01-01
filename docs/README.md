# gewe-rs

GeWe API Rust SDK 项目

## 📁 项目结构

```
gewe-rs/
├── .claude/
│   └── skills/
│       └── gewe-api/              # GeWe API Claude Skill
│           ├── skill.md           # Skill 主文件
│           └── REFERENCE.md       # API 快速参考
│
└── docs/GeweAPI-Official          # GeWe API 完整文档库（141个文档）
    ├── index.json                 # 文档索引
    ├── 开发指南/
    ├── 登录模块/
    ├── 联系人模块/
    ├── 群模块/
    ├── 消息模块/
    ├── 朋友圈模块/
    ├── 标签模块/
    ├── 个人资料模块/
    ├── 收藏夹模块/
    ├── 视频号模块/
    ├── 系统功能/
    └── 账号规范/
```

## 🚀 Claude Skill

本项目包含一个 **GeWe API Claude Skill**，可以帮助你：

- 🔍 快速查找 GeWe API 文档
- 📖 理解 API 参数和返回值
- 💻 根据你的项目自动生成对应语言的代码（Rust/Python/JS/Go等）
- ⚠️ 获取最佳实践和安全建议

### 使用方法

在 Claude Code 中，Skill 会自动激活。你只需要询问相关问题：

```
"如何发送文字消息？"
"怎么获取群成员列表？"
"发朋友圈需要哪些参数？"
```

Claude 会：
1. 自动查找相关文档
2. 提取 API 信息
3. 根据你的项目语言生成代码
4. 提供完整的使用说明

## 📚 文档库

`docs/` 目录包含从 https://doc.geweapi.com/ 下载的完整 API 文档：

- **文档数量**: 141 个
- **文档格式**: Markdown
- **包含内容**: API 说明、参数、返回值、示例
- **索引文件**: `docs/index.json` （可程序化访问）

## 🔗 相关链接

- 官方网站: https://www.geweapi.com/
- 在线文档: https://doc.geweapi.com/
- 后台系统: http://manager.geweapi.com/

## ⚠️ 免责声明

严禁将 GeWe API 用于非法用途。使用本服务请遵守微信运营规范和国家法律法规。
