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
pub struct ViolationAnalysis {
    #[serde(rename = "analysisState")]
    pub analysis_state: AnalysisState,
    #[serde(rename = "analysisComments", skip_serializing_if = "Option::is_none")]
    pub analysis_comments: Option<Vec<models::ViolationAnalysisComment>>,
    #[serde(rename = "isSuppressed", skip_serializing_if = "Option::is_none")]
    pub is_suppressed: Option<bool>,
}

impl ViolationAnalysis {
    pub fn new(analysis_state: AnalysisState) -> ViolationAnalysis {
        ViolationAnalysis {
            analysis_state,
            analysis_comments: None,
            is_suppressed: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AnalysisState {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "NOT_SET")]
    NotSet,
}

impl Default for AnalysisState {
    fn default() -> AnalysisState {
        Self::Approved
    }
}
