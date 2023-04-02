mod github;

use std::env;
use crate::commands::github::provider::GitHubProvider;
use crate::Error;
use async_trait::async_trait;
// mod snyk;

#[async_trait]
/// Provider trait.
/// Defines the scan method that is
/// used to start a scan of targets
///
pub trait Provider {
    
    /// Method to start the scan for a Provider
    /// 
    async fn scan(&self);
}

/// Enum to define the types of Providers
/// we have available
///
pub enum ProviderKind {

    /// GitHub type Provider
    ///
    GitHub,

    /// Snyk type Provider
    ///
    Snyk,
}

/// Struct to define options related to a Provider
///
pub struct ProviderOpts {

    /// Type of Provider
    ///
    pub provider: ProviderKind,

    /// Output format type
    ///
    pub output_format: Option<OutputFormat>,

    /// Organization containing the targets
    ///
    pub org: Option<String>,
}

/// Opts implementation for ProviderOpts
///
impl Opts for ProviderOpts {

    /// Method to format output for a given option
    /// 
    fn format(&self) -> OutputFormat {
        let format = self.output_format.clone();
        match format {
            None => OutputFormat::Text,
            Some(format) => format,
        }
    }
}

/// Struct definition for the ProviderCommand.
/// This struct executes a command to start a
/// Provider. Command pattern style.
pub struct ProviderCommand {}

/// Implementation for ProviderCommand
/// Contains the execute() method
///
impl ProviderCommand {

    /// Method to actually execute the Provider's scan
    ///
    pub async fn execute(opts: ProviderOpts) -> Result<(), Error> {
        let provider = ProviderFactory::new(opts);
        provider.scan().await;
        Ok(())
    }
}

/// Struct definition for the ProviderFactory.
/// This struct generates Provider types to crawl various services
/// to extract SBOMs
///
pub struct ProviderFactory {}

/// Implementation for ProviderFactory
///
impl ProviderFactory {

    /// Conventional 'new' method to actually create an instance
    /// of ProviderFactory
    ///
    pub fn new(pilot_ops: ProviderOpts) -> Box<dyn Provider> {
        return match pilot_ops.provider {
            ProviderKind::GitHub => Box::new(GitHubProvider {}),
            ProviderKind::Snyk => panic!("Jon, return SnykProvider implementation"),
        };
    }
}

/// Allows specifying the output format.
#[derive(Clone)]
pub enum OutputFormat {

    /// Output as JSON.
    ///
    Json,

    /// Output as plaintext.
    ///
    Text,
}

/// Generic trait that all command options must implement.
///
trait Opts {
    fn format(&self) -> OutputFormat {
        OutputFormat::Text
    }
}

/// Generic Command trait that all command handlers must implement.
///
trait Command<T> where T: Opts + Send + Sync {

    /// Execute method actually runs the command
    ///
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

#[tokio::test]
async fn test_get_env_var() {
    let var_name: &str = "TEST_ENV_VAR";
    let var_value: &str = "testvalue";
    env::set_var(var_name, var_value);

    match get_env_var(var_name) {
        Some(value) => assert_eq!(var_value, value),
        None => panic!("No test value...?")
    }
}