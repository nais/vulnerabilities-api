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
            eprintln!("Project \"{:?}\" has no metrics, skipping...", project.name);
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
            eprintln!(
                "Invalid workload tag format or namespace does not match: {}",
                tag_name
            );
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use dependencytrack::models::{Project, ProjectMetrics, Tag};

    // Mock project data with valid metrics and tags
    fn create_mock_project(
        name: &str,
        namespace: &str,
        metrics: Option<ProjectMetrics>,
        tags: Option<Vec<Tag>>,
    ) -> Project {
        Project {
            author: None,
            publisher: None,
            manufacturer: None,
            supplier: None,
            group: Some(namespace.to_string()),
            name: Some(name.to_string()),
            metrics: metrics.map(Box::new),
            tags,
            last_bom_import: None,
            last_bom_import_format: None,
            last_inherited_risk_score: None,
            active: None,
            external_references: None,
            metadata: None,
            uuid: uuid::Uuid::new_v4(),
            parent: None,
            children: None,
            description: Some("Description".to_string()),
            version: None,
            classifier: None,
            cpe: None,
            purl: None,
            swid_tag_id: None,
            direct_dependencies: None,
            properties: None,
            versions: None,
            bom_ref: None,
        }
    }

    fn create_mock_metrics() -> Option<ProjectMetrics> {
        Some(ProjectMetrics {
            project: None,
            critical: 1,
            high: 2,
            medium: 3,
            low: 4,
            unassigned: Some(5),
            vulnerabilities: None,
            vulnerable_components: None,
            components: None,
            suppressed: None,
            findings_total: None,
            findings_audited: None,
            findings_unaudited: None,
            inherited_risk_score: None,
            policy_violations_fail: None,
            policy_violations_warn: None,
            policy_violations_info: None,
            policy_violations_total: None,
            policy_violations_audited: None,
            policy_violations_unaudited: None,
            policy_violations_security_total: None,
            policy_violations_security_audited: None,
            policy_violations_security_unaudited: None,
            policy_violations_license_total: None,
            policy_violations_license_audited: None,
            policy_violations_license_unaudited: None,
            policy_violations_operational_total: None,
            policy_violations_operational_audited: None,
            policy_violations_operational_unaudited: None,
            first_occurrence: 0.0,
            last_occurrence: 0.0,
        })
    }

    // Test for valid workloads when a project has metrics and matching tags
    #[test]
    fn test_parse_workloads_valid() {
        let metrics = create_mock_metrics();

        let tag = Tag {
            name: Some("workload:cluster1|namespace1|type1|workload1".to_string()),
        };

        let project = create_mock_project("Test Project", "namespace1", metrics, Some(vec![tag]));

        let projects = vec![project];
        let namespace_name = "namespace1";
        let cluster = "cluster1";

        let reply = parse_workloads(projects, namespace_name, cluster);

        assert_eq!(reply.workload.len(), 1);
        assert_eq!(reply.workload[0].name, "workload1");
        assert_eq!(reply.workload[0].workload_type, "type1");
        assert_eq!(reply.workload[0].cluster, "cluster1");
        assert_eq!(reply.workload[0].vulnerabilities.len(), 1);
    }

    // Test for skipping projects with no metrics
    #[test]
    fn test_parse_workloads_no_metrics() {
        let project = create_mock_project(
            "Test Project",
            "namespace1",
            None,         // No metrics
            Some(vec![]), // No tags
        );

        let projects = vec![project];
        let namespace_name = "namespace1";
        let cluster = "cluster1";

        let reply = parse_workloads(projects, namespace_name, cluster);

        assert_eq!(reply.workload.len(), 0);
    }

    // Test for invalid tags in projects
    #[test]
    fn test_parse_workloads_invalid_tag() {
        let metrics = create_mock_metrics();

        let tag = Tag {
            name: Some("invalid_tag_format".to_string()), // Invalid tag
        };

        let project = create_mock_project("Test Project", "namespace1", metrics, Some(vec![tag]));

        let projects = vec![project];
        let namespace_name = "namespace1";
        let cluster = "cluster1";

        let reply = parse_workloads(projects, namespace_name, cluster);

        assert_eq!(reply.workload.len(), 0); // Invalid tags should result in no workloads
    }

    // Test for projects with tags that do not match the namespace or cluster
    #[test]
    fn test_parse_workloads_no_match() {
        let metrics = create_mock_metrics();

        let tag = Tag {
            name: Some("workload:cluster2|namespace2|type2|workload2".to_string()), // Non-matching /namespace
        };

        let project = create_mock_project(
            "Test Project",
            "namespace1", // Namespace doesn't match
            metrics,
            Some(vec![tag]),
        );

        let projects = vec![project];
        let namespace_name = "namespace1";
        let cluster = "cluster1";

        let reply = parse_workloads(projects, namespace_name, cluster);

        assert_eq!(reply.workload.len(), 0); // No match, no workloads
    }
}
