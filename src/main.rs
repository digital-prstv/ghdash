mod cli;
mod config;

use crate::cli::GhDashCli;
use crate::config::GhConfig;
use clap::Parser;

type Error = confy::ConfyError;

fn main() -> Result<(), Error> {
    let _args = GhDashCli::parse();
    let cfg: GhConfig = confy::load("ghdash", None)?;
    dbg!(cfg);

    Ok(())
}
