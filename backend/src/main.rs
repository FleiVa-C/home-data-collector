#![allow(unused)]
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::{info, warn};
use std::net::{IpAddr, SocketAddr};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;

mod app;
mod config;
mod sdb;

use app::interface::route::*;
use app::signal_data::route::*;
use app::signal_meta::route::*;
use app::user::route::*;
use config::ServerConfig;
use hdc_shared::utils::config::load_config;
use sdb::SDBRepository;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config: ServerConfig = ServerConfig::load();
    println!("{:?}", config);

    std::env::set_var("RUST_LOG", &config.loglevel);
    env_logger::init();

    let sdb_repo: SDBRepository = SDBRepository::init(&config).await;

    HttpServer::new(move || {
        let logger = Logger::default();
        let sdb_data = Data::new(sdb_repo.clone());
        App::new()
            .wrap(logger)
            .app_data(sdb_data)
            .service(ingest_ts_data)
            .service(query_timeseries)
            .service(register_interface)
            .service(get_tasks)
            .service(query_interface)
            .service(register_user)
            .service(query_user)
            .service(query_signal_meta)
    })
    .bind((SocketAddr::new(IpAddr::V4(config.listen_address), config.listen_port)))?
    .run()
    .await
}
