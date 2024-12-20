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
pub struct NotificationRule {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "notifyChildren", skip_serializing_if = "Option::is_none")]
    pub notify_children: Option<bool>,
    #[serde(rename = "logSuccessfulPublish", skip_serializing_if = "Option::is_none")]
    pub log_successful_publish: Option<bool>,
    #[serde(rename = "scope")]
    pub scope: Scope,
    #[serde(rename = "notificationLevel", skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<NotificationLevel>,
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<models::Project>>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<models::Team>>,
    #[serde(rename = "notifyOn", skip_serializing_if = "Option::is_none")]
    pub notify_on: Option<std::collections::HashSet<NotifyOn>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Box<models::NotificationPublisher>>,
    #[serde(rename = "publisherConfig", skip_serializing_if = "Option::is_none")]
    pub publisher_config: Option<String>,
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl NotificationRule {
    pub fn new(name: String, scope: Scope, uuid: uuid::Uuid) -> NotificationRule {
        NotificationRule {
            name,
            enabled: None,
            notify_children: None,
            log_successful_publish: None,
            scope,
            notification_level: None,
            projects: None,
            teams: None,
            notify_on: None,
            message: None,
            publisher: None,
            publisher_config: None,
            uuid,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "PORTFOLIO")]
    Portfolio,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::System
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotificationLevel {
    #[serde(rename = "INFORMATIONAL")]
    Informational,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "ERROR")]
    Error,
}

impl Default for NotificationLevel {
    fn default() -> NotificationLevel {
        Self::Informational
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NotifyOn {
    #[serde(rename = "CONFIGURATION")]
    Configuration,
    #[serde(rename = "DATASOURCE_MIRRORING")]
    DatasourceMirroring,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "INTEGRATION")]
    Integration,
    #[serde(rename = "INDEXING_SERVICE")]
    IndexingService,
    #[serde(rename = "FILE_SYSTEM")]
    FileSystem,
    #[serde(rename = "ANALYZER")]
    Analyzer,
    #[serde(rename = "NEW_VULNERABILITY")]
    NewVulnerability,
    #[serde(rename = "NEW_VULNERABLE_DEPENDENCY")]
    NewVulnerableDependency,
    #[serde(rename = "PROJECT_AUDIT_CHANGE")]
    ProjectAuditChange,
    #[serde(rename = "BOM_CONSUMED")]
    BomConsumed,
    #[serde(rename = "BOM_PROCESSED")]
    BomProcessed,
    #[serde(rename = "BOM_PROCESSING_FAILED")]
    BomProcessingFailed,
    #[serde(rename = "VEX_CONSUMED")]
    VexConsumed,
    #[serde(rename = "VEX_PROCESSED")]
    VexProcessed,
    #[serde(rename = "POLICY_VIOLATION")]
    PolicyViolation,
    #[serde(rename = "PROJECT_CREATED")]
    ProjectCreated,
    #[serde(rename = "USER_CREATED")]
    UserCreated,
    #[serde(rename = "USER_DELETED")]
    UserDeleted,
}

impl Default for NotifyOn {
    fn default() -> NotifyOn {
        Self::Configuration
    }
}

