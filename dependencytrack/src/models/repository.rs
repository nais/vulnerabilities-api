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
pub struct Repository {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "resolutionOrder")]
    pub resolution_order: i32,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "internal")]
    pub internal: bool,
    #[serde(rename = "authenticationRequired", skip_serializing_if = "Option::is_none")]
    pub authentication_required: Option<bool>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
}

impl Repository {
    pub fn new(r#type: Type, resolution_order: i32, enabled: bool, internal: bool, uuid: uuid::Uuid) -> Repository {
        Repository {
            r#type,
            identifier: None,
            url: None,
            resolution_order,
            enabled,
            internal,
            authentication_required: None,
            username: None,
            uuid,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CPAN")]
    Cpan,
    #[serde(rename = "MAVEN")]
    Maven,
    #[serde(rename = "NPM")]
    Npm,
    #[serde(rename = "GEM")]
    Gem,
    #[serde(rename = "PYPI")]
    Pypi,
    #[serde(rename = "NUGET")]
    Nuget,
    #[serde(rename = "HEX")]
    Hex,
    #[serde(rename = "COMPOSER")]
    Composer,
    #[serde(rename = "CARGO")]
    Cargo,
    #[serde(rename = "GO_MODULES")]
    GoModules,
    #[serde(rename = "GITHUB")]
    Github,
    #[serde(rename = "HACKAGE")]
    Hackage,
    #[serde(rename = "NIXPKGS")]
    Nixpkgs,
    #[serde(rename = "UNSUPPORTED")]
    Unsupported,
}

impl Default for Type {
    fn default() -> Type {
        Self::Cpan
    }
}
