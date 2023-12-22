use actix_web::{
    error::{self, PayloadError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use super::model::DefaultErrorResponse;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum DefaultError {
    NotFound,
    AlreadyExists(String),
    RegisterFailure(String),
    Overflow(String),
    PayloadError,
    ParseError,
}

impl ResponseError for DefaultError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&DefaultErrorResponse::init(
                    self.status_code(),
                    self.to_string(),
                ))
                .unwrap(),
            )
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DefaultError::NotFound => StatusCode::NOT_FOUND,
            DefaultError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            DefaultError::Overflow(_) => StatusCode::BAD_REQUEST,
            DefaultError::RegisterFailure(_) => StatusCode::FAILED_DEPENDENCY,
            DefaultError::PayloadError => StatusCode::BAD_REQUEST,
            DefaultError::ParseError => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<PayloadError> for DefaultError {
    fn from(e: PayloadError) -> Self {
        DefaultError::PayloadError
    }
}

impl From<serde_json::Error> for DefaultError {
    fn from(e: serde_json::Error) -> Self {
        DefaultError::ParseError
    }
}
