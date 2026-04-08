use std::sync::LazyLock;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub static STREAM_API_TOKEN: LazyLock<String> = LazyLock::new(|| {
    std::env::var("STREAM_API_TOKEN").expect("`STREAM_API_TOKEN` environment variable to be set")
});

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    let Some(authorization) = request.headers().get("Authorization") else {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    };
    let Ok(authorization) = authorization.to_str() else {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    };

    if authorization != format!("Bearer {}", *STREAM_API_TOKEN) {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    }

    next.run(request).await
}
