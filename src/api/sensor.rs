use actix_web::{
    get,
    post,
    put,
    error::{ResponseError, PayloadError, self},
    web::Path,
    web::Json,
    web::Data,
    web,
    HttpResponse,
    http::{header::ContentType, StatusCode}};
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use derive_more::Display;
use crate::model::sensor::Sensor;
use crate::repository::sdb::{SDBRepository, SDBError};



#[derive(Debug, Display)]
pub enum SensorError{
    SensorNotFound,
    SensorRegisterFailure,
    SensorInfoOverflow,
    PayloadError,
    ParseError,
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
            SensorError::SensorInfoOverflow => StatusCode::BAD_REQUEST,
            SensorError::PayloadError => StatusCode::BAD_REQUEST,
            SensorError::ParseError => StatusCode::BAD_REQUEST
        }
    }
}

impl From<PayloadError> for SensorError{
    fn from(e: PayloadError) -> Self{
        SensorError::PayloadError
    }
}

impl From<serde_json::Error> for SensorError{
    fn from(e: serde_json::Error) -> Self{
        SensorError::ParseError
    }
}

#[derive(Deserialize, Serialize)]
pub struct SensorIdentifier {
    sensor_identifier: String,
}

impl SensorIdentifier{
    fn new(identifier: String) -> SensorIdentifier{
        SensorIdentifier { sensor_identifier: identifier }
    }
    pub fn get_global_id(&self) -> String {
        return format!("{}", self.sensor_identifier);
    }
}


const MAX_SIZE: usize = 262_144;


#[get("/sensor/{sensor_uuid}")]
pub async fn get_sensor_uuid(sensor_identifier: Path<SensorIdentifier>) -> Json<String> {
    return Json(sensor_identifier.into_inner().sensor_identifier);
}

#[post("/register_sensor/{sensor_identifier}")]
pub async fn register_sensor(sdb_repo: Data<SDBRepository>, sensor_uuid: Path<SensorIdentifier>, mut payload: web::Payload) -> Result<Json<SensorIdentifier>, SensorError> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(SensorError::SensorInfoOverflow);
        }
        body.extend_from_slice(&chunk);
    }
    let sensor: Sensor = serde_json::from_slice::<Sensor>(&body)?;
    let sensor_id = sensor.get_global_id();
    match sdb_repo.register_sensor(sensor).await {
        Ok(()) => Ok(Json(SensorIdentifier {sensor_identifier: sensor_id})),
        Err(_) => Err(SensorError::SensorNotFound)
    }
}

#[get("/get_sensor_all")]
pub async fn get_all_sensors(sdb_repo: Data<SDBRepository>) -> Result<Json<Vec<Sensor>>, SensorError> {
    let response: Result<Vec<Sensor>, SDBError> = sdb_repo.get_all_sensors().await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(_)=> Err(SensorError::SensorNotFound)
    }
}

#[get("/get_sensor/{sensor_identifier}")]
pub async fn get_sensor(sdb_repo: Data<SDBRepository>, sensor_uuid: Path<SensorIdentifier>) -> Result<Json<Sensor>, SensorError> {
    let sensor_identifier: SensorIdentifier = SensorIdentifier::new(sensor_uuid.sensor_identifier.clone());
    let response: Option<Sensor> = sdb_repo.get_sensor(sensor_identifier).await;
    match response {
        Some(response) => Ok(Json(response)),
        None => Err(SensorError::SensorRegisterFailure)
    }
}