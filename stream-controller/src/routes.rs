use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::Serialize;

use crate::{AppState, auth};

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResult {
    Success { ok: bool },
    Error { message: String },
}

impl From<miette::Result<()>> for ApiResult {
    fn from(value: miette::Result<()>) -> Self {
        match value {
            Ok(()) => ApiResult::Success { ok: true },
            Err(e) => {
                tracing::error!(?e, "failed to start stream");
                ApiResult::Error {
                    message: format!("{e}"),
                }
            }
        }
    }
}

impl IntoResponse for ApiResult {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Success { .. } => Json(self).into_response(),
            Self::Error { .. } => (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response(),
        }
    }
}

#[tracing::instrument(skip(state))]
async fn start_streamer(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult {
    state.service_manager.start_streamer(&id).await.into()
}

#[tracing::instrument(skip(state))]
async fn stop_streamer(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult {
    state.service_manager.stop_streamer(&id).await.into()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/stream/{id}/start", post(start_streamer))
        .route("/stream/{id}/stop", post(stop_streamer))
        .layer(axum::middleware::from_fn(auth::auth_middleware))
}
