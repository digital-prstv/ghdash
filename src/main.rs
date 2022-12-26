mod cli;
mod config;

use crate::cli::GhDashCli;
use crate::config::GhConfig;
use clap::Parser;
use ghdash::{Dashboard, Error};
use log::LevelFilter;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const APP_NAME: &str = clap::crate_name!();

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = GhDashCli::parse();

    get_logging(args.logging.log_level_filter());

    let config_name = args.config;
    let config_name = config_name.as_deref();

    let mut cfg: GhConfig = confy::load(APP_NAME, config_name)?;

    if let Some(user) = args.user {
        cfg.set_user(user.as_str());
        confy::store(APP_NAME, config_name, cfg.clone())?;
    }

    if let Some(token) = args.token {
        cfg.set_token(token.as_str());
        confy::store(APP_NAME, config_name, cfg.clone())?;
    }

    let dashboard = Dashboard::builder(cfg.user().as_str(), cfg.token().as_str())?
        .set_repo_scope(args.repositories.unwrap_or_default())
        .finish()
        .await?;

    info!(dashboard = ?dashboard);

    let table = dashboard.build_dashboard();

    print!("{}", table);

    Ok(())
}

fn get_logging(verbosity: log::LevelFilter) {
    let level = match verbosity {
        LevelFilter::Debug => Level::DEBUG,
        LevelFilter::Error => Level::ERROR,
        LevelFilter::Info => Level::INFO,
        LevelFilter::Off => Level::INFO,
        LevelFilter::Trace => Level::TRACE,
        LevelFilter::Warn => Level::WARN,
    };

    if verbosity != LevelFilter::Off {
        let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }
}
