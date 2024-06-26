use tonic::{transport::Server, Request, Response, Status};
use mather_rs::mather_server::{Mather, MatherServer};
use mather_rs::{AddInputMessage, AddOutputMessage};

pub mod mather_rs {
    tonic::include_proto!("com.pojtinger.felicitas.grpc_examples");
}

#[derive(Debug, Default)]
pub struct MyMather;

#[tonic::async_trait]
impl Mather for MyMather {
    async fn add(
        &self,
        request: Request<AddInputMessage>,
    ) -> Result<Response<AddOutputMessage>, Status> {
        let aim = request.into_inner();

        let sum = aim.first_summand + aim.second_summand;

        let reply = AddOutputMessage { sum };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut laddr = std::env::var("LADDR").unwrap_or("0.0.0.0:5000".to_string());
    if laddr == "" {
        laddr = "0.0.0.0:5000".to_string();
    }

    println!("Listening on {}", laddr);

    Server::builder()
        .add_service(MatherServer::new(MyMather::default()))
        .serve(laddr.parse()?)
        .await?;

    Ok(())
}

