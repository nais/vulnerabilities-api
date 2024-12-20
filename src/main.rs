use std::env;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use vulnerabilities::vulnerabilities_server::{Vulnerabilities, VulnerabilitiesServer};
use vulnerabilities::{WorkloadMetricReply, WorkloadMetricRequest};
use vulnerabilities::{WorkloadVulnerabilityDetailsReply, WorkloadVulnerabilityDetailsRequest};

mod dependencytrack;
mod workload;

pub mod vulnerabilities {
    tonic::include_proto!("vulnerabilities");
}

fn setup_client() -> dependencytrack::Client {
    dotenv::dotenv().ok();
    let base_url =
        env::var("DEPENDENCYTRACK_BASE_URL").expect("DEPENDENCYTRACK_BASE_URL must be set");
    let api_key = env::var("DEPENDENCYTRACK_API_KEY").expect("DEPENDENCYTRACK_API_KEY must be set");

    dependencytrack::Client::new(base_url, api_key)
}

#[derive(Debug, Default)]
pub struct VulnerabilitiesService {}

#[tonic::async_trait]
impl Vulnerabilities for VulnerabilitiesService {
    async fn get_workloads_vulnerability_metrics(
        &self,
        request: Request<WorkloadMetricRequest>,
    ) -> Result<Response<WorkloadMetricReply>, Status> {
        println!("Got a request: {:?}", request);
        let namespace_prefix = "team:";
        let client = setup_client();

        let req = request.into_inner();
        let namespace = req.namespace.as_str();
        let cluster = req.cluster.as_str();

        // Fetch projects by tag
        match client
            .get_projects_by_tag(&format!("{}{}", namespace_prefix, namespace))
            .await
        {
            Ok(projects) => {
                let reply = workload::parse_workloads(projects, namespace, cluster);
                println!("Reply: {:?}", reply);
                Ok(Response::new(reply))
            }
            Err(e) => {
                eprintln!("Error fetching projects: {}", e);
                Err(e)
            }
        }
    }

    async fn get_workload_vulnerability_details(
        &self,
        request: Request<WorkloadVulnerabilityDetailsRequest>,
    ) -> Result<Response<WorkloadVulnerabilityDetailsReply>, Status> {
        println!("Got a request: {:?}", request);
        let client = setup_client();

        let req = request.into_inner();
        let namespace = req.namespace.as_str();
        let cluster = req.cluster.as_str();
        let workload = req.workload.as_str();
        let workload_type = req.workload_type.as_str();

        // Fetch projects by tag
        match client
            .get_vulnerability_details_for_workload(workload, workload_type, namespace, cluster)
            .await
        {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => {
                eprintln!("Error fetching projects: {}", e);
                Err(e)
            }
        }
    }
}

impl VulnerabilitiesService {
    pub fn new() -> VulnerabilitiesService {
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
