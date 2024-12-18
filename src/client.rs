use vulnerabilities::vulnerabilities_client::VulnerabilitiesClient;
use vulnerabilities::WorkloadMetricRequest;

pub mod vulnerabilities {
    tonic::include_proto!("vulnerabilities");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = VulnerabilitiesClient::connect("http://[::1]:50051").await?;

    let req = tonic::Request::new(WorkloadMetricRequest {
        namespace: "nais-system".to_string(),
        cluster: "".to_string(),
    });

    let metric_reply = client.get_workloads_vulnerability_metrics(req).await?;
    
    println!("RESPONSE={:?}", metric_reply);
    Ok(())
}
