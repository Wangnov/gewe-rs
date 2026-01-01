//! Tauri 桌面端占位,后续接入 tauri 2.x。

pub fn placeholder() -> &'static str {
    "gewe-tauri placeholder"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_returns_correct_string() {
        let result = placeholder();
        assert_eq!(result, "gewe-tauri placeholder");
    }

    #[test]
    fn test_placeholder_returns_static_str() {
        let result = placeholder();
        assert!(!result.is_empty());
        assert!(result.contains("gewe-tauri"));
        assert!(result.contains("placeholder"));
    }

    #[test]
    fn test_placeholder_is_consistent() {
        // 确保多次调用返回相同的结果
        let first_call = placeholder();
        let second_call = placeholder();
        assert_eq!(first_call, second_call);
    }

    #[test]
    fn test_placeholder_is_valid_static_lifetime() {
        // 测试返回的字符串切片具有 'static 生命周期
        let result: &'static str = placeholder();
        let _leaked: &'static str = result; // 应该能够赋值给 'static 变量
        assert!(true);
    }
}
