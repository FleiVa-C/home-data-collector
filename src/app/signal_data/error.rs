use actix_web::{
    error::ResponseError,
    HttpResponse,
    http::{StatusCode, header::ContentType}};
use derive_more::Display;

use crate::app::signal_data::model::IngestionPacket;

impl ResponseError for IngestionPacket {
    fn error_response(&self) -> HttpResponse {
       HttpResponse::build(StatusCode::ACCEPTED) 
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Display)]
pub enum QueryError{
    Failed
}

impl ResponseError for QueryError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            QueryError::Failed => StatusCode::NOT_FOUND
        }
    }
}
