use tonic::{transport::Channel, Request, Response, Status};
use subtractor_rs::subtractor_client::SubtractorClient; // Import SubtractorClient
use subtractor_rs::subtractor_server::Subtractor; // Import Subtractor trait
use subtractor_rs::{SubtractInputMessage, SubtractOutputMessage};

pub mod subtractor_rs {
    tonic::include_proto!("com.pojtinger.felicitas.grpc_examples");
}

    
#[derive(Debug)]
pub struct MySubtractor {
    addr: String,
}

impl MySubtractor {
    pub fn new(addr: &str) -> Self {
        MySubtractor { addr: addr.to_string() } // Clone addr to take ownership
    }

    async fn make_subtraction_request(
        &self,
        minuend: i64,
        subtrahend: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(self.addr.clone())?.connect().await?;
        let mut client = SubtractorClient::new(channel);

        let request = Request::new(SubtractInputMessage {
            minuend,
            subtrahend,
        });

        let response = client.subtract(request).await?;
        Ok(response.into_inner().difference)
    }
}

#[tonic::async_trait]
impl Subtractor for MySubtractor {
    async fn subtract(
        &self,
        request: Request<SubtractInputMessage>,
    ) -> Result<Response<SubtractOutputMessage>, Status> {
        let sim = request.into_inner();

        let difference = sim.minuend - sim.subtrahend;

        let reply = SubtractOutputMessage { difference };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("ADDR").unwrap_or("http://localhost:5000".to_string());

    let subtractor = MySubtractor::new(&addr); // Pass a reference to addr instead of ownership

    println!("Subtractor listening on {}", addr);

    Ok(())
}



