//! Web 应用占位，提供纯 Web 版本的管理面板。
//!
//! 与 gewe-tauri 共享前端代码，通过 HTTP API 与后端通信。
//! 适合部署到云服务器或 CDN。

pub fn placeholder() -> &'static str {
    "gewe-web placeholder"
}
