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
pub struct BomSubmitRequest {
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "projectName", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "projectVersion", skip_serializing_if = "Option::is_none")]
    pub project_version: Option<String>,
    #[serde(rename = "autoCreate", skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
    #[serde(rename = "parentUUID", skip_serializing_if = "Option::is_none")]
    pub parent_uuid: Option<String>,
    #[serde(rename = "parentName", skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[serde(rename = "parentVersion", skip_serializing_if = "Option::is_none")]
    pub parent_version: Option<String>,
    /// Base64 encoded BOM
    #[serde(rename = "bom")]
    pub bom: String,
}

impl BomSubmitRequest {
    pub fn new(project: String, bom: String) -> BomSubmitRequest {
        BomSubmitRequest {
            project,
            project_name: None,
            project_version: None,
            auto_create: None,
            parent_uuid: None,
            parent_name: None,
            parent_version: None,
            bom,
        }
    }
}
