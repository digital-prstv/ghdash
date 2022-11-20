mod cli;
mod config;

use crate::cli::GhDashCli;
use crate::config::GhConfig;
use clap::Parser;
use ghdash::Dashboard;

type Error = ghdash::Error;
const APP_NAME: &str = clap::crate_name!();

fn main() -> Result<(), Error> {
    let args = GhDashCli::parse();

    let cfg: GhConfig = confy::load(APP_NAME, args.config().as_deref())?;

    let dashboard = Dashboard::new(cfg.user().as_str(), cfg.token().as_str())?;

    dbg!(dashboard);

    Ok(())
}
