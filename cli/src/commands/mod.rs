mod github;

use std::env;
use crate::commands::github::provider::GitHubProvider;
use crate::Error;
use async_trait::async_trait;
// mod snyk;

fn get_cf_domain() -> String {
    return match env::var("CF_DOMAIN") {
        Ok(v) => v,
        Err(e) => panic!("$CF_DOMAIN is not set ({})", e),
    };
}

#[async_trait]
pub trait Provider {
    async fn scan(&self);
}

pub enum PilotKind {
    GITHUB,
    SNYK,
}

// #[derive(Clone)]
pub struct PilotOpts {
    pub provider: PilotKind,
    pub output_format: Option<OutputFormat>,
    pub org: Option<String>,
}

impl Opts for PilotOpts {
    fn format(&self) -> OutputFormat {
        let format = self.output_format.clone();
        match format {
            None => OutputFormat::Text,
            Some(format) => format,
        }
    }
}

// TODO Should this be a trait?
pub struct PilotCommand {}

impl PilotCommand {
    pub async fn execute(_opts: PilotOpts) -> Result<(), Error> {

        let provider = PilotFactory::new(
            _opts
        );

        provider.scan().await;

        Ok(())
    }
}

pub struct PilotFactory {}

impl PilotFactory {
    pub fn new(pilot_ops: PilotOpts) -> Box<dyn Provider> {
        return match pilot_ops.provider {
            PilotKind::GITHUB => Box::new(GitHubProvider {}),
            PilotKind::SNYK => panic!("Jon, return SnykProvider implementation"),
        };
    }
}

/// Allows specifying the output format.
#[derive(Clone)]
pub enum OutputFormat {
    /// Output as JSON.
    Json,
    /// Output as plaintext.
    Text,
}

/// Generic trait that all command options must implement.
trait Opts {
    fn format(&self) -> OutputFormat {
        OutputFormat::Text
    }
}

/// Generic Command trait that all command handlers must implement.
trait Command<T>
where
    T: Opts + Send + Sync,
{
    fn execute(opts: T) -> i32;
}

/// Extracts the value of an environment variable
///
fn get_env_var(variable_name: &str) -> Option<String> {
    return match env::var(variable_name) {
        Ok(v) => Some(v),
        Err(e) => None,
    };
}