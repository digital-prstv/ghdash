//! Command line app
//!
//! ## config
//!
//! allows the user to provide an alternative config file. If it does not exist
//! it will be created in the default storage location for the OS.
//!

use clap::Parser;
use ghdash::RepoScope;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GhDashCli {
    /// alternate configuration file
    #[arg(short, long)]
    config: Option<String>,
    /// github user name
    #[arg(short, long)]
    user: Option<String>,
    /// github personal access token
    #[arg(short, long)]
    token: Option<String>,
    /// scope for the repositories shown in the dashboard
    #[arg(value_enum, short, long)]
    repositories: Option<RepoScope>,
}

impl GhDashCli {
    pub fn config(&self) -> Option<String> {
        self.config.clone()
    }

    pub fn user(&self) -> Option<String> {
        self.user.clone()
    }

    pub fn token(&self) -> Option<String> {
        self.token.clone()
    }

    pub fn repositories(&self) -> Option<RepoScope> {
        self.repositories
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    GhDashCli::command().debug_assert()
}
