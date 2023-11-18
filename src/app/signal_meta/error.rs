use actix_web::{
    error::{ResponseError, PayloadError, self},
    HttpResponse,
    http::{StatusCode, header::ContentType}};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum SignalError{
    SignalNotFound,
    SignalRegisterFailure,
    Overflow,
    PayloadError,
    ParseError,
}

impl ResponseError for SignalError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SignalError::SignalNotFound => StatusCode::NOT_FOUND,
            SignalError::SignalRegisterFailure => StatusCode::FAILED_DEPENDENCY,
            SignalError::Overflow => StatusCode::BAD_REQUEST,
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
