use hello_api::hello_api_client::HelloApiClient;
use hello_api::HelloRequest;

pub mod hello_api {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloApiClient::connect("http://[::1]:50051").await?;

    println!("RESPONSE={:?}", client.hello(tonic::Request::new(HelloRequest {
      name: "Rocky".into(),
  })).await?);

    Ok(())
}
