use std::sync::LazyLock;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    AppState, auth,
    liquidsoap::{LiquidsoapClient, Source},
    response::ApiError,
};

static STREAM_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[a-z0-9_.-]+$").unwrap());

#[tracing::instrument(skip(state))]
async fn start_streamer(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    if !STREAM_NAME_REGEX.is_match(&id) {
        return Err(ApiError::UnprocessableEntity);
    }

    state.service_manager.start_streamer(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(skip(state))]
async fn stop_streamer(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    if !STREAM_NAME_REGEX.is_match(&id) {
        return Err(ApiError::UnprocessableEntity);
    }

    state.service_manager.stop_streamer(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Clone, Deserialize)]
struct SourceData {
    source: Source,
}

#[tracing::instrument]
async fn set_stream_source(
    Path(id): Path<String>,
    Json(data): Json<SourceData>,
) -> Result<impl IntoResponse, ApiError> {
    if !STREAM_NAME_REGEX.is_match(&id) {
        return Err(ApiError::UnprocessableEntity);
    }

    let client = LiquidsoapClient::new(&id);
    let mut connection = client
        .create_connection()
        .await
        .map_err(|_| ApiError::NotFound)?;
    connection.set_source(&data.source).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument]
async fn get_stream_source(Path(id): Path<String>) -> Result<Json<impl Serialize>, ApiError> {
    #[derive(Serialize)]
    struct Response {
        source: Source,
    }

    if !STREAM_NAME_REGEX.is_match(&id) {
        return Err(ApiError::UnprocessableEntity);
    }

    let client = LiquidsoapClient::new(&id);
    let mut connection = client
        .create_connection()
        .await
        .map_err(|_| ApiError::NotFound)?;

    let source = connection.get_source().await?;

    Ok(Json(Response { source }))
}

#[tracing::instrument]
async fn get_sources() -> Json<Vec<String>> {
    Json(
        Source::sources()
            .iter()
            .map(|source| source.identifier())
            .collect(),
    )
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/stream/{id}/start", post(start_streamer))
        .route("/stream/{id}/stop", post(stop_streamer))
        .route("/stream/{id}/source", post(set_stream_source))
        .route("/stream/{id}/source", get(get_stream_source))
        .layer(axum::middleware::from_fn(auth::auth_middleware))
        .route("/sources", get(get_sources))
}
