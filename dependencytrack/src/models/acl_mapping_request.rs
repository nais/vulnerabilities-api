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
pub struct AclMappingRequest {
    #[serde(rename = "team")]
    pub team: String,
    #[serde(rename = "project")]
    pub project: String,
}

impl AclMappingRequest {
    pub fn new(team: String, project: String) -> AclMappingRequest {
        AclMappingRequest {
            team,
            project,
        }
    }
}

