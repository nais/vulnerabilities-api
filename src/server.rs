use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("helloworld_descriptor");
}

#[derive(Debug)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
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

impl Default for MyGreeter {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::new();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(hello_world::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Server::builder()
        .add_service(service)
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
