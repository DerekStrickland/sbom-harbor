use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AggregateType {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "incomplete_first_party_only")]
    IncompleteFirstPartyOnly,
    #[serde(rename = "incomplete_third_party_only")]
    IncompleteThirdPartyOnly,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "not_specified")]
    NotSpecified,

}

impl ToString for AggregateType {
    fn to_string(&self) -> String {
        match self {
            Self::Complete => String::from("complete"),
            Self::Incomplete => String::from("incomplete"),
            Self::IncompleteFirstPartyOnly => String::from("incomplete_first_party_only"),
            Self::IncompleteThirdPartyOnly => String::from("incomplete_third_party_only"),
            Self::Unknown => String::from("unknown"),
            Self::NotSpecified => String::from("not_specified"),
        }
    }
}

impl Default for AggregateType {
    fn default() -> AggregateType {
        Self::Complete
    }
}




