use actix_web::{
    get,
    post,
    error::{ResponseError, PayloadError, self},
    web::{Path, Json, Data, Header},
    web,
    HttpResponse,
    http::{header::ContentType, StatusCode, header::ContentLength}};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use derive_more::Display;
use crate::app::signal_meta::{
    model::{Signal, SignalIdentifier},
    controller,
    error::SignalError};
use crate::sdb::SDBRepository;

#[post("v1/register_signal/{signal_identifier}")]
pub async fn register_signal(sdb_repo: Data<SDBRepository>, mut payload: web::Payload,
                             content_length: Header<ContentLength>)-> Result<Json<String>, SignalError> {
    let mut body = web::BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > body_length {
            return Err(SignalError::Overflow("Overflow Error".to_string()));
        }
        body.extend_from_slice(&chunk);
    }
    let signal: Signal = serde_json::from_slice::<Signal>(&body)?;
    match sdb_repo.register_signal(signal).await {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(_) => Err(SignalError::SignalAlreadyExists("Signal already exists.".to_string()))
    }
}

#[get("v1/get_signal_all")]
pub async fn get_signal_all(sdb_repo: Data<SDBRepository>) -> Result<Json<Vec<Signal>>, SignalError> {
    let response: Result<Vec<Signal>, SignalError> = sdb_repo.get_all_signals().await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(_)=> Err(SignalError::SignalNotFound)
    }
}

#[get("v1/get_signal/{signal_identifier}")]
pub async fn get_signal(sdb_repo: Data<SDBRepository>, signal_uuid: Path<SignalIdentifier>) -> Result<Json<Signal>, SignalError> {
    let signal_identifier: SignalIdentifier = SignalIdentifier::new(signal_uuid.signal_identifier.clone());
    let response: Option<Signal> = sdb_repo.get_signal(signal_identifier).await;
    match response {
        Some(response) => Ok(Json(response)),
        None => Err(SignalError::SignalNotFound)
    }
}
