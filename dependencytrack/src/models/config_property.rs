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
pub struct ConfigProperty {
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "propertyName", skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
    #[serde(rename = "propertyValue", skip_serializing_if = "Option::is_none")]
    pub property_value: Option<String>,
    #[serde(rename = "propertyType")]
    pub property_type: PropertyType,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ConfigProperty {
    pub fn new(property_type: PropertyType) -> ConfigProperty {
        ConfigProperty {
            group_name: None,
            property_name: None,
            property_value: None,
            property_type,
            description: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "ENCRYPTEDSTRING")]
    Encryptedstring,
    #[serde(rename = "TIMESTAMP")]
    Timestamp,
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "UUID")]
    Uuid,
}

impl Default for PropertyType {
    fn default() -> PropertyType {
        Self::Boolean
    }
}
