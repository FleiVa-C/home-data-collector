use std::net::{IpAddr, SocketAddr};

use crate::config::ServerConfig;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{opt::auth::Root, Surreal};

#[derive(Clone)]
pub struct SDBRepository {
    pub db: Surreal<Client>,
}

impl SDBRepository {
    pub async fn init(config: &ServerConfig) -> Self {
        let mut client: Surreal<Client> = Surreal::new::<Ws>(SocketAddr::new(
            IpAddr::V4(config.db_address),
            config.db_port,
        ))
        .await
        .expect("Can't connect to SurrealBD instance!");
        client
            .signin(Root {
                username: &config.db_username,
                password: &config.db_password,
            })
            .await
            .unwrap();
        client
            .use_ns(&config.db_namespace)
            .use_db(&config.db_database)
            .await
            .unwrap();
        SDBRepository { db: client }
    }
}
