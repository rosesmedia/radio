use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use reqwest::StatusCode;
use serde::Serialize;

use crate::AppState;

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
                tracing::error!(?e, "failed to start ingest");
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
async fn ingest_created(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult {
    state.service_manager.start_ingest(&id).await.into()
}

#[tracing::instrument(skip(state))]
async fn ingest_updated(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult {
    state.service_manager.restart_ingest(&id).await.into()
}

#[tracing::instrument(skip(state))]
async fn ingest_deleted(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult {
    state.service_manager.stop_ingest(&id).await.into()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ingest/{id}/create", post(ingest_created))
        .route("/ingest/{id}/update", post(ingest_updated))
        .route("/ingest/{id}/delete", post(ingest_deleted))
}
