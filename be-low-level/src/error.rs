use actix_web::HttpResponse;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<AppError> for HttpResponse {
    fn from(err: AppError) -> Self {
        let message = err.to_string();
        let body = ErrorResponse { error: message };
        match err {
            AppError::Unauthorized(_) => HttpResponse::Unauthorized().json(body),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(body),
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let body = ErrorResponse { error: self.to_string() };
        match self {
            AppError::Unauthorized(_) => HttpResponse::Unauthorized().json(body),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(body),
        }
    }
}
