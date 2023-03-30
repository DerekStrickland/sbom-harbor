use clap::{Arg, ArgAction, ArgMatches, Command};
use harbor_cli::commands::OutputFormat;
use harbor_cli::commands::{
    ProviderCommand,
    ProviderKind,
    ProviderOpts
};

const PROVIDER: &str = "provider";
const GITHUB: &str = "github";
const SNYK: &str = "snyk";
const CMS_ORG: &str = "cmsgov";

fn get_matches() -> ArgMatches {
    return Command::new("harbor-cli")
        .about("SBOM Harbor Runtime CLI")
        .version("0.0.1")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("SBOM Harbor Team")
        .arg(
            Arg::new(PROVIDER)
                .required(true)
                .long(PROVIDER)
                .help("the type of SBOM provider to start: github or snyk")
                .action(ArgAction::Set)
                .num_args(1),
        )
        .get_matches();
}

#[tokio::main]
async fn main() {
    let matches = get_matches();

    // Extract the Provider
    if let Some(provider) = matches.get_one::<String>(PROVIDER) {

        // Test to see if the provider is for the GitHub SBOM Provider

        match provider.as_str() {

            // Run the GitHub Provider
            GITHUB => {
                println!("Starting {} provider.", provider);
                ProviderCommand::execute(
                    ProviderOpts {
                        provider: ProviderKind::GitHub,
                        output_format: Some(OutputFormat::Text),
                        org: Some(CMS_ORG.to_string()),
                    }
                ).await.unwrap();
            },

            // Run the Snyk Provider
            SNYK => {
                println!("Snyk Provider is not implemented yet");
            },

            // On Default, drop a message about no provider with that name existing.
            _ => {
                println!(
                    "No Provider named {} exists. Use either {} or {}",
                    provider, GITHUB, SNYK
                )
            }
        }
    }
}
