use std::net::SocketAddr;
use tonic::{async_trait, Extensions, Request, Response, Status};
use tonic::metadata::MetadataMap;
use tonic::transport::Server;
use records::recorder_server::{Recorder, RecorderServer};
use records::{RecordRequest, RecordResponse};

pub mod records {
    tonic::include_proto!("records");
}

#[derive(Debug, Default)]
pub struct RecorderService {}

#[async_trait]
impl Recorder for RecorderService {
    async fn send_message(&self, request: Request<RecordRequest>) -> Result<Response<RecordResponse>, Status> {
        let req: RecordRequest = request.into_inner();
        println!("request={:#?}", req);

        let response = RecordResponse{
            successful: true,
            message: format!("name: {}, age: {}", req.user_name, req.user_age).into()
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("server");
    // let addr: SocketAddr = "[::1]:50050".parse()?;
    let addr: SocketAddr = "127.0.0.1:50050".parse()?;
    println!("Server listening on {:?}", addr);

    let recorder = RecorderService::default();
    Server::builder()
        .add_service(RecorderServer::new(recorder))
        .serve(addr)
        .await;
    Ok(())
}














