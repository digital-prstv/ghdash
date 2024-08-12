//! Command line app
//!
//! ## config
//!
//! allows the user to provide an alternative config file. If it does not exist
//! it will be created in the default storage location for the OS.
//!

use super::RepoScope;
use clap::{Parser, Subcommand};

/// Command line app
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Logging level
    #[clap(flatten)]
    pub logging: clap_verbosity_flag::Verbosity,
    /// Force the calculation of the version number
    /// alternate configuration file
    #[arg(short, long)]
    pub config: Option<String>,
    /// github user name
    #[arg(short, long)]
    pub user: Option<String>,
    /// github personal access token
    #[arg(short, long)]
    pub token: Option<String>,
    /// scope for the repositories shown in the dashboard
    #[arg(value_enum, short, long)]
    pub repositories: Option<RepoScope>,
    /// Subcommand
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcommands
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List repositories
    #[command(hide = true)]
    List,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
