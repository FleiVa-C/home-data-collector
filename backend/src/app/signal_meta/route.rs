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

use super::model::SignalMetaQuery;
use hdc_shared::models::signal_meta::SignalMeta;
use crate::sdb::SDBRepository;
use crate::app::signal_meta::{
    controller,
    error::SignalError,
};

#[get("v1/get_signal_all")]
pub async fn get_signal_all(
    sdb_repo: Data<SDBRepository>,
) -> Result<Json<Vec<SignalMeta>>, SignalError> {
    let response: Result<Vec<SignalMeta>, SignalError> = sdb_repo.get_all_signals().await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(SignalError::SignalNotFound),
    }
}

#[get("v1/signal")]
pub async fn query_signal_meta(
    sdb_repo: Data<SDBRepository>,
    query: Query<SignalMetaQuery>
    ) -> Result<Json<Vec<SignalMeta>>, SignalError> {
    let response: Result<Vec<SignalMeta>, surrealdb::Error> = sdb_repo.query_signal_meta(query.into_inner()).await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(SignalError::SignalNotFound),
    }
}
