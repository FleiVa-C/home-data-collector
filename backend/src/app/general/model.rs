use actix_web::http;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DefaultErrorResponse {
    pub status: u16,
    pub message: String,
}

impl DefaultErrorResponse {
    pub fn init(code: http::StatusCode, msg: String) -> Self {
        DefaultErrorResponse {
            status: code.as_u16(),
            message: msg,
        }
    }
}
