use actix_web::{
    error::{self, PayloadError, ResponseError},
    get,
    http::{header::ContentLength, header::ContentType, StatusCode},
    post, web,
    web::{Data, Header, Json, Path, Query},
    HttpResponse,
};
use derive_more::Display;
use futures::StreamExt;
use log::{info, error};
use serde::{Deserialize, Serialize};
use surrealdb::error::Api;

use super::model::UserQuery;
use crate::app::general::error::{unpack_surrealdb_error, BackendError};
use crate::sdb::SDBRepository;
use hdc_shared::models::user::User;

#[post("v1/user/register")]
pub async fn register_user(
    sdb_repo: Data<SDBRepository>,
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
    let mut user = serde_json::from_slice::<User>(&body)?;
    user.add_uuid();

    let response = sdb_repo.register_user(user).await;
    match response {
        Ok(()) => Ok(Json("Success".to_string())),
        Err(e) => {
            info!("{}", e);
            let error = unpack_surrealdb_error(e).unwrap();
            match error {
                Api::Query(msg) => Err(BackendError::AlreadyExists(msg)),
                _ => Err(BackendError::SomethingWentWrong(
                    "Something went wrong.".to_string(),
                )),
            }
        }
    }
}
#[get("v1/user")]
pub async fn query_user(
    sdb_repo: Data<SDBRepository>,
    query: Query<UserQuery>,
) -> Result<Json<Vec<User>>, BackendError> {
    let response: Result<Vec<User>, surrealdb::Error> =
        sdb_repo.query_user(query.into_inner()).await;
    match response {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            error!("{}", e);
            let error = unpack_surrealdb_error(e).unwrap();
            match error {
                Api::Query(msg) => Err(BackendError::MalformedQuerry(msg)),
                _ => Err(BackendError::SomethingWentWrong(
                    "Something went wrong.".to_string(),
                )),
            }
        }
    }
}
