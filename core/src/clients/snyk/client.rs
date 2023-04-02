use std::fmt::{Display, Formatter};
use tracing::debug;

use platform::hyper;
use platform::hyper::ContentType;
use urlencoding::encode;

use crate::clients::snyk::models::{CommonIssueModel, Group, IssuesResponse, ListOrgProjects200Response, ListOrgProjects200ResponseDataInner, Org, OrgsResponse, OrgV1, SbomResource, SbomResponse};
use crate::Error;

const V1_URL:&str = "https://snyk.io/api/v1";
const V3_URL:&str = "https://api.snyk.io/rest";
const ORGS_ROUTE: &str = "/orgs";
const API_VERSION: &str = "2023-03-08~beta"; // "2023-03-29"

pub enum SbomFormat {
    CycloneDxJson,
    CycloneDxXml,
    SpdxJson,
}

impl Display for SbomFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SbomFormat::CycloneDxJson => write!(f, "{}", encode("cyclonedx+json")),
            SbomFormat::CycloneDxXml => write!(f, "{}", encode("cyclonedx+xml")),
            SbomFormat::SpdxJson => write!(f, "{}", encode("spdx2.3+json")),
        }
    }
}

#[allow(dead_code)]
fn orgs_url() -> String {
    format!("{}{}", V1_URL, ORGS_ROUTE)
}

#[allow(dead_code)]
fn issues_url(org_id: &str, purl: &str) -> String {
    let route = format!("/orgs/{}/packages/{}/issues?version={}", org_id, encode(purl), API_VERSION);
    format!("{}{}", V3_URL, route)
}

#[allow(dead_code)]
fn projects_url(org_id: &str) -> String {
    let route = format!("/orgs/{}/projects?version={}", org_id, API_VERSION);
    format!("{}{}", V3_URL, route)
}

#[allow(dead_code)]
fn sbom_url(org_id: &str, project_id: &str, format: SbomFormat) -> String {
    let route = format!("/orgs/{}/projects/{}/sbom?version={}&format={}", org_id, project_id, API_VERSION, format);
    format!("{}{}", V3_URL, route)
}

/// A purpose build Snyk HTTP Client.
pub(crate) struct Client {
    token: String,
}

impl Client {
    /// Factory method for creating new instances of a Client.
    pub fn new(token: String) -> Self {
        Self{token}
    }

    fn token(&self) -> String {
        format!("token {}", self.token)
    }

    #[allow(dead_code)]
    pub async fn orgs(&self) -> Result<Option<Vec<OrgV1>>, Error> {
        let response:Option<OrgsResponse> = hyper::get(
            &orgs_url(),
            ContentType::Json,
            &self.token(),
            None::<OrgsResponse>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list orgs".to_string())),
            Some(r) => Ok(r.orgs),
        }

    }

    #[allow(dead_code)]
    pub async fn projects(&self, org_id: &str) -> Result<Option<Vec<ListOrgProjects200ResponseDataInner>>, Error> {
        let response:Option<ListOrgProjects200Response> = hyper::get(
            &projects_url(org_id),
            ContentType::Json,
            &self.token(),
            None::<ListOrgProjects200Response>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to list projects".to_string())),
            Some(r) => Ok(r.data),
        }
    }

    #[allow(dead_code)]
    pub async fn sbom(&self, org_id: &str, project_id: &str, format: SbomFormat) -> Result<Option<String>, Error> {
        let response = hyper::raw(
            hyper::Method::GET,
            &sbom_url(org_id, project_id, format),
            ContentType::Json,
            self.token(),
            None::<String>).await?;

        if response.0 != hyper::StatusCode::OK {
            return Err(Error::Runtime(format!("remote returned error: {}", response.1)));
        }

        if response.1.is_empty() {
            return Ok(None);
        }

        Ok(Some(response.1))
    }

    #[allow(dead_code)]
    pub async fn get_issues(&self, org_id: &str, purl: &str) -> Result<Option<Vec<CommonIssueModel>>, Error> {
        // println!("getting issues for purl: {}", purl);

        let response:Option<IssuesResponse> = hyper::get(
            &issues_url(org_id, purl),
                ContentType::Json,
            &self.token(),
            None::<IssuesResponse>).await?;

        match response {
            None => Err(Error::Runtime("snyk failed to return issues".to_string())),
            Some(r) => Ok(r.data),
        }
    }
}

