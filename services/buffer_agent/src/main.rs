#![allow(unused)]
use tonic::{transport::Server, Request, Response, Status};
use surrealdb::Surreal;
use surrealdb::engine::local::{File, Db};

use timeseries_data::buffer_agent_server::{BufferAgent, BufferAgentServer};
use timeseries_data::{BufferRequest, BufferResponse};

pub mod timeseries_data {
    tonic::include_proto!("timeseries_buffer");
}

use hdc_shared::models::ingestion_container::IngestionPacket;

pub struct BufferService{
    db: String//Surreal<Db>
}


#[tonic::async_trait]
impl BufferAgent for BufferService {
    async fn send_timeseries_buffer(
        &self,
        request: Request<BufferRequest>
        ) -> Result<Response<BufferResponse>, Status>{
        println!("Got a request: {:?}", request);
        let data = serde_json::from_slice::<IngestionPacket>(&request.into_inner().data).unwrap();
        println!("{:?}", data);
        let response = BufferResponse {
            confirmation: "success".to_string()
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let addr = "[::1]:50051".parse()?;
    //let db = Surreal::new::<File>("./test.db").await?;
    let buffer_service = BufferService{db: "test".to_string()};

    Server::builder()
        .add_service(BufferAgentServer::new(buffer_service))
        .serve(addr)
        .await?;

    Ok(())
}
