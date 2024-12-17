use std::env;
use std::net::SocketAddr;
use tonic::Response;
use tonic::transport::Server;
use vulnerabilities::vulnerabilities_server::VulnerabilitiesServer;
use vulnerabilities::{WorkloadReply, WorkloadRequest};
use crate::vulnerabilities::vulnerabilities_server::Vulnerabilities;

mod dependencytrack;
mod workload;

pub mod vulnerabilities {
    tonic::include_proto!("vulnerabilities");
}

fn setup_client() -> dependencytrack::Client {
    dotenv::dotenv().ok();
    let base_url = env::var("DEPENDENCYTRACK_BASE_URL")
        .expect("DEPENDENCYTRACK_BASE_URL must be set");
    let api_key = env::var("DEPENDENCYTRACK_API_KEY")
        .expect("DEPENDENCYTRACK_API_KEY must be set");

    dependencytrack::Client::new(base_url, api_key)
}

#[derive(Debug)]
pub struct VulnerabilitiesService {}

#[tonic::async_trait]
impl Vulnerabilities for VulnerabilitiesService {
    async fn get_workload_vulnerabilities(
        &self,
        request: tonic::Request<WorkloadRequest>,
    ) -> Result<Response<WorkloadReply>, tonic::Status> {
        println!("Got a request: {:?}", request);
        let namespace_prefix = "team:"; 
        let client = setup_client();
        
        let req = request.into_inner();
        let namespace = req.namespace.as_str();
        let cluster = req.cluster.as_str();

        // Fetch projects by tag
        match client.get_projects_by_tag(&format!("{}{}", namespace_prefix, namespace)).await {
            Ok(projects) => {
                let reply = workload::parse_workloads(projects, namespace, cluster);
                println!("Reply: {:?}", reply);
                Ok(Response::new(reply))
            },
            Err(e) => {
                eprintln!("Error fetching projects: {}", e);
                Err(tonic::Status::internal("Error fetching projects"))
            }
        }
    }
}

impl VulnerabilitiesService {
    pub fn new() -> VulnerabilitiesService {
        VulnerabilitiesService {}
    }
}

impl Default for VulnerabilitiesService {
    fn default() -> Self {
        VulnerabilitiesService {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()?;
    println!("Parsed address: {:?}", addr);
    let vulnerability_server = VulnerabilitiesService {};

    Server::builder()
        .add_service(VulnerabilitiesServer::new(vulnerability_server))
        .serve(addr)
        .await?;
    
    println!("Successfully fetched vulnerabilities.");

    Ok(())
}
