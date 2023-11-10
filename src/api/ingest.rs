use actix_web::{
    Error,
    get,
    post,
    put,
    error::{ResponseError, PayloadError, self},
    web::Path,
    web::Json,
    web::Data,
    web,
    HttpResponse,
    http::{header::ContentType, StatusCode}};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use derive_more::Display;
use crate::{model::signal::Signal, repository::sdb::IngestionResponse};
use crate::model::ingest_packet::{IngestionPacket, DataPoint};
use crate::repository::sdb::{SDBRepository, SDBError};
use chrono::{DateTime, Utc};

#[derive(Debug, Display)]
pub enum IngestionError{
    PartialIngestion(IngestionPacket),
    DefaultError
}

impl ResponseError for IngestionError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            IngestionError::PartialIngestion(_) => StatusCode::PARTIAL_CONTENT,
            IngestionError::DefaultError => StatusCode::BAD_REQUEST
        }
    }
}

impl From<IngestionResponse> for IngestionError{
    fn from(e: IngestionResponse) -> Self{
        match e{
            IngestionResponse::Failed(pl) => IngestionError::PartialIngestion(pl),
            _ => IngestionError::DefaultError,
        }
   }
}
const MAX_SIZE: usize = 262_144;

#[post("/ingest")]
pub async fn ingest(sdb_repo: Data<SDBRepository>, mut payload: web::Payload) -> Result<Json<String>, Error>{
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let data_points = serde_json::from_slice::<IngestionPacket>(&body)?;
    match sdb_repo.ingest_data(data_points).await {
        Ok(response) => Ok(Json(response)),
        Err(response) => Err(IngestionError::from(response).into())
    }
}
