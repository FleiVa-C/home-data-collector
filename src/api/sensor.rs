use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    HttpResponse,
    http::{header::ContentType, StatusCode}};

use serde::{Serialize, Deserialize};
//use derive_more::{Display};

#[derive(Deserialize, Serialize)]
pub struct SensorUUID {
    sensor_uuid: String,
}


#[get("/sensor/{sensor_uuid}")]
pub async fn get_sensor_uuid(sensor_identifier: Path<SensorUUID>) -> Json<String> {
    return Json(sensor_identifier.into_inner().sensor_uuid);
}