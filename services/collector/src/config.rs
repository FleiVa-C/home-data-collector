use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize)]
pub struct CollectorConfigYml {
    logging: Option<Logging>,
    api_endpoints: Option<ApiEndpoint>,
    ingestion: Option<IngestionParams>,
    database: Option<LocalDatabase>,
}

#[derive(Serialize, Deserialize)]
struct Logging {
    loglevel: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ApiEndpoint {
    ingestion_url: Option<String>,
    tasklist_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct IngestionParams {
    collection_interval: Option<u64>,
    task_update_interval: Option<u64>,
    buffer_ingestion_interval: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct LocalDatabase {
    db_path: Option<String>,
}

#[derive(Debug)]
pub struct CollectorConfig {
    pub loglevel: String,
    pub ingestion_url: String,
    pub tasklist_url: String,
    pub collection_interval: u64,
    pub task_update_interval: u64,
    pub buffer_ingestion_interval: u64,
    pub db_path: String,
}

impl CollectorConfig {
    pub fn load(config_filepath: &str) -> Self {
        let mut content: String = String::new();

        let config: io::Result<String> = fs::read_to_string(config_filepath);

        if config.is_ok() {
            let reg = Regex::new(r"\$\{(?<value>[A-Za-z_]+)\}").unwrap();
            content = config.unwrap();
            content = reg
                .replace_all(&content, |caps: &Captures| {
                    let var_name = &caps["value"];
                    std::env::var(var_name)
                        .unwrap_or_else(|_| panic!("Environment variable {} not set", var_name))
                })
                .to_string();
        }

        let config_yml: CollectorConfigYml = serde_yml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to read Config");
            CollectorConfigYml {
                logging: None,
                api_endpoints: None,
                ingestion: None,
                database: None,
            }
        });

        let loglevel: String = match config_yml.logging {
            Some(logging) => logging.loglevel.unwrap_or_else(|| {
                println!("Missing field loglevel in table logging, taking info as default");
                "info".to_owned()
            }),
            None => {
                println!("Missing table logging, taking info as default loglevel");
                "info".to_owned()
            }
        };

        let (ingestion_url, tasklist_url): (String, String) = match config_yml.api_endpoints {
            Some(endpoints) => {
                let ingestion_url = endpoints.ingestion_url.unwrap_or_else(|| {
                    println!(
                        "Missing field ingestion_url in table ingestione, taking  default value."
                    );
                    "http://127.0.0.1:8080/v1/ingest".to_owned()
                });

                let tasklist_url = endpoints.tasklist_url.unwrap_or_else(|| {
                    println!(
                        "Missing field tasklist_url in table ingestione, taking  default value."
                    );
                    "http://127.0.0.1:8080/v1/get_tasks".to_owned()
                });
                (ingestion_url, tasklist_url)
            }
            None => {
                println!("Missing table endpoints");
                (
                    "http://127.0.0.1:8080/v1/ingest".to_owned(),
                    "http://127.0.0.1:8080/v1/get_tasks".to_owned(),
                )
            }
        };

        let (collection_interval, task_update_interval, buffer_ingestion_interval): (
            u64,
            u64,
            u64,
        ) = match config_yml.ingestion {
            Some(ingestion_params) => {
                let collection_interval = ingestion_params.collection_interval.unwrap_or_else(||{
                        println!("Missign field collection_interval in Table ingestion, taking 30 as default.");
                        30
                    });
                let task_update_interval = ingestion_params.task_update_interval.unwrap_or_else(||{
                        println!("Missign field task_update_interval in Table ingestion, taking 300 as default.");
                        300
                    });
                let buffer_ingestion_interval = ingestion_params.buffer_ingestion_interval.unwrap_or_else(||{
                        println!("Missign field buffer_ingestion_interval in Table ingestion, taking 600 as default.");
                        600
                    });
                (
                    collection_interval,
                    task_update_interval,
                    buffer_ingestion_interval,
                )
            }
            None => {
                println!("Missign Table ingestion, taking default values.");
                (30, 300, 600)
            }
        };
        let db_path: String = match config_yml.database {
            Some(path) => path.db_path.unwrap_or_else(|| {
                println!(
                    "Missing field db_path in table database, taking ./Data/buffer.db as default."
                );
                "./Data/buffer.db".to_owned()
            }),
            None => {
                println!("Missing table database, taking default parameters.");
                "./Data/buffer.db".to_owned()
            }
        };
        CollectorConfig {
            loglevel,
            ingestion_url,
            tasklist_url,
            collection_interval,
            task_update_interval,
            buffer_ingestion_interval,
            db_path,
        }
    }
}
