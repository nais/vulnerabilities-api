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
pub struct ProjectMetrics {
    #[serde(rename = "project")]
    pub project: Box<models::Project>,
    #[serde(rename = "critical")]
    pub critical: i32,
    #[serde(rename = "high")]
    pub high: i32,
    #[serde(rename = "medium")]
    pub medium: i32,
    #[serde(rename = "low")]
    pub low: i32,
    #[serde(rename = "unassigned", skip_serializing_if = "Option::is_none")]
    pub unassigned: Option<i32>,
    #[serde(rename = "vulnerabilities", skip_serializing_if = "Option::is_none")]
    pub vulnerabilities: Option<i64>,
    #[serde(rename = "vulnerableComponents", skip_serializing_if = "Option::is_none")]
    pub vulnerable_components: Option<i32>,
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<i32>,
    #[serde(rename = "suppressed", skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<i32>,
    #[serde(rename = "findingsTotal", skip_serializing_if = "Option::is_none")]
    pub findings_total: Option<i32>,
    #[serde(rename = "findingsAudited", skip_serializing_if = "Option::is_none")]
    pub findings_audited: Option<i32>,
    #[serde(rename = "findingsUnaudited", skip_serializing_if = "Option::is_none")]
    pub findings_unaudited: Option<i32>,
    #[serde(rename = "inheritedRiskScore", skip_serializing_if = "Option::is_none")]
    pub inherited_risk_score: Option<f64>,
    #[serde(rename = "policyViolationsFail", skip_serializing_if = "Option::is_none")]
    pub policy_violations_fail: Option<i32>,
    #[serde(rename = "policyViolationsWarn", skip_serializing_if = "Option::is_none")]
    pub policy_violations_warn: Option<i32>,
    #[serde(rename = "policyViolationsInfo", skip_serializing_if = "Option::is_none")]
    pub policy_violations_info: Option<i32>,
    #[serde(rename = "policyViolationsTotal", skip_serializing_if = "Option::is_none")]
    pub policy_violations_total: Option<i32>,
    #[serde(rename = "policyViolationsAudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_audited: Option<i32>,
    #[serde(rename = "policyViolationsUnaudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_unaudited: Option<i32>,
    #[serde(rename = "policyViolationsSecurityTotal", skip_serializing_if = "Option::is_none")]
    pub policy_violations_security_total: Option<i32>,
    #[serde(rename = "policyViolationsSecurityAudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_security_audited: Option<i32>,
    #[serde(rename = "policyViolationsSecurityUnaudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_security_unaudited: Option<i32>,
    #[serde(rename = "policyViolationsLicenseTotal", skip_serializing_if = "Option::is_none")]
    pub policy_violations_license_total: Option<i32>,
    #[serde(rename = "policyViolationsLicenseAudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_license_audited: Option<i32>,
    #[serde(rename = "policyViolationsLicenseUnaudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_license_unaudited: Option<i32>,
    #[serde(rename = "policyViolationsOperationalTotal", skip_serializing_if = "Option::is_none")]
    pub policy_violations_operational_total: Option<i32>,
    #[serde(rename = "policyViolationsOperationalAudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_operational_audited: Option<i32>,
    #[serde(rename = "policyViolationsOperationalUnaudited", skip_serializing_if = "Option::is_none")]
    pub policy_violations_operational_unaudited: Option<i32>,
    #[serde(rename = "firstOccurrence")]
    pub first_occurrence: f64,
    #[serde(rename = "lastOccurrence")]
    pub last_occurrence: f64,
}

impl ProjectMetrics {
    pub fn new(project: models::Project, critical: i32, high: i32, medium: i32, low: i32, first_occurrence: f64, last_occurrence: f64) -> ProjectMetrics {
        ProjectMetrics {
            project: Box::new(project),
            critical,
            high,
            medium,
            low,
            unassigned: None,
            vulnerabilities: None,
            vulnerable_components: None,
            components: None,
            suppressed: None,
            findings_total: None,
            findings_audited: None,
            findings_unaudited: None,
            inherited_risk_score: None,
            policy_violations_fail: None,
            policy_violations_warn: None,
            policy_violations_info: None,
            policy_violations_total: None,
            policy_violations_audited: None,
            policy_violations_unaudited: None,
            policy_violations_security_total: None,
            policy_violations_security_audited: None,
            policy_violations_security_unaudited: None,
            policy_violations_license_total: None,
            policy_violations_license_audited: None,
            policy_violations_license_unaudited: None,
            policy_violations_operational_total: None,
            policy_violations_operational_audited: None,
            policy_violations_operational_unaudited: None,
            first_occurrence,
            last_occurrence,
        }
    }
}

