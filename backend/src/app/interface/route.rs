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
use derive_more::Display;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::io;
use super::model::InterfaceQuery;
use hdc_shared::models::{tasklist::*, interface::*};

use crate::app::general::error::*;
use crate::sdb::SDBRepository;

#[post("v1/register_interface/{interface_type}")]
pub async fn register_interface(
    sdb_repo: Data<SDBRepository>,
    interface_type: Path<InterfaceType>,
    mut payload: web::Payload,
    content_length: Header<ContentLength>,
) -> Result<Json<String>, DefaultError> {
    let mut body = web::BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > body_length {
            return Err(DefaultError::Overflow("Overflow Error".to_string()));
        }
        body.extend_from_slice(&chunk);
    }
    let mut interface = serde_json::from_slice::<Interface>(&body)?;

    interface.add_interface_type(interface_type.into_inner());
    interface.add_uuid();
    
   let response = sdb_repo.register_interface(interface).await;
    match response {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(_) => Err(DefaultError::AlreadyExists(
            "Interface already exists.".to_string(),
        )),
    }
}

#[get("v1/get_all_interfaces")]
pub async fn get_all_interfaces(
    sdb_repo: Data<SDBRepository>,
    ) -> Result<Json<Vec<Interface>>, DefaultError> {
    let response: Option<Vec<Interface>> = sdb_repo.get_all_interfaces().await;
    match response {
        Some(response) => Ok(Json(response)),
        None => Err(DefaultError::NotFound),
    }
}

#[get("v1/get_interface/{interface_url}")]
pub async fn get_interface(
    sdb_repo: Data<SDBRepository>,
    interface_url: Path<String>,
) -> Result<Json<Interface>, DefaultError> {
    let response: Option<Interface> =
        sdb_repo.get_interface(interface_url.into_inner()).await; 
    match response {
        Some(response) => Ok(Json(response)),
        None => Err(DefaultError::NotFound),
    }
}

#[get("v1/get_tasks")]
pub async fn get_tasks(
    sdb_repo: Data<SDBRepository>,
    ) -> Result<Json<TaskList>, DefaultError> {
    let response: Option<Vec<CollectorTask>> = sdb_repo.get_tasks().await;
    match response {
        Some(response) => Ok(Json(TaskList{tasks: response})),
        None => Err(DefaultError::NotFound),
    }
}

#[get("v1/interface")]
pub async fn query_interface(
    sdb_repo: Data<SDBRepository>,
    query: Query<InterfaceQuery>
    ) -> Result<Json<Vec<Interface>>, DefaultError> {
    let response: Result<Vec<Interface>, surrealdb::Error> = sdb_repo.query_interfaces(query.into_inner()).await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(DefaultError::NotFound),
    }
        
}
