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
pub struct ExternalReference {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl ExternalReference {
    pub fn new() -> ExternalReference {
        ExternalReference {
            r#type: None,
            url: None,
            comment: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "vcs")]
    Vcs,
    #[serde(rename = "issue-tracker")]
    IssueTracker,
    #[serde(rename = "website")]
    Website,
    #[serde(rename = "advisories")]
    Advisories,
    #[serde(rename = "bom")]
    Bom,
    #[serde(rename = "mailing-list")]
    MailingList,
    #[serde(rename = "social")]
    Social,
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "documentation")]
    Documentation,
    #[serde(rename = "support")]
    Support,
    #[serde(rename = "source-distribution")]
    SourceDistribution,
    #[serde(rename = "distribution")]
    Distribution,
    #[serde(rename = "distribution-intake")]
    DistributionIntake,
    #[serde(rename = "license")]
    License,
    #[serde(rename = "build-meta")]
    BuildMeta,
    #[serde(rename = "build-system")]
    BuildSystem,
    #[serde(rename = "release-notes")]
    ReleaseNotes,
    #[serde(rename = "security-contact")]
    SecurityContact,
    #[serde(rename = "model_card")]
    ModelCard,
    #[serde(rename = "attestation")]
    Attestation,
    #[serde(rename = "threat-model")]
    ThreatModel,
    #[serde(rename = "adversary-model")]
    AdversaryModel,
    #[serde(rename = "risk-assessment")]
    RiskAssessment,
    #[serde(rename = "vulnerability-assertion")]
    VulnerabilityAssertion,
    #[serde(rename = "exploitability-statement")]
    ExploitabilityStatement,
    #[serde(rename = "pentest-report")]
    PentestReport,
    #[serde(rename = "static-analysis-report")]
    StaticAnalysisReport,
    #[serde(rename = "dynamic-analysis-report")]
    DynamicAnalysisReport,
    #[serde(rename = "runtime-analysis-report")]
    RuntimeAnalysisReport,
    #[serde(rename = "component-analysis-report")]
    ComponentAnalysisReport,
    #[serde(rename = "maturity-report")]
    MaturityReport,
    #[serde(rename = "certification-report")]
    CertificationReport,
    #[serde(rename = "codified-infrastructure")]
    CodifiedInfrastructure,
    #[serde(rename = "quality-metrics")]
    QualityMetrics,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "configuration")]
    Configuration,
    #[serde(rename = "evidence")]
    Evidence,
    #[serde(rename = "formulation")]
    Formulation,
    #[serde(rename = "rfc-9116")]
    Rfc9116,
    #[serde(rename = "electronic-signature")]
    ElectronicSignature,
    #[serde(rename = "digital-signature")]
    DigitalSignature,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Vcs
    }
}

