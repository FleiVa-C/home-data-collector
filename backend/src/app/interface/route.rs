use super::model::InterfaceQuery;
use actix_web::{
    get,
    http::{
        header::{ContentLength, ContentType},
        StatusCode,
    },
    post,
    web::{self, Data, Header, Json, Path, Query},
    HttpResponse,
};
use futures::StreamExt;
use log::*;
use serde::{Deserialize, Serialize};
use std::io;
use surrealdb::{Error, error::Api};

use crate::app::general::error::{BackendError, unpack_surrealdb_error};
use crate::sdb::SDBRepository;

use hdc_shared::models::{
    interface::{Interface, InterfaceType},
    tasklist::{CollectorTask, Tasklist}
};
#[post("v1/register_interface/{interface_type}")]
pub async fn register_interface(
    sdb_repo: Data<SDBRepository>,
    interface_type: Path<InterfaceType>,
    mut payload: web::Payload,
    content_length: Header<ContentLength>,
) -> Result<Json<String>, BackendError> {
    let mut body = web::BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > body_length {
            return Err(BackendError::Overflow("Overflow Error".to_string()));
        }
        body.extend_from_slice(&chunk);
    }
    let mut interface = serde_json::from_slice::<Interface>(&body)?;

    interface.add_interface_type(interface_type.into_inner());
    interface.add_uuid();

    let response = sdb_repo.register_interface(interface).await;
    match response {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(e) => {
            info!("{}",e);
            let error = unpack_surrealdb_error(e).unwrap();
            match error {
                Api::Query(msg) => Err(BackendError::AlreadyExists(msg)),
                _ => Err(BackendError::SomethingWentWrong("Something went wrong.".to_string()))
                 }
        }
    }
}

#[get("v1/get_all_interfaces")]
pub async fn get_all_interfaces(
    sdb_repo: Data<SDBRepository>,
) -> Result<Json<Vec<Interface>>, BackendError> {
    let response: Result<Vec<Interface>, surrealdb::Error> = sdb_repo.get_all_interfaces().await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            info!("{}", e);
            Err(BackendError::SomethingWentWrong("Something went wrong".to_string()))
        }
    }
}

#[get("v1/get_tasks")]
pub async fn get_tasks(sdb_repo: Data<SDBRepository>) -> Result<Json<Tasklist>, BackendError> {
    let response: Result<Vec<CollectorTask>, surrealdb::Error> = sdb_repo.get_tasks().await;
    match response {
        Ok(response) => Ok(Json(Tasklist { tasks: response })),
        Err(e) => { info!("{}", e);
            Err(BackendError::NotFound)
        }
    }
}

#[get("v1/interface")]
pub async fn query_interface(
    sdb_repo: Data<SDBRepository>,
    query: Query<InterfaceQuery>,
) -> Result<Json<Vec<Interface>>, BackendError> {
    let response: Result<Vec<Interface>, surrealdb::Error> =
        sdb_repo.query_interfaces(query.into_inner()).await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            info!("{}", e);
            let error = unpack_surrealdb_error(e).unwrap();
            match error {
                Api::Query(msg) => Err(BackendError::MalformedQuerry(msg)),
                _ => Err(BackendError::SomethingWentWrong("Something went wrong.".to_string()))
                 }
        }
    }
}
