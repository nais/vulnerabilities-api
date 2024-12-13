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
pub struct Finding {
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "vulnerability", skip_serializing_if = "Option::is_none")]
    pub vulnerability: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "attribution", skip_serializing_if = "Option::is_none")]
    pub attribution: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "matrix", skip_serializing_if = "Option::is_none")]
    pub matrix: Option<String>,
}

impl Finding {
    pub fn new() -> Finding {
        Finding {
            component: None,
            vulnerability: None,
            analysis: None,
            attribution: None,
            matrix: None,
        }
    }
}

