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
pub struct PolicyViolation {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<models::Project>>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<Box<models::Component>>,
    #[serde(rename = "policyCondition", skip_serializing_if = "Option::is_none")]
    pub policy_condition: Option<Box<models::PolicyCondition>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "analysis", skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Box<models::ViolationAnalysis>>,
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl PolicyViolation {
    pub fn new(uuid: uuid::Uuid) -> PolicyViolation {
        PolicyViolation {
            r#type: None,
            project: None,
            component: None,
            policy_condition: None,
            timestamp: None,
            text: None,
            analysis: None,
            uuid,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "LICENSE")]
    License,
    #[serde(rename = "SECURITY")]
    Security,
    #[serde(rename = "OPERATIONAL")]
    Operational,
}

impl Default for Type {
    fn default() -> Type {
        Self::License
    }
}
