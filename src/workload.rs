use crate::vulnerabilities;
use dependencytrack::models::Project;
use vulnerabilities::{VulnerabilityMetrics, Workload, WorkloadMetricReply};

pub fn parse_workloads(projects: Vec<Project>, namespace_name: &str, cluster: &str) -> WorkloadMetricReply {
    println!("Workloads for namespace: {}", namespace_name);

    let mut workload_count = 0; // Counter for workloads
    let mut reply: WorkloadMetricReply = WorkloadMetricReply::default();

    for project in projects {
        let workloads = process_project(&project, namespace_name, cluster);

        workload_count += workloads.len();
        reply.workload.extend(workloads);
    }

    println!("Total workloads in namespace '{}': {}", namespace_name, workload_count);
    reply
}

fn process_project(project: &Project, namespace_name: &str, cluster: &str) -> Vec<Workload> {
    // Skip projects without metrics
    let metrics_data = match &project.metrics {
        Some(metrics) => metrics,
        None => {
            eprintln!("Project {:?} has no metrics, skipping...", project.name);
            return vec![];
        }
    };

    let vulnerability = VulnerabilityMetrics {
        critical: metrics_data.critical,
        high: metrics_data.high,
        medium: metrics_data.medium,
        low: metrics_data.low,
        unassigned: metrics_data.unassigned.unwrap_or(0),
    };

    println!("vulnerability: {:?}", vulnerability);

    let mut workloads = Vec::new();

    // Process tags
    project.tags.iter().for_each(|tags| {
        tags.iter().for_each(|tag| {
            if let Some(tag_name) = &tag.name {
                if let Some(workload) = process_tag(tag_name, namespace_name, cluster, &vulnerability) {
                    workloads.push(workload);
                }
            }
        });
    });

    workloads
}

fn process_tag(
    tag_name: &str,
    namespace_name: &str,
    cluster: &str,
    vulnerability: &VulnerabilityMetrics,
) -> Option<Workload> {
    if tag_name.starts_with("workload:") {
        let parts: Vec<&str> = tag_name["workload:".len()..].split('|').collect();
        if parts.len() == 4 && parts[1] == namespace_name {
            // Match on the cluster or namespace if cluster is provided
            if cluster.is_empty() || parts[0] == cluster {
                return Some(Workload {
                    name: parts[3].to_string(),
                    workload_type: parts[2].to_string(),
                    cluster: parts[0].to_string(),
                    vulnerabilities: vec![vulnerability.clone()],
                });
            }
        } else {
            eprintln!("Invalid workload tag format: {}", tag_name);
        }
    }
    None
}