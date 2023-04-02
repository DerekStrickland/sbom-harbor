use serde::{Deserialize, Serialize};
/*
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Component {
    /// Specifies the type of component. For software components, classify as application if no more specific appropriate classification is available or cannot be determined for the component. Types include:  * __application__ = A software application. Refer to [https://en.wikipedia.org/wiki/Application_software](https://en.wikipedia.org/wiki/Application_software) for information about applications. * __framework__ = A software framework. Refer to [https://en.wikipedia.org/wiki/Software_framework](https://en.wikipedia.org/wiki/Software_framework) for information on how frameworks vary slightly from libraries. * __library__ = A software library. Refer to [https://en.wikipedia.org/wiki/Library_(computing)](https://en.wikipedia.org/wiki/Library_(computing))  for information about libraries. All third-party and open source reusable components will likely be a library. If the library also has key features of a framework, then it should be classified as a framework. If not, or is unknown, then specifying library is RECOMMENDED. * __container__ = A packaging and/or runtime format, not specific to any particular technology, which isolates software inside the container from software outside of a container through virtualization technology. Refer to [https://en.wikipedia.org/wiki/OS-level_virtualization](https://en.wikipedia.org/wiki/OS-level_virtualization) * __operating-system__ = A software operating system without regard to deployment model (i.e. installed on physical hardware, virtual machine, image, etc) Refer to [https://en.wikipedia.org/wiki/Operating_system](https://en.wikipedia.org/wiki/Operating_system) * __device__ = A hardware device such as a processor, or chip-set. A hardware device containing firmware SHOULD include a component for the physical hardware itself, and another component of type 'firmware' or 'operating-system' (whichever is relevant), describing information about the software running on the device. * __firmware__ = A special type of software that provides low-level control over a devices hardware. Refer to [https://en.wikipedia.org/wiki/Firmware](https://en.wikipedia.org/wiki/Firmware) * __file__ = A computer file. Refer to [https://en.wikipedia.org/wiki/Computer_file](https://en.wikipedia.org/wiki/Computer_file) for information about files.
    #[serde(rename = "type")]
    pub r#type: ComponentType,
    /// The optional mime-type of the component. When used on file components, the mime-type can provide additional context about the kind of file being represented such as an image, font, or executable. Some library or framework components may also have an associated mime-type.
    #[serde(rename = "mime-type", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "bom-ref", skip_serializing_if = "Option::is_none")]
    pub bom_ref: Option<String>,
    #[serde(rename = "supplier", skip_serializing_if = "Option::is_none")]
    pub supplier: Option<Box<crate::models::cyclonedx::OrganizationalEntity>>,
    /// The person(s) or organization(s) that authored the component
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// The person(s) or organization(s) that published the component
    #[serde(rename = "publisher", skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    /// The grouping name or identifier. This will often be a shortened, single name of the company or project that produced the component, or the source package or domain name. Whitespace and special characters should be avoided. Examples include: apache, org.apache.commons, and apache.org.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The name of the component. This will often be a shortened, single name of the component. Examples: commons-lang3 and jquery
    #[serde(rename = "name")]
    pub name: String,
    /// The component version. The version should ideally comply with semantic versioning but is not enforced.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Specifies a description for the component
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Specifies the scope of the component. If scope is not specified, 'required' scope SHOULD be assumed by the consumer of the BOM.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(rename = "hashes", skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<crate::models::cyclonedx::Hash>>,
    #[serde(rename = "licenses", skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<crate::models::cyclonedx::LicenseChoice>>,
    /// A copyright notice informing users of the underlying claims to copyright ownership in a published work.
    #[serde(rename = "copyright", skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    /// Specifies a well-formed CPE name that conforms to the CPE 2.2 or 2.3 specification. See [https://nvd.nist.gov/products/cpe](https://nvd.nist.gov/products/cpe)
    #[serde(rename = "cpe", skip_serializing_if = "Option::is_none")]
    pub cpe: Option<String>,
    /// Specifies the package-url (purl). The purl, if specified, MUST be valid and conform to the specification defined at: [https://github.com/package-url/purl-spec](https://github.com/package-url/purl-spec)
    #[serde(rename = "purl", skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,
    #[serde(rename = "swid", skip_serializing_if = "Option::is_none")]
    pub swid: Option<Box<crate::models::cyclonedx::Swid>>,
    /// [Deprecated] - DO NOT USE. This will be removed in a future version. Use the pedigree element instead to supply information on exactly how the component was modified. A boolean value indicating if the component has been modified from the original. A value of true indicates the component is a derivative of the original. A value of false indicates the component has not been modified from the original.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<bool>,
    #[serde(rename = "pedigree", skip_serializing_if = "Option::is_none")]
    pub pedigree: Option<Box<crate::models::cyclonedx::ComponentPedigree>>,
    /// External references provide a way to document systems, sites, and information that may be relevant but which are not included with the BOM.
    #[serde(rename = "externalReferences", skip_serializing_if = "Option::is_none")]
    pub external_references: Option<Vec<crate::models::cyclonedx::ExternalReference>>,
    /// A list of software and hardware components included in the parent component. This is not a dependency tree. It provides a way to specify a hierarchical representation of component assemblies, similar to system &#8594; subsystem &#8594; parts assembly in physical supply chains.
    #[serde(rename = "components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<crate::models::cyclonedx::Component>>,
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Box<crate::models::cyclonedx::ComponentEvidence>>,
    #[serde(rename = "releaseNotes", skip_serializing_if = "Option::is_none")]
    pub release_notes: Option<Box<crate::models::cyclonedx::ReleaseNotes>>,
    /// Provides the ability to document properties in a name-value store. This provides flexibility to include data not officially supported in the standard without having to use additional namespaces or create extensions. Unlike key-value stores, properties support duplicate names, each potentially having different values. Property names of interest to the general public are encouraged to be registered in the [CycloneDX Property Taxonomy](https://github.com/CycloneDX/cyclonedx-property-taxonomy). Formal registration is OPTIONAL.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::cyclonedx::Property>>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<serde_json::Value>,
}

impl Component {
    pub fn new(r#type: ComponentType, name: String) -> Component {
        Component {
            r#type,
            mime_type: None,
            bom_ref: None,
            supplier: None,
            author: None,
            publisher: None,
            group: None,
            name,
            version: None,
            description: None,
            scope: None,
            hashes: None,
            licenses: None,
            copyright: None,
            cpe: None,
            purl: None,
            swid: None,
            modified: None,
            pedigree: None,
            external_references: None,
            components: None,
            evidence: None,
            release_notes: None,
            properties: None,
            signature: None,
        }
    }
}

/// Specifies the type of component. For software components, classify as application if no more specific appropriate classification is available or cannot be determined for the component. Types include:  * __application__ = A software application. Refer to [https://en.wikipedia.org/wiki/Application_software](https://en.wikipedia.org/wiki/Application_software) for information about applications. * __framework__ = A software framework. Refer to [https://en.wikipedia.org/wiki/Software_framework](https://en.wikipedia.org/wiki/Software_framework) for information on how frameworks vary slightly from libraries. * __library__ = A software library. Refer to [https://en.wikipedia.org/wiki/Library_(computing)](https://en.wikipedia.org/wiki/Library_(computing))  for information about libraries. All third-party and open source reusable components will likely be a library. If the library also has key features of a framework, then it should be classified as a framework. If not, or is unknown, then specifying library is RECOMMENDED. * __container__ = A packaging and/or runtime format, not specific to any particular technology, which isolates software inside the container from software outside of a container through virtualization technology. Refer to [https://en.wikipedia.org/wiki/OS-level_virtualization](https://en.wikipedia.org/wiki/OS-level_virtualization) * __operating-system__ = A software operating system without regard to deployment model (i.e. installed on physical hardware, virtual machine, image, etc) Refer to [https://en.wikipedia.org/wiki/Operating_system](https://en.wikipedia.org/wiki/Operating_system) * __device__ = A hardware device such as a processor, or chip-set. A hardware device containing firmware SHOULD include a component for the physical hardware itself, and another component of type 'firmware' or 'operating-system' (whichever is relevant), describing information about the software running on the device. * __firmware__ = A special type of software that provides low-level control over a devices hardware. Refer to [https://en.wikipedia.org/wiki/Firmware](https://en.wikipedia.org/wiki/Firmware) * __file__ = A computer file. Refer to [https://en.wikipedia.org/wiki/Computer_file](https://en.wikipedia.org/wiki/Computer_file) for information about files.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    #[serde(rename = "application")]
    Application,
    #[serde(rename = "framework")]
    Framework,
    #[serde(rename = "library")]
    Library,
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "operating-system")]
    OperatingSystem,
    #[serde(rename = "device")]
    Device,
    #[serde(rename = "firmware")]
    Firmware,
    #[serde(rename = "file")]
    File,
}

impl Default for ComponentType {
    fn default() -> ComponentType {
        Self::Application
    }
}
/// Specifies the scope of the component. If scope is not specified, 'required' scope SHOULD be assumed by the consumer of the BOM.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    Optional,
    #[serde(rename = "excluded")]
    Excluded,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Required
    }
}

