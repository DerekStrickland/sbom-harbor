use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Compositions {
    #[serde(rename = "aggregate")]
    pub aggregate: crate::models::cyclonedx::AggregateType,
    /// The bom-ref identifiers of the components or services being described. Assemblies refer to nested relationships whereby a constituent part may include other constituent parts. References do not cascade to child parts. References are explicit for the specified constituent part only.
    #[serde(rename = "assemblies", skip_serializing_if = "Option::is_none")]
    pub assemblies: Option<Vec<String>>,
    /// The bom-ref identifiers of the components or services being described. Dependencies refer to a relationship whereby an independent constituent part requires another independent constituent part. References do not cascade to transitive dependencies. References are explicit for the specified dependency only.
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<serde_json::Value>,
}

impl Compositions {
    pub fn new(aggregate: crate::models::cyclonedx::AggregateType) -> Compositions {
        Compositions {
            aggregate,
            assemblies: None,
            dependencies: None,
            signature: None,
        }
    }
}


