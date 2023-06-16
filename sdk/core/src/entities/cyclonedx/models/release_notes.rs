use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReleaseNotes {
    /// The software versioning type. It is RECOMMENDED that the release type use one of 'major', 'minor', 'patch', 'pre-release', or 'internal'. Representing all possible software release types is not practical, so standardizing on the recommended values, whenever possible, is strongly encouraged.  * __major__ = A major release may contain significant changes or may introduce breaking changes. * __minor__ = A minor release, also known as an update, may contain a smaller number of changes than major releases. * __patch__ = Patch releases are typically unplanned and may resolve defects or important security issues. * __pre-release__ = A pre-release may include alpha, beta, or release candidates and typically have limited support. They provide the ability to preview a release prior to its general availability. * __internal__ = Internal releases are not for public consumption and are intended to be used exclusively by the project or manufacturer that produced it.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The title of the release.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL to an image that may be prominently displayed with the release note.
    #[serde(rename = "featuredImage", skip_serializing_if = "Option::is_none")]
    pub featured_image: Option<String>,
    /// The URL to an image that may be used in messaging on social media platforms.
    #[serde(rename = "socialImage", skip_serializing_if = "Option::is_none")]
    pub social_image: Option<String>,
    /// A short description of the release.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The date and time (timestamp) when the release note was created.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// One or more alternate names the release may be referred to. This may include unofficial terms used by development and marketing teams (e.g. code names).
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// One or more tags that may aid in search or retrieval of the release note.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// A collection of issues that have been resolved.
    #[serde(rename = "resolves", skip_serializing_if = "Option::is_none")]
    pub resolves: Option<Vec<crate::entities::cyclonedx::models::Issue>>,
    /// Zero or more release notes containing the locale and content. Multiple note objects may be specified to support release notes in a wide variety of languages.
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<crate::entities::cyclonedx::models::Note>>,
    /// Provides the ability to document properties in a name-value store. This provides flexibility to include data not officially supported in the standard without having to use additional namespaces or create extensions. Unlike key-value stores, properties support duplicate names, each potentially having different values. Property names of interest to the general public are encouraged to be registered in the [CycloneDX Property Taxonomy](https://github.com/CycloneDX/cyclonedx-property-taxonomy). Formal registration is OPTIONAL.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::entities::cyclonedx::models::Property>>,
}

impl ReleaseNotes {
    pub fn new(r#type: String) -> ReleaseNotes {
        ReleaseNotes {
            r#type,
            title: None,
            featured_image: None,
            social_image: None,
            description: None,
            timestamp: None,
            aliases: None,
            tags: None,
            resolves: None,
            notes: None,
            properties: None,
        }
    }
}