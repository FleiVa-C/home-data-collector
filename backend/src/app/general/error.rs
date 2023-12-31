use actix_web::{
    error::{self, PayloadError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use super::model::BasicErrorMessage;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum BackendError {
    NotFound,
    AlreadyExists(String),
    RegisterFailure(String),
    Overflow(String),
    PayloadError,
    ParseError,
}

impl ResponseError for BackendError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&BasicErrorMessage::init(
                    self.status_code(),
                    self.to_string(),
                ))
                .unwrap(),
            )
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BackendError::NotFound => StatusCode::NOT_FOUND,
            BackendError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            BackendError::Overflow(_) => StatusCode::BAD_REQUEST,
            BackendError::RegisterFailure(_) => StatusCode::FAILED_DEPENDENCY,
            BackendError::PayloadError => StatusCode::BAD_REQUEST,
            BackendError::ParseError => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<PayloadError> for BackendError {
    fn from(e: PayloadError) -> Self {
        BackendError::PayloadError
    }
}

impl From<serde_json::Error> for BackendError {
    fn from(e: serde_json::Error) -> Self {
        BackendError::ParseError
    }
}
