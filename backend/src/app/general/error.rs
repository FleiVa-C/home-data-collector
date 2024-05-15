use std::panic;

use actix_web::{
    error::{self, PayloadError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use surrealdb::error::Api;
use surrealdb::Error;

use super::model::BasicErrorMessage;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum BackendError {
    SomethingWentWrong(String),
    NotFound,
    EmptyResponse(String),
    AlreadyExists(String),
    RegisterFailure(String),
    Overflow(String),
    PayloadError,
    ParseError,
    MalformedQuerry(String),
}

impl ResponseError for BackendError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&BasicErrorMessage::new(
                    self.status_code(),
                    self.to_string(),
                ))
                .unwrap(),
            )
    }

    fn status_code(&self) -> StatusCode {
        match self {
            BackendError::SomethingWentWrong(_) => StatusCode::BAD_REQUEST,
            BackendError::NotFound => StatusCode::NOT_FOUND,
            BackendError::EmptyResponse(_) => StatusCode::NOT_FOUND,
            BackendError::AlreadyExists(_) => StatusCode::BAD_REQUEST,
            BackendError::Overflow(_) => StatusCode::BAD_REQUEST,
            BackendError::RegisterFailure(_) => StatusCode::BAD_REQUEST,
            BackendError::PayloadError => StatusCode::BAD_REQUEST,
            BackendError::ParseError => StatusCode::BAD_REQUEST,
            BackendError::MalformedQuerry(_) => StatusCode::BAD_REQUEST,
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

pub fn unpack_surrealdb_error(err: Error) -> Option<Api> {
    match err {
        Error::Api(error_type) => Some(error_type),
        Error::Db(_) => None,
    }
}

#[test]
fn access_surrealdb_error() {
    let response = Error::Api(Api::Query("No Interfaces found".to_string()));
    let a: Option<Api> = unpack_surrealdb_error(response);
    match a.unwrap() {
        Api::Query(_) => (),
        _ => panic!("mismatched types"),
    }
}
