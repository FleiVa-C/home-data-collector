use actix_web::{
    Error,
    get,
    post,
    put,
    error::{ResponseError, PayloadError, self, JsonPayloadError},
    web::Path,
    web::{Json, JsonBody},
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

impl ResponseError for IngestionPacket {
    fn error_response(&self) -> HttpResponse {
       HttpResponse::build(StatusCode::ACCEPTED) 
            .insert_header(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
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
        IngestionResponse::Success => Ok(Json("Success".to_string())),
        IngestionResponse::Failed(response) => Err(response.into())
    }
}
