mod sbom;
pub use sbom::*;

use crate::entities::xrefs::{Xref, Xrefs};
use crate::xref;
use platform::persistence::mongodb::{mongo_doc, MongoDocument};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

mongo_doc!(Sbom);
xref!(Sbom);

/// Discriminator that indicates how an SBOM was generated.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SbomProviderKind {
    /// SBOM generated by Harbor.
    GitHub,
    // TODO: Seems good to know the API version, but also creates rich text problems by embedding
    // it here.
    /// Package was extracted from an SBOM generated by Snyk.
    Snyk,
    /// SBOM generated and submitted by Vendor.
    Vendor(String),
}

impl Display for SbomProviderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SbomProviderKind::GitHub => write!(f, "github"),
            SbomProviderKind::Snyk => write!(f, "snyk"),
            SbomProviderKind::Vendor(name) => write!(f, "vendor::{}", name.to_lowercase()),
        }
    }
}