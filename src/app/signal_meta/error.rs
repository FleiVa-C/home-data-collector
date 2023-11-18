use actix_web::{
    error::{ResponseError, PayloadError, self},
    HttpResponse,
    http::{StatusCode, header::ContentType}};
use derive_more::Display;

use crate::app::general::model::DefaultErrorResponse;

#[derive(Debug, Display)]
pub enum SignalError{
    SignalNotFound,
    SignalAlreadyExists(String),
    SignalRegisterFailure(String),
    Overflow(String),
    PayloadError,
    ParseError,
}

impl ResponseError for SignalError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&DefaultErrorResponse::init(self.status_code(),
            self.to_string())).unwrap())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SignalError::SignalNotFound => StatusCode::NOT_FOUND,
            SignalError::SignalAlreadyExists(_) => StatusCode::BAD_REQUEST,
            SignalError::Overflow(_) => StatusCode::BAD_REQUEST,
            SignalError::SignalRegisterFailure(_) => StatusCode::FAILED_DEPENDENCY,
            SignalError::PayloadError => StatusCode::BAD_REQUEST,
            SignalError::ParseError => StatusCode::BAD_REQUEST
        }
    }
}

impl From<PayloadError> for SignalError{
    fn from(e: PayloadError) -> Self{
        SignalError::PayloadError
    }
}

impl From<serde_json::Error> for SignalError{
    fn from(e: serde_json::Error) -> Self{
        SignalError::ParseError
    }
}
