use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not found")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] eyre::Report),
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}
