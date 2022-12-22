mod cli;
mod config;

use crate::cli::GhDashCli;
use crate::config::GhConfig;
use clap::Parser;
use ghdash::{Dashboard, Error, RepoScope};

const APP_NAME: &str = clap::crate_name!();

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = GhDashCli::parse();

    let config_name = args.config();
    let config_name = config_name.as_deref();

    let mut cfg: GhConfig = confy::load(APP_NAME, config_name)?;

    if let Some(user) = args.user() {
        cfg.set_user(user.as_str());
        confy::store(APP_NAME, config_name, cfg.clone())?;
    }

    if let Some(token) = args.token() {
        cfg.set_token(token.as_str());
        confy::store(APP_NAME, config_name, cfg.clone())?;
    }

    let repo_scope = args.repositories().unwrap_or(RepoScope::Authored);

    if RepoScope::Private == repo_scope {
        return Err(Error::FeatureNotImplemented);
    }

    let dashboard = Dashboard::new(cfg.user().as_str(), cfg.token().as_str(), repo_scope).await?;

    let table = dashboard.build_dashboard();

    print!("{}", table);

    Ok(())
}
