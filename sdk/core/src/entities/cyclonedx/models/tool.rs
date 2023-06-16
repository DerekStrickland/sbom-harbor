use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

/// Tool : Information about the automated or manual tool used
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Tool {
    /// The name of the vendor who created the tool
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    /// The name of the tool
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The version of the tool
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The hashes of the tool (if applicable).
    #[serde(rename = "hashes", skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<crate::entities::cyclonedx::models::Hash>>,
    /// External references provide a way to document systems, sites, and information that may be relevant but which are not included with the BOM.
    #[serde(rename = "externalReferences", skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<crate::entities::cyclonedx::models::ExternalReference>>,
}

impl Tool {
    /// Information about the automated or manual tool used
    pub fn new() -> Tool {
        Tool {
            vendor: None,
            name: None,
            version: None,
            hashes: None,
            external_references: None,
        }
    }
}