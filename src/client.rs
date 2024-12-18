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
    
    println!("METRICS={:?}", metric_reply);
    
    let vulnz_reply = client.get_workload_vulnerability_details(
        tonic::Request::new(vulnerabilities::WorkloadVulnerabilityDetailsRequest {
            workload: "certificator".to_string(),
            workload_type: "app".to_string(),
            namespace: "nais-system".to_string(),
            cluster: "dev".to_string(),
        })
    ).await?;
    
    println!("VULNERABILITIES={:?}", vulnz_reply);
    
    Ok(())
}
