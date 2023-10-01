use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}};

use serde::{Serialize, Deserialize};
use derive_more::Display;

use crate::model::sensor::Sensor;
use crate::repository::sdb::SDBRepository;

#[derive(Debug, Display)]
pub enum SensorError{
    SensorNotFound,
    SensorRegisterFailure,
}

impl ResponseError for SensorError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SensorError::SensorNotFound => StatusCode::NOT_FOUND,
            SensorError::SensorRegisterFailure => StatusCode::FAILED_DEPENDENCY,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SensorIdentifier {
    sensor_identifier: String,
}


#[get("/sensor/{sensor_uuid}")]
pub async fn get_sensor_uuid(sensor_identifier: Path<SensorIdentifier>) -> Json<String> {
    return Json(sensor_identifier.into_inner().sensor_identifier);
}

#[post("/register_sensor/{sensor_identifier}")]
pub async fn register_sensor(sdb_repo: Data<SDBRepository>, sensor_uuid: Path<SensorIdentifier>) -> Result<Json<SensorIdentifier>, SensorError> {
    let sensor = Sensor::new(sensor_uuid.sensor_identifier.clone());
    let sensor_id = sensor.get_global_id();
    match sdb_repo.register_sensor(sensor).await {
        Ok(()) => Ok(Json(SensorIdentifier {sensor_identifier: sensor_id})),
        Err(_) => Err(SensorError::SensorRegisterFailure)
    }
}