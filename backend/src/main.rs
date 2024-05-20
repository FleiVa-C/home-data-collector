#![allow(unused)]
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use actix_cors::Cors;
use log::{info, warn, debug};
use std::net::{IpAddr, SocketAddr};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;

mod app;
mod config;
mod sdb;
mod error;

use app::interface::route::*;
use app::signal_data::route::*;
use config::ServerConfig;
use hdc_shared::utils::config::load_config;
use sdb::SDBRepository;


pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config: ServerConfig = ServerConfig::load();

    std::env::set_var("RUST_LOG", &config.loglevel);
    env_logger::init();
    debug!("{:?}", config);

    let sdb_repo: SDBRepository = SDBRepository::init(&config).await;

    HttpServer::new(move || {
        let logger = Logger::default();
        let sdb_data = Data::new(sdb_repo.clone());
        let cors = Cors::default()
            .allow_any_origin();
        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(sdb_data)
            .service(ingest_ts_data)
            .service(query_timeseries)
            .service(register_interface)
            .service(update_interface)
            .service(get_tasks)
            .service(query_interface)
    })
    .bind((SocketAddr::new(IpAddr::V4(config.listen_address), config.listen_port)))?
    .run()
    .await
}
