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
