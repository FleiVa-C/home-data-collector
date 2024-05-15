use actix_web::http;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BasicErrorMessage {
    pub status: u16,
    pub message: String,
}

impl BasicErrorMessage {
    pub fn new(code: http::StatusCode, msg: String) -> Self {
        BasicErrorMessage {
            status: code.as_u16(),
            message: msg,
        }
    }
}
