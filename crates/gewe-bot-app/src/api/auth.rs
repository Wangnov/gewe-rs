//! 鉴权中间件

use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// 简单 Token 鉴权中间件
pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 从环境变量读取 Token
    let expected_token = std::env::var("GEWE_API_TOKEN").ok();

    // 如果未设置 GEWE_API_TOKEN，跳过鉴权
    if expected_token.is_none() {
        return Ok(next.run(request).await);
    }

    // 检查 Authorization 头
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(auth) => {
            // 支持 Bearer Token
            if let Some(stripped) = auth.strip_prefix("Bearer ") {
                stripped
            } else {
                auth
            }
        }
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // 验证 Token
    if Some(token.to_string()) == expected_token {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Basic Auth 中间件（用户名/密码）
pub async fn basic_auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 从环境变量读取用户名和密码
    let expected_username = std::env::var("GEWE_API_USERNAME").ok();
    let expected_password = std::env::var("GEWE_API_PASSWORD").ok();

    // 如果未设置，跳过鉴权
    if expected_username.is_none() || expected_password.is_none() {
        return Ok(next.run(request).await);
    }

    // 解析 Authorization 头
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let (username, password) = match auth_header {
        Some(auth) if auth.starts_with("Basic ") => {
            // 解码 Base64
            let encoded = &auth[6..];
            use base64::Engine;
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            let credentials = String::from_utf8(decoded).map_err(|_| StatusCode::BAD_REQUEST)?;

            // 解析 username:password
            let parts: Vec<&str> = credentials.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(StatusCode::BAD_REQUEST);
            }
            (parts[0].to_string(), parts[1].to_string())
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // 验证用户名和密码
    if Some(username) == expected_username && Some(password) == expected_password {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[cfg(test)]
mod tests {
    use base64::Engine;

    #[test]
    fn test_token_validation_logic() {
        let auth_header = "Bearer test_token_12345";
        assert!(auth_header.starts_with("Bearer "));
        let token = auth_header.strip_prefix("Bearer ").unwrap();
        assert_eq!(token, "test_token_12345");
    }

    #[test]
    fn test_basic_auth_parsing() {
        let username = "testuser";
        let password = "testpass";
        let credentials = format!("{}:{}", username, password);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);

        let decoded_bytes = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        let decoded = String::from_utf8(decoded_bytes).unwrap();
        assert_eq!(decoded, "testuser:testpass");
    }

    #[test]
    fn test_basic_auth_with_colon_in_password() {
        let username = "testuser";
        let password = "test:pass:word";
        let credentials = format!("{}:{}", username, password);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);

        let decoded_bytes = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        let decoded = String::from_utf8(decoded_bytes).unwrap();

        let parts: Vec<&str> = decoded.splitn(2, ':').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "testuser");
        assert_eq!(parts[1], "test:pass:word");
    }
}
