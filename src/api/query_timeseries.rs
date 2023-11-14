use actix_web::{
    get,
    Error,
    error::{ResponseError, self},
    web::{Data, Json, BytesMut, Header, Payload},
    HttpResponse,
    http::{header::{ContentType, ContentLength}, StatusCode}};
use serde::{Serialize, Deserialize};
use crate::model::query_timeseries::QueryTimeseriesData;
use crate::repository::sdb::{SDBRepository, QueryResponse, QueryResult};
use futures::StreamExt;
use std::fmt::Debug;
use derive_more::Display;


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

const MAX_SIZE: usize = 262_144;

#[get("/query/timeseriesdata")]
pub async fn query_timeseries(sdb_repo: Data<SDBRepository>, mut payload: Payload,
                              content_length: Header<ContentLength>) -> Result<Json<QueryResult>, Error>{
    let mut body = BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > body_length{
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let query = serde_json::from_slice::<QueryTimeseriesData>(&body)?;
    match sdb_repo.query_timeseries(query).await {
        QueryResponse::Success(response) => Ok(Json(response)),
        QueryResponse::Failed => return Err(QueryError::Failed.into())
    }
}
