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
use crate::model::signal::Signal;
use crate::repository::sdb::{SDBRepository, SDBError};



#[derive(Debug, Display)]
pub enum SignalError{
    SignalNotFound,
    SignalRegisterFailure,
    SignalInfoOverflow,
    PayloadError,
    ParseError,
}

impl ResponseError for SignalError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SignalError::SignalNotFound => StatusCode::NOT_FOUND,
            SignalError::SignalRegisterFailure => StatusCode::FAILED_DEPENDENCY,
            SignalError::SignalInfoOverflow => StatusCode::BAD_REQUEST,
            SignalError::PayloadError => StatusCode::BAD_REQUEST,
            SignalError::ParseError => StatusCode::BAD_REQUEST
        }
    }
}

impl From<PayloadError> for SignalError{
    fn from(e: PayloadError) -> Self{
        SignalError::PayloadError
    }
}

impl From<serde_json::Error> for SignalError{
    fn from(e: serde_json::Error) -> Self{
        SignalError::ParseError
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignalIdentifier {
    signal_identifier: String,
}

impl SignalIdentifier{
    fn new(identifier: String) -> SignalIdentifier{
        SignalIdentifier { signal_identifier: identifier }
    }
    pub fn get_global_id(&self) -> String {
        return format!("{}", self.signal_identifier);
    }
}

const MAX_SIZE: usize = 262_144;

#[post("/register_signal/{signal_identifier}")]
pub async fn register_signal(sdb_repo: Data<SDBRepository>, signal_uuid: Path<SignalIdentifier>, mut payload: web::Payload) -> Result<Json<SignalIdentifier>, SignalError> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(SignalError::SignalInfoOverflow);
        }
        body.extend_from_slice(&chunk);
    }
    let signal: Signal = serde_json::from_slice::<Signal>(&body)?;
    let signal_id = signal.get_global_id();
    match sdb_repo.register_signal(signal).await {
        Ok(()) => Ok(Json(SignalIdentifier {signal_identifier: signal_id})),
        Err(_) => Err(SignalError::SignalNotFound)
    }
}

#[get("/get_signal_all")]
pub async fn get_signal_all(sdb_repo: Data<SDBRepository>) -> Result<Json<Vec<Signal>>, SignalError> {
    let response: Result<Vec<Signal>, SDBError> = sdb_repo.get_all_signals().await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(_)=> Err(SignalError::SignalNotFound)
    }
}

#[get("/get_signal/{signal_identifier}")]
pub async fn get_signal(sdb_repo: Data<SDBRepository>, signal_uuid: Path<SignalIdentifier>) -> Result<Json<Signal>, SignalError> {
    let signal_identifier: SignalIdentifier = SignalIdentifier::new(signal_uuid.signal_identifier.clone());
    let response: Option<Signal> = sdb_repo.get_signal(signal_identifier).await;
    match response {
        Some(response) => Ok(Json(response)),
        None => Err(SignalError::SignalRegisterFailure)
    }
}
