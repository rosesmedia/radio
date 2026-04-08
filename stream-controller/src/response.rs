use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    NotFound,
    Internal,
    UnprocessableEntity,
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::NotFound => "not found",
            Self::Internal => "internal server error",
            Self::UnprocessableEntity => "unprocessable entity",
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
