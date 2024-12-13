use serde::{Deserialize, Serialize};
use dependencytrack::apis::configuration::Configuration;
use dependencytrack::apis::{project_api, Error};
use dependencytrack::apis::project_api::GetProjectsByTagError;

struct Client {
    config: Configuration,
}

struct Workload {
    name: String,
    workload_type: String,

}

struct Metrics {
    severity: Severity
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "CRITICAL")]
    Critical,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "UNASSIGNED")]
    Unassigned,
}

impl Client {
    pub fn new(base_url: String, api_key: String) -> Client {
        let config = Configuration{
            base_path: base_url,
            user_agent: None,
            client: Default::default(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: Some(api_key.to_string()),
        };
        Client { config }
    }

    pub async fn getVulnerabilities(&self) -> Result<(), Error<GetProjectsByTagError>> {
        let projects = project_api::get_projects_by_tag(&self.config, "yolo", None, None, None, None, None, None, None, None).await?;
        Ok(())
    }
}