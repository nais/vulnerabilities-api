use dependencytrack::apis::configuration::Configuration;
use dependencytrack::apis::project_api::GetProjectsByTagError;
use dependencytrack::apis::{project_api, Error};
use dependencytrack::models::Project;

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


    pub async fn get_projects_by_tag(&self, tag: &str) -> Result<Vec<Project>, Error<GetProjectsByTagError>> {
        let projects = project_api::get_projects_by_tag(
            &self.config,
            tag,
            None, None, None, None, None, None, None, None,
        ).await?;

        Ok(projects)
    }
}

