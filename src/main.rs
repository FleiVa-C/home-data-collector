#![allow(unused)]

use actix_web::{HttpServer, App, web::Data, middleware::Logger};

use crate::prelude::*;

mod error;
mod prelude;
mod utils;
mod api;

use api::test::{
    index
};


#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUTS_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
        .wrap(logger)
        .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}