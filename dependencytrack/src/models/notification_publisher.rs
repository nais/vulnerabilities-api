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
pub struct NotificationPublisher {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "publisherClass")]
    pub publisher_class: String,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(rename = "templateMimeType")]
    pub template_mime_type: String,
    #[serde(rename = "defaultPublisher", skip_serializing_if = "Option::is_none")]
    pub default_publisher: Option<bool>,
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl NotificationPublisher {
    pub fn new(name: String, publisher_class: String, template_mime_type: String, uuid: uuid::Uuid) -> NotificationPublisher {
        NotificationPublisher {
            name,
            description: None,
            publisher_class,
            template: None,
            template_mime_type,
            default_publisher: None,
            uuid,
        }
    }
}

