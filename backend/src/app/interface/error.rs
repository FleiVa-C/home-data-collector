use actix_web::{
    error::{self, PayloadError, ResponseError},
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::From;
use serde::Serialize;
use serde_with::SerializeDisplay;
use derive_more::Display;
use super::super::problem_details::ProblemDetails;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    BodyOverflow(String),
    UnknownLength,
    CannotParse,
    MalformedQuery,
    InterfaceAlreadyExists(String),
    UpdateUuidNotAllowed,
    UpdateInterfacetypeNotAllowed,
    SomethingWrong,

    #[from]
    Db{error: surrealdb::Error, instance: String},

    #[from]
    Json{error: serde_json::Error, instance: String},

    #[from]
    Payload{error: actix_web::error::PayloadError, instance: String},

}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(
                serde_json::to_string(&ProblemDetails::build(self))
                .unwrap(),
            )
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(error_status(self)).unwrap()
    }
}

impl ProblemDetails{
    fn build(value: &Error) -> Self {
        let status_code: u16 = error_status(&value);
        match value {
            Error::Json { error, instance } => ProblemDetails::new(None, status_code, "Unable to Parse body".to_owned(), error.to_string(), instance.to_string()),
            Error::Payload { error, instance } => ProblemDetails::new(None, status_code, "Payload Error".to_owned(), error.to_string(), instance.to_string()),
            Error::Db { error, instance } => ProblemDetails::new(None, status_code, "Database Error".to_owned(), error.to_string(), instance.to_string()),
            Error::InterfaceAlreadyExists(instance) => ProblemDetails::new(None, status_code, "Database Error".to_owned(), "Interface Already exists".to_owned(), instance.to_owned()),
            _ => ProblemDetails::new(None, status_code, "Some Other Error".to_owned(), "Some Message".to_string(), "Some Instance".to_string()),
        }
    }
}

fn error_status(error: &Error) -> u16 {
    match error {
        Error::UpdateUuidNotAllowed => 403,
        Error::UpdateInterfacetypeNotAllowed => 403,
        Error::SomethingWrong => 500,
        _ => 400
    }
}

