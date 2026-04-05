use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};

use crate::{
    AppState, auth,
    liquidsoap::{LiquidsoapClient, Source},
};

enum ApiError {
    NotFound,
    Internal,
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::NotFound => "not found",
            Self::Internal => "internal server error",
        }
        .to_string()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status_code(), self.message()).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<miette::Report>,
{
    fn from(err: E) -> Self {
        tracing::error!("{:?}", err.into());
        ApiError::Internal
    }
}

#[tracing::instrument(skip(state))]
async fn start_streamer(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state.service_manager.start_streamer(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument(skip(state))]
async fn stop_streamer(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state.service_manager.stop_streamer(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[tracing::instrument]
async fn switch_stream_source(
    Path((id, source)): Path<(String, u8)>,
) -> Result<StatusCode, ApiError> {
    let source = Source::from_id(source);
    let Some(source) = source else {
        return Err(ApiError::NotFound);
    };

    let client = LiquidsoapClient::new(&id);
    let mut connection = client
        .create_connection()
        .await
        .map_err(|_| ApiError::NotFound)?;
    connection.set_source(&source).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/stream/{id}/start", post(start_streamer))
        .route("/stream/{id}/stop", post(stop_streamer))
        .route("/stream/{id}/switch/{source}", post(switch_stream_source))
        .layer(axum::middleware::from_fn(auth::auth_middleware))
}
