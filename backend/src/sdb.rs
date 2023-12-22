use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{opt::auth::Root, Surreal};

#[derive(Clone)]
pub struct SDBRepository {
    pub db: Surreal<Client>,
}

impl SDBRepository {
    pub async fn init() -> Self {
        let mut client: Surreal<Client> = Surreal::new::<Ws>("192.168.0.240:80")
            .await
            .expect("Can't connect to SurrealBD instance!");
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap();
        client.use_ns("test").use_db("test").await.unwrap();
        SDBRepository { db: client }
    }
}
