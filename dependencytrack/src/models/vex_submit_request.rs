/*
 * Dependency-Track API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 4.11.7
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VexSubmitRequest {
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "projectName", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "projectVersion", skip_serializing_if = "Option::is_none")]
    pub project_version: Option<String>,
    #[serde(rename = "vex")]
    pub vex: String,
}

impl VexSubmitRequest {
    pub fn new(project: String, vex: String) -> VexSubmitRequest {
        VexSubmitRequest {
            project,
            project_name: None,
            project_version: None,
            vex,
        }
    }
}

