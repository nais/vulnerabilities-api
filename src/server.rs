use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use hello_world::admin_server::{Admin, AdminServer};
use hello_world::{StatCountReply, StatCountRequest};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic::metadata::MetadataValue;

pub mod hello_world {
    tonic::include_proto!("helloworld");
    //include!("proto/helloworld.rs");
    //pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
      //  tonic::include_file_descriptor_set!("/src/proto/helloworld_descriptor");
}

// This function will check the authorization header
fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token: MetadataValue<_> = "Bearer my-token".parse().unwrap();
    match  req.metadata().get("authorization") {
        Some(t) if t == token => Ok(req),
        _ => Err(Status::unauthenticated("Invalid token")),
    }
}

// This is a type alias for the state of the server, which is a shared counter
type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug)]
pub struct AdminService {
    stat: State,
}

// Implement the Admin trait for the AdminService struct
#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_stat_count(
        &self,
        _request: Request<StatCountRequest>,
    ) -> Result<Response<StatCountReply>, Status> {
        let stat = self.stat.read().await;
        Ok(Response::new(StatCountReply {
            count: *stat,
        }))
    }
}

#[derive(Debug)]
pub struct MyGreeter {
    state: State,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let input = request.get_ref();
        self.increment_stat().await;

        // Error handling, if the name field is empty, return an error
        if input.name.is_empty() {
            return Err(Status::invalid_argument("name must not be empty"));
        }

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().clone().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply))
    }
}

impl MyGreeter {
    pub async fn increment_stat(&self) {
        let mut stat = self.state.write().await;
        *stat += 1;
        println!("Stat counter: {}", *stat);
    }
}

impl MyGreeter {
    pub fn new() -> MyGreeter {
        MyGreeter {
            state: std::sync::Arc::new(tokio::sync::RwLock::new(0)),
        }
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
    let state = State::default();
    let greeter = MyGreeter {
        state: state.clone(),
    };

    let admin = AdminService {
        stat: state.clone(),
    };

   /* let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(hello_world::FILE_DESCRIPTOR_SET)
        .build_v1()?;
*/
    Server::builder()
       // .add_service(service)
        .add_service(GreeterServer::new(greeter))
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}
