use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

/// Source : The source of the issue where it is documented
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Source {
    /// The name of the source. For example 'National Vulnerability Database', 'NVD', and 'Apache'
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The url of the issue documentation as provided by the source
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Source {
    /// The source of the issue where it is documented
    pub fn new() -> Source {
        Source {
            name: None,
            url: None,
        }
    }
}


