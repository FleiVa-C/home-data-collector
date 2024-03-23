use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::u16;

#[derive(Serialize, Deserialize)]
struct ServerConfigToml {
    logging: Option<Logging>,
    listen_parameters: Option<Endpoint>,
    database: Option<Database>,
}

#[derive(Serialize, Deserialize)]
struct Logging {
    loglevel: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Endpoint {
    address: Option<Ipv4Addr>,
    port: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct Database {
    address: Option<Ipv4Addr>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    namespace: Option<String>,
    database: Option<String>,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub loglevel: String,
    pub listen_address: Ipv4Addr,
    pub listen_port: u16,
    pub db_address: Ipv4Addr,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_namespace: String,
    pub db_database: String,
}

impl ServerConfig {
    pub fn load() -> Self {
        let config_filepaths: [&str; 2] = ["./config.toml", "./Config.toml"];

        let mut content: String = String::new();

        for filepath in config_filepaths {
            let config: io::Result<String> = fs::read_to_string(filepath);

            if config.is_ok() {
                content = config.unwrap();
                break;
            }
        }

        let config_toml: ServerConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            warn!("Failed to read Config");
            ServerConfigToml {
                logging: None,
                listen_parameters: None,
                database: None,
            }
        });

        let loglevel: String = match config_toml.logging {
            Some(logging) => logging.loglevel.unwrap_or_else(|| {
                warn!("Missing field loglevel in table logging, taking info as default");
                "info".to_owned()
            }),
            None => {
                warn!("Missing table logging, taking info as default loglevel");
                "info".to_owned()
            }
        };

        let (listen_address, listen_port): (Ipv4Addr, u16) = match config_toml.listen_parameters {
            Some(listen_parameters) => {
                let address: Ipv4Addr = listen_parameters.address.unwrap_or_else(|| {
                    warn!("Missing field address in table endpoint, taking 0.0.0.0 as default");
                    Ipv4Addr::new(0, 0, 0, 0)
                });

                let port: u16 = listen_parameters.port.unwrap_or_else(|| {
                    warn!("Missing field port in table endpoint, taking 8080 as default");
                    8080
                });
                (address, port)
            }
            None => {
                warn!("Missing table endpoint, taking default values");
                (Ipv4Addr::new(127, 0, 0, 1), 8080)
            }
        };
        let (db_address, db_port, db_username, db_password, db_namespace, db_database): (
            Ipv4Addr,
            u16,
            String,
            String,
            String,
            String,
        ) = match config_toml.database {
            Some(db_param) => {
                let address: Ipv4Addr = db_param.address.unwrap_or_else(|| {
                    warn!("Missing field address in table databse, taking 127.0.0.1 as default.");
                    Ipv4Addr::new(127, 0, 0, 1)
                });

                let port: u16 = db_param.port.unwrap_or_else(|| {
                    warn!("Missing field port in table database, taking 80 as default.");
                    80
                });

                let username: String = db_param.username.unwrap_or_else(|| {
                    warn!("Missing field username in table database, taking default.");
                    "dev".to_owned()
                });

                let password: String = db_param.password.unwrap_or_else(|| {
                    warn!("Missing field password in table database, taking default.");
                    "dev".to_owned()
                });

                let namespace: String = db_param.namespace.unwrap_or_else(|| {
                    warn!("Missing field namespace in table database, taking default.");
                    "dev".to_owned()
                });

                let database: String = db_param.database.unwrap_or_else(|| {
                    warn!("Missing field database in table database, taking default.");
                    "dev".to_owned()
                });
                (address, port, username, password, namespace, database)
            }
            None => (
                Ipv4Addr::new(127, 0, 0, 1),
                80,
                "dev".to_owned(),
                "dev".to_owned(),
                "dev".to_owned(),
                "dev".to_owned(),
            ),
        };
        ServerConfig {
            loglevel,
            listen_address,
            listen_port,
            db_address,
            db_port,
            db_username,
            db_password,
            db_namespace,
            db_database,
        }
    }
}
