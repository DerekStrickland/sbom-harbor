mod finding;
mod sbom;
mod scan;

pub use finding::*;
pub use sbom::*;
pub use scan::*;

use crate::entities::xrefs::{Xref, XrefKind, Xrefs};
use crate::xref;
use platform::mongodb::{mongo_doc, MongoDocument};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

mongo_doc!(Sbom);
xref!(Sbom);
xref!(Finding);

/// Discriminator that indicates how an SBOM was generated.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SbomProviderKind {
    /// SBOM generated by Harbor.
    GitHub,
    /// Package was extracted from an SBOM generated by Snyk.
    Snyk { api_version: String },
}

impl Display for SbomProviderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SbomProviderKind::GitHub => write!(f, "github"),
            SbomProviderKind::Snyk(api_version) => write!(f, "snyk::{}", api_version),
        }
    }
}
