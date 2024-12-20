use crate::vulnerabilities::{
    Cwe, Severity, VulnerabilityAlias, VulnerabilityDetails, WorkloadVulnerabilityDetailsReply,
};
use dependencytrack::apis::configuration::Configuration;
use dependencytrack::apis::{project_api, vulnerability_api};
use dependencytrack::models::Project;
use tonic::Status;

pub struct Client {
    config: Configuration,
}

impl Client {
    pub fn new(base_url: String, api_key: String) -> Client {
        let config = Configuration {
            base_path: base_url.to_owned(),
            user_agent: None,
            client: Default::default(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: Some(dependencytrack::apis::configuration::ApiKey {
                prefix: None,
                key: api_key,
            }),
        };

        Client { config }
    }

    pub async fn get_projects_by_tag(&self, tag: &str) -> Result<Vec<Project>, Status> {
        let mut all_projects = Vec::new();
        let mut page_number = 1;
        let page_size = 100;

        loop {
            let projects = match project_api::get_projects_by_tag(
                &self.config,
                tag,
                None,
                None,
                Some(page_number.into()),
                Some(page_size.into()),
                None,
                None,
                None,
                None,
            )
            .await
            {
                Ok(projects) => projects,
                Err(e) => {
                    eprintln!("Failed to fetch projects for tag {}: {}", tag, e);
                    return Err(Status::internal(format!("Request failed: {}", e)));
                }
            };

            // Exit the loop if no more projects are returned
            if projects.is_empty() {
                break;
            }

            all_projects.extend(projects);
            page_number += 1;
        }

        Ok(all_projects)
    }

    pub async fn get_vulnerability_details_for_workload(
        &self,
        workload: &str,
        workload_type: &str,
        namespace: &str,
        cluster: &str,
    ) -> Result<WorkloadVulnerabilityDetailsReply, Status> {
        let tag = format!(
            "workload:{}|{}|{}|{}",
            cluster, namespace, workload_type, workload
        );

        let projects = match self.get_projects_by_tag(&tag).await {
            Ok(projects) => projects,
            Err(e) => {
                eprintln!("Failed to fetch projects for tag {}: {}", tag, e);
                return Err(Status::internal(format!("Failed to fetch projects: {}", e)));
            }
        };

        match self.get_vulnerabilities(projects).await {
            Ok(reply) => Ok(reply),
            Err(e) => Err(e),
        }
    }

    async fn get_vulnerabilities(
        &self,
        projects: Vec<Project>,
    ) -> Result<WorkloadVulnerabilityDetailsReply, Status> {
        let mut details = Vec::new();

        for project in projects {
            match vulnerability_api::get_vulnerabilities_by_project(
                &self.config,
                &project.uuid.to_string(),
                None,
            )
            .await
            {
                Ok(vulnerabilities) => {
                    for vuln in vulnerabilities {
                        let detail = VulnerabilityDetails {
                            title: vuln.title.unwrap_or_else(|| "Unknown".to_string()),
                            vuln_id: vuln.vuln_id.unwrap_or_else(|| "N/A".to_string()),
                            source: vuln.source.unwrap_or_else(|| "Unknown".to_string()),
                            description: vuln
                                .description
                                .unwrap_or_else(|| "No description available".to_string()),
                            detail: vuln
                                .detail
                                .unwrap_or_else(|| "No details available".to_string()),
                            recommendation: vuln
                                .recommendation
                                .unwrap_or_else(|| "No recommendations provided.".to_string()),
                            created: vuln.created.unwrap_or_else(|| "Unknown".to_string()),
                            published: vuln.published.unwrap_or_else(|| "Unknown".to_string()),
                            updated: vuln.updated.unwrap_or_else(|| "Unknown".to_string()),
                            cwes: vuln
                                .cwes
                                .unwrap_or_default()
                                .into_iter()
                                .map(|cwe| Cwe {
                                    cwe_id: cwe.cwe_id.unwrap_or(0),
                                    cwe_name: cwe.name.unwrap_or_else(|| "Unknown".to_string()),
                                })
                                .collect(),
                            severity: vuln
                                .severity
                                .map(|s| s as i32)
                                .unwrap_or(Severity::Unassigned as i32),
                            aliases: vuln
                                .aliases
                                .unwrap_or_default()
                                .into_iter()
                                .map(|alias| VulnerabilityAlias {
                                    cve_id: alias.cve_id.unwrap_or_default(),
                                    ghsa_id: alias.ghsa_id.unwrap_or_default(),
                                    osv_id: alias.osv_id.unwrap_or_default(),
                                    gsd_id: alias.gsd_id.unwrap_or_default(),
                                })
                                .collect(),
                        };
                        details.push(detail);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch vulnerabilities: {}", e);
                    return Err(Status::internal(format!(
                        "Failed to fetch vulnerabilities: {}",
                        e
                    )));
                }
            }
        }

        Ok(WorkloadVulnerabilityDetailsReply {
            vulnerability_details: details,
        })
    }
}
