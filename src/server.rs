use tonic::{Request, Response, Status};
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().clone().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply))
    }
}

impl MyGreeter {
    pub fn new() -> MyGreeter {
        MyGreeter {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::new();

    use tonic::transport::Server;
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
