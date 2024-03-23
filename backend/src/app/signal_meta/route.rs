use actix_web::{
    error::{self, PayloadError, ResponseError},
    get,
    http::{header::ContentLength, header::ContentType, StatusCode},
    post, web,
    web::{Data, Header, Json, Path, Query},
    HttpResponse,
};
use derive_more::Display;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use surrealdb::error::Api;

use super::model::SignalMetaQuery;
use crate::app::general::error::{unpack_surrealdb_error, BackendError};
use crate::sdb::SDBRepository;
use hdc_shared::models::signal_meta::SignalMeta;

#[get("v1/signal")]
pub async fn query_signal_meta(
    sdb_repo: Data<SDBRepository>,
    query: Query<SignalMetaQuery>,
) -> Result<Json<Vec<SignalMeta>>, BackendError> {
    let response: Result<Vec<SignalMeta>, surrealdb::Error> =
        sdb_repo.query_signal_meta(query.into_inner()).await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error = unpack_surrealdb_error(e).unwrap();
            match error {
                Api::Query(msg) => Err(BackendError::MalformedQuerry(msg)),
                _ => Err(BackendError::SomethingWentWrong(
                    "Something went wrong.".to_string(),
                )),
            }
        }
    }
}
