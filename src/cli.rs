//! Command line app
//!
//! ## config
//!
//! allows the user to provide an alternative config file. If it does not exist
//! it will be created in the default storage location for the OS.
//!

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GhDashCli {
    /// alternate configuration file
    #[arg(short, long)]
    config: Option<String>,
    /// github user name required for data
    #[arg(short, long)]
    user: Option<String>,
}

impl GhDashCli {
    pub fn config(&self) -> Option<String> {
        self.config.clone()
    }

    pub fn user(&self) -> Option<String> {
        self.user.clone()
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    GhDashCli::command().debug_assert()
}
