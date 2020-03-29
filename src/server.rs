use tonic::{transport::Server, Request, Response, Status};

use hello_api::hello_api_server::{HelloApi, HelloApiServer};
use hello_api::{HelloRequest, HelloResponse};

pub mod hello_api {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct HApi {}

#[tonic::async_trait]
impl HelloApi for HApi {
    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(hello_api::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = HApi::default();

    Server::builder()
        .add_service(HelloApiServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
