//! Web 应用占位，提供纯 Web 版本的管理面板。
//!
//! 与 gewe-tauri 共享前端代码，通过 HTTP API 与后端通信。
//! 适合部署到云服务器或 CDN。

pub fn placeholder() -> &'static str {
    "gewe-web placeholder"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        let result = placeholder();
        assert_eq!(result, "gewe-web placeholder");
    }

    #[test]
    fn test_placeholder_is_static_str() {
        let result = placeholder();
        assert!(!result.is_empty());
        assert!(result.len() > 0);
    }

    #[test]
    fn test_placeholder_content() {
        let result = placeholder();
        assert!(result.contains("gewe-web"));
        assert!(result.contains("placeholder"));
    }

    #[test]
    fn test_placeholder_consistent() {
        // 多次调用应返回相同的结果
        let result1 = placeholder();
        let result2 = placeholder();
        assert_eq!(result1, result2);
        assert_eq!(result1.as_ptr(), result2.as_ptr()); // 确保是同一个静态字符串
    }
}
