mod cli;
mod config;

use crate::cli::GhDashCli;
use crate::config::GhConfig;
use clap::Parser;

type Error = confy::ConfyError;
const APP_NAME: &str = clap::crate_name!();

fn main() -> Result<(), Error> {
    let args = GhDashCli::parse();

    let cfg: GhConfig = confy::load(APP_NAME, args.config().as_deref())?;

    Ok(())
}
