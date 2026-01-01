pub mod client;
pub mod contact;
pub mod favorite;
pub mod group;
pub mod login;
pub mod message;
pub mod moments;
pub mod personal;
pub mod tag;
pub mod video_account;

pub use client::GeweHttpClient;

#[cfg(test)]
mod tests {
    use super::*;
    use gewe_core::{ApiEnvelope, GeweError};
    use serde::{Deserialize, Serialize};

    // 测试用的简单结构体
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestRequest<'a> {
        #[serde(rename = "appId")]
        app_id: &'a str,
        #[serde(rename = "userId")]
        user_id: Option<&'a str>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestResponse {
        message: String,
        count: i32,
    }

    mod client_tests {
        use super::*;

        #[test]
        fn test_client_new() {
            // 测试创建客户端
            let result = GeweHttpClient::new("test_token_123", "https://api.example.com");
            assert!(result.is_ok());

            let client = result.unwrap();
            assert_eq!(client.base_url, "https://api.example.com");
        }

        #[test]
        fn test_client_new_with_trailing_slash() {
            // 测试带尾部斜杠的 URL
            let result = GeweHttpClient::new("test_token", "https://api.example.com/");
            assert!(result.is_ok());

            let client = result.unwrap();
            assert_eq!(client.base_url, "https://api.example.com/");
        }

        #[test]
        fn test_client_new_with_invalid_token() {
            // 测试无效的 token（包含非 ASCII 换行符）
            let result = GeweHttpClient::new("invalid\ntoken", "https://api.example.com");

            // 确保返回错误且是 Http 错误
            assert!(result.is_err());
            assert!(matches!(result, Err(GeweError::Http(_))));
        }

        #[test]
        fn test_endpoint_generation() {
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");

            // 测试基本路径
            assert_eq!(
                client.endpoint("gewe/v2/api/login/getLoginQrCode"),
                "https://api.example.com/gewe/v2/api/login/getLoginQrCode"
            );

            // 测试带前导斜杠的路径
            assert_eq!(
                client.endpoint("/gewe/v2/api/login/getLoginQrCode"),
                "https://api.example.com/gewe/v2/api/login/getLoginQrCode"
            );

            // 测试带尾部斜杠的 base_url
            let client2 = GeweHttpClient::new("token", "https://api.example.com/")
                .expect("Failed to create client");
            assert_eq!(
                client2.endpoint("gewe/v2/api/login/getLoginQrCode"),
                "https://api.example.com/gewe/v2/api/login/getLoginQrCode"
            );

            // 测试两者都有斜杠
            assert_eq!(
                client2.endpoint("/gewe/v2/api/login/getLoginQrCode"),
                "https://api.example.com/gewe/v2/api/login/getLoginQrCode"
            );
        }

        #[test]
        fn test_endpoint_with_empty_path() {
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");
            assert_eq!(client.endpoint(""), "https://api.example.com/");
        }

        #[test]
        fn test_client_is_clone() {
            // 测试客户端可以被克隆
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");
            let client2 = client.clone();

            assert_eq!(client.base_url, client2.base_url);
        }
    }

    mod serialization_tests {
        use super::*;

        #[test]
        fn test_request_serialization() {
            // 测试请求结构体的序列化
            let req = TestRequest {
                app_id: "app123",
                user_id: Some("user456"),
            };

            let json = serde_json::to_string(&req).expect("Failed to serialize");
            assert!(json.contains("appId"));
            assert!(json.contains("app123"));
            assert!(json.contains("userId"));
            assert!(json.contains("user456"));
        }

        #[test]
        fn test_request_serialization_with_none() {
            // 测试带 None 的请求序列化
            let req = TestRequest {
                app_id: "app123",
                user_id: None,
            };

            let json = serde_json::to_string(&req).expect("Failed to serialize");
            assert!(json.contains("appId"));
            assert!(json.contains("app123"));
            // Option::None 会被序列化为 null
            assert!(json.contains("null"));
        }

        #[test]
        fn test_response_deserialization() {
            // 测试响应结构体的反序列化
            let json = r#"{"message":"success","count":42}"#;
            let resp: TestResponse = serde_json::from_str(json).expect("Failed to deserialize");

            assert_eq!(resp.message, "success");
            assert_eq!(resp.count, 42);
        }

        #[test]
        fn test_api_envelope_deserialization_success() {
            // 测试成功响应的反序列化
            let json = r#"{
                "ret": 200,
                "msg": "success",
                "data": {
                    "message": "hello",
                    "count": 10
                }
            }"#;

            let env: ApiEnvelope<TestResponse> =
                serde_json::from_str(json).expect("Failed to deserialize");

            assert_eq!(env.ret, 200);
            assert_eq!(env.msg, "success");
            assert!(env.data.is_some());

            let data = env.data.unwrap();
            assert_eq!(data.message, "hello");
            assert_eq!(data.count, 10);
        }

        #[test]
        fn test_api_envelope_deserialization_no_data() {
            // 测试没有 data 字段的响应
            let json = r#"{
                "ret": 200,
                "msg": "success"
            }"#;

            let env: ApiEnvelope<TestResponse> =
                serde_json::from_str(json).expect("Failed to deserialize");

            assert_eq!(env.ret, 200);
            assert_eq!(env.msg, "success");
            assert!(env.data.is_none());
        }

        #[test]
        fn test_api_envelope_deserialization_error() {
            // 测试错误响应的反序列化
            let json = r#"{
                "ret": 400,
                "msg": "invalid request"
            }"#;

            let env: ApiEnvelope<TestResponse> =
                serde_json::from_str(json).expect("Failed to deserialize");

            assert_eq!(env.ret, 400);
            assert_eq!(env.msg, "invalid request");
            assert!(env.data.is_none());
        }
    }

    mod error_tests {
        use super::*;

        #[test]
        fn test_gewe_error_http() {
            // 测试 HTTP 错误
            let err = GeweError::Http("connection failed".to_string());
            assert!(err.to_string().contains("http error"));
            assert!(err.to_string().contains("connection failed"));
        }

        #[test]
        fn test_gewe_error_api() {
            // 测试 API 错误
            let err = GeweError::Api {
                code: 401,
                message: "unauthorized".to_string(),
            };
            assert!(err.to_string().contains("api error"));
            assert!(err.to_string().contains("401"));
            assert!(err.to_string().contains("unauthorized"));
        }

        #[test]
        fn test_gewe_error_decode() {
            // 测试解码错误
            let err = GeweError::Decode("invalid json".to_string());
            assert!(err.to_string().contains("decode error"));
            assert!(err.to_string().contains("invalid json"));
        }

        #[test]
        fn test_gewe_error_missing_data() {
            // 测试缺失数据错误
            let err = GeweError::MissingData;
            assert!(err.to_string().contains("missing data"));
        }
    }

    mod url_building_tests {
        use super::*;

        #[test]
        fn test_various_base_url_formats() {
            // 测试各种 base_url 格式
            let test_cases = vec![
                (
                    "https://api.example.com",
                    "api/test",
                    "https://api.example.com/api/test",
                ),
                (
                    "https://api.example.com/",
                    "api/test",
                    "https://api.example.com/api/test",
                ),
                (
                    "https://api.example.com",
                    "/api/test",
                    "https://api.example.com/api/test",
                ),
                (
                    "https://api.example.com/",
                    "/api/test",
                    "https://api.example.com/api/test",
                ),
                (
                    "https://api.example.com/v1",
                    "login/check",
                    "https://api.example.com/v1/login/check",
                ),
                (
                    "https://api.example.com/v1/",
                    "/login/check",
                    "https://api.example.com/v1/login/check",
                ),
            ];

            for (base_url, path, expected) in test_cases {
                let client =
                    GeweHttpClient::new("token", base_url).expect("Failed to create client");
                let result = client.endpoint(path);
                assert_eq!(
                    result, expected,
                    "Failed for base_url='{}', path='{}'",
                    base_url, path
                );
            }
        }

        #[test]
        fn test_url_with_port() {
            // 测试带端口号的 URL
            let client = GeweHttpClient::new("token", "https://api.example.com:8080")
                .expect("Failed to create client");
            assert_eq!(
                client.endpoint("gewe/api/test"),
                "https://api.example.com:8080/gewe/api/test"
            );
        }

        #[test]
        fn test_url_with_path_prefix() {
            // 测试带路径前缀的 base_url
            let client = GeweHttpClient::new("token", "https://api.example.com/api/v2")
                .expect("Failed to create client");
            assert_eq!(
                client.endpoint("login/check"),
                "https://api.example.com/api/v2/login/check"
            );
        }
    }

    mod integration_mock_tests {
        use super::*;

        // 由于实际的 HTTP 请求需要真实服务器，这里我们主要测试请求构建逻辑
        // 在实际项目中，可以使用 mockito 或 wiremock 等库来模拟 HTTP 服务器

        #[test]
        fn test_client_timeout_configuration() {
            // 测试客户端超时配置
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");

            // 虽然我们不能直接访问 timeout 设置，但可以确保客户端创建成功
            // 在实际使用中，超时设置会在 HTTP 请求时生效
            assert_eq!(client.base_url, "https://api.example.com");
        }

        #[test]
        fn test_multiple_clients() {
            // 测试创建多个客户端实例
            let client1 = GeweHttpClient::new("token1", "https://api1.example.com")
                .expect("Failed to create client");
            let client2 = GeweHttpClient::new("token2", "https://api2.example.com")
                .expect("Failed to create client");

            assert_eq!(client1.base_url, "https://api1.example.com");
            assert_eq!(client2.base_url, "https://api2.example.com");
        }
    }

    mod edge_case_tests {
        use super::*;

        #[test]
        fn test_empty_token() {
            // 测试空 token
            let result = GeweHttpClient::new("", "https://api.example.com");
            // 空 token 是有效的，只是可能在 API 调用时失败
            assert!(result.is_ok());
        }

        #[test]
        fn test_long_token() {
            // 测试长 token
            let long_token = "a".repeat(1000);
            let result = GeweHttpClient::new(&long_token, "https://api.example.com");
            assert!(result.is_ok());
        }

        #[test]
        fn test_special_characters_in_base_url() {
            // 测试 base_url 中的特殊字符（URL 编码）
            let client =
                GeweHttpClient::new("token", "https://api.example.com/path%20with%20spaces")
                    .expect("Failed to create client");
            assert_eq!(
                client.base_url,
                "https://api.example.com/path%20with%20spaces"
            );
        }

        #[test]
        fn test_unicode_in_data() {
            // 测试 Unicode 字符的序列化
            let req = TestRequest {
                app_id: "测试应用",
                user_id: Some("用户123"),
            };

            let json = serde_json::to_string(&req).expect("Failed to serialize");
            assert!(json.contains("测试应用"));
            assert!(json.contains("用户123"));
        }

        #[test]
        fn test_api_envelope_with_null_data() {
            // 测试 data 为 null 的情况
            let json = r#"{
                "ret": 200,
                "msg": "success",
                "data": null
            }"#;

            let env: ApiEnvelope<TestResponse> =
                serde_json::from_str(json).expect("Failed to deserialize");

            assert_eq!(env.ret, 200);
            assert!(env.data.is_none());
        }
    }

    mod path_handling_tests {
        use super::*;

        #[test]
        fn test_path_with_query_string() {
            // 测试带查询字符串的路径（虽然通常使用 POST，但测试边界情况）
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");
            let endpoint = client.endpoint("api/test?param=value");
            assert_eq!(endpoint, "https://api.example.com/api/test?param=value");
        }

        #[test]
        fn test_path_with_fragment() {
            // 测试带片段标识符的路径
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");
            let endpoint = client.endpoint("api/test#section");
            assert_eq!(endpoint, "https://api.example.com/api/test#section");
        }

        #[test]
        fn test_deeply_nested_path() {
            // 测试深层嵌套的路径
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");
            let endpoint = client.endpoint("a/b/c/d/e/f/g/h");
            assert_eq!(endpoint, "https://api.example.com/a/b/c/d/e/f/g/h");
        }

        #[test]
        fn test_path_with_multiple_slashes() {
            // 测试包含多个斜杠的路径
            let client = GeweHttpClient::new("token", "https://api.example.com")
                .expect("Failed to create client");
            // 注意：endpoint 方法只处理前导斜杠，不会清理中间的多余斜杠
            let endpoint = client.endpoint("api//test///path");
            assert_eq!(endpoint, "https://api.example.com/api//test///path");
        }
    }
}
