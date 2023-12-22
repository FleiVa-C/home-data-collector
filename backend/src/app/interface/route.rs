use actix_web::{
    get,
    http::{
        header::{ContentLength, ContentType},
        StatusCode,
    },
    post,
    web::{self, Data, Header, Json, Path},
    HttpResponse,
};
use derive_more::Display;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::io;
use hdc_shared::models::interface::*;

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
    let mut interface = serde_json::from_slice::<Interface<ShellyV1Adapter>>(&body)?;

//    let mut interface = match &interface_type{
//        InterfaceType::ShellyV1 => serde_json::from_slice::<Interface<ShellyV1Adapter>>(&body)?,
//        InterfaceType::ShellyV2 => serde_json::from_slice::<Interface<ShellyV1Adapter>>(&body)?,
//        InterfaceType::WeatherAPI => serde_json::from_slice::<Interface<WeatherAdapter>>(&body)?,
//    };

    interface.add_interface_type(interface_type.into_inner());
    interface.add_uuids();
    
//    let response = match &interface_type{
//        InterfaceType::ShellyV1 => sdb_repo.register_interface::<ShellyV1Adapter>(interface).await,
//        InterfaceType::ShellyV2 => sdb_repo.register_interface::<ShellyV2Adapter>(interface).await,
//        InterfaceType::WeatherAPI => sdb_repo.register_interface::<WeatherAdapter>(interface).await,
//    };
    let response = sdb_repo.register_interface(interface).await;
    match response {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(_) => Err(DefaultError::AlreadyExists(
            "Interface already exists.".to_string(),
        )),
    }
}

//#[get("v1/get_all_interfaces")]
//pub async fn get_all_interfaces(
//    sdb_repo: Data<SDBRepository>,
//    ) -> Result<Json<Vec<Interface>>, DefaultError> {
//    let response: Option<Vec<Interface>> = sdb_repo.get_all_interfaces().await;
//    match response {
//        Some(response) => Ok(Json(response)),
//        None => Err(DefaultError::NotFound),
//    }
//}
//
//#[get("v1/get_interface/{interface_url}")]
//pub async fn get_interface(
//    sdb_repo: Data<SDBRepository>,
//    interface_url: Path<InterfaceIdentifier>,
//) -> Result<Json<Interface>, DefaultError> {
//    let response: Option<Interface> =
//        sdb_repo.get_interface(interface_url.into_inner()).await; 
//    match response {
//        Some(response) => Ok(Json(response)),
//        None => Err(DefaultError::NotFound),
//    }
//}

