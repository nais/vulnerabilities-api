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
pub struct MappedOidcGroupRequest {
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

impl MappedOidcGroupRequest {
    pub fn new() -> MappedOidcGroupRequest {
        MappedOidcGroupRequest {
            team: None,
            group: None,
        }
    }
}

