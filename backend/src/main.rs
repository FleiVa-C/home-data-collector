#![allow(unused)]
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;

mod app;
mod sdb;

use app::interface::route::*;
use app::signal_data::route::*;
use app::signal_meta::route::*;
use sdb::SDBRepository;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sdb_repo: SDBRepository = SDBRepository::init().await;

    HttpServer::new(move || {
        let logger = Logger::default();
        let sdb_data = Data::new(sdb_repo.clone());
        App::new()
            .wrap(logger)
            .app_data(sdb_data)
            .service(get_signal_all)
            .service(ingest)
            .service(query_timeseries)
            .service(get_interface)
            .service(register_interface)
            .service(get_all_interfaces)
            .service(get_tasks)
            .service(query_interface)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
