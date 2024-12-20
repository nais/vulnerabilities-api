use crate::vulnerabilities;
use dependencytrack::models::Project;
use vulnerabilities::{VulnerabilityMetrics, Workload, WorkloadMetricReply};

pub fn parse_workloads(
    projects: Vec<Project>,
    namespace_name: &str,
    cluster: &str,
) -> WorkloadMetricReply {
    println!("Workloads for namespace: {}", namespace_name);

    let mut reply: WorkloadMetricReply = WorkloadMetricReply::default();

    for project in projects {
        let workloads = process_project(&project, namespace_name, cluster);
        reply.workload.extend(workloads);
    }

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

    let mut workloads = Vec::new();

    // Process tags
    project.tags.iter().for_each(|tags| {
        tags.iter().for_each(|tag| {
            if let Some(tag_name) = &tag.name {
                if let Some(workload) =
                    process_tag(tag_name, namespace_name, cluster, &vulnerability)
                {
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
    if let Some(stripped_prefix) = tag_name.strip_prefix("workload:") {
        let parts: Vec<&str> = stripped_prefix.split('|').collect();
        if parts.len() == 4 && parts[1] == namespace_name {
            // Match on the cluster or namespace if cluster is provided
            if cluster.is_empty() || parts[0] == cluster {
                return Some(Workload {
                    name: parts[3].to_string(),
                    workload_type: parts[2].to_string(),
                    cluster: parts[0].to_string(),
                    vulnerabilities: vec![*vulnerability],
                });
            }
        } else {
            eprintln!("Invalid workload tag format: {}", tag_name);
        }
    }
    None
}
