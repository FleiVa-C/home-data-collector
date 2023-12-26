use super::model::MultiStatusData;
use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use hdc_shared::models::ingestion_container::*;

impl ResponseError for MultiStatusData {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::MULTI_STATUS)
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Display)]
pub enum QueryError {
    Failed,
}

impl ResponseError for QueryError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            QueryError::Failed => StatusCode::NOT_FOUND,
        }
    }
}
