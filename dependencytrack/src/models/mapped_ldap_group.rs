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
pub struct MappedLdapGroup {
    #[serde(rename = "dn", skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl MappedLdapGroup {
    pub fn new(uuid: uuid::Uuid) -> MappedLdapGroup {
        MappedLdapGroup {
            dn: None,
            uuid,
        }
    }
}
