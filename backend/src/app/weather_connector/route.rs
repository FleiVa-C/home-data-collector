use super::error::Error;
use actix_web::{
    get,
    http::{
        header::{ContentLength, ContentType},
        StatusCode,
    },
    post, put,
    web::{self, Data, Form, Header, Json, Path, Query},
    HttpRequest, HttpResponse,
};
use futures::StreamExt;
use hdc_shared::models::signal_data::IngestionResponse;
use log::*;
use serde::{Deserialize, Serialize};
use std::io;
use surrealdb::error::Api;

use crate::sdb::SDBRepository;

use hdc_shared::models::{weather_adapter::WeatherModel, weather_model::WeatherResponse};
#[post("v1/weather/ingest")]
pub async fn ingest_weather(
    sdb_repo: Data<SDBRepository>,
    content_length: Header<ContentLength>,
    data: Form<WeatherResponse>,
    req: HttpRequest,
) -> Result<Json<String>, actix_web::Error> {
    let instance: &str = req.path();
    println!("{:?}", data);
    let meta = sdb_repo
        .get_weather_meta(req.peer_addr().unwrap().ip().to_string(), instance)
        .await;
    let data_points = data.into_inner().to_ingestion_packet(meta.unwrap().signals);
    match sdb_repo.ingest_data(data_points).await {
        IngestionResponse::Success => Ok(Json("Success".to_string())),
        IngestionResponse::MultiStatus(response) => Ok(Json(response.to_string())),
    }
}

#[post("v1/weather/register")]
pub async fn register_weather_connector(
    sdb_repo: Data<SDBRepository>,
    mut payload: web::Payload,
    content_length: Header<ContentLength>,
    req: HttpRequest,
) -> Result<Json<String>, Error> {
    let instance: &str = req.path();
    let mut body = web::BytesMut::new();
    let body_length: usize = *content_length.into_inner();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.map_err(|e| Error::Payload {
            error: e,
            instance: instance.to_owned(),
        })?;

        if (body.len() + chunk.len()) > body_length {
            return Err(Error::BodyOverflow {
                instance: instance.to_owned(),
            });
        }
        body.extend_from_slice(&chunk);
    }
    let mut interface =
        serde_json::from_slice::<WeatherModel>(&body).map_err(|error| Error::Json {
            error,
            instance: instance.to_owned(),
        })?;
    interface.add_uuid();

    let response = sdb_repo
        .register_weather_connector(interface, instance)
        .await;
    match response {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(e) => {
            info!("{}", e);
            Err(e)
        }
    }
}
