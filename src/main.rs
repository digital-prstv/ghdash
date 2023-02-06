mod cli;
mod config;

use crate::cli::GhDashCli;
use crate::config::GhConfig;
use clap::Parser;
use ghdash::{Dashboard, Error};
use opentelemetry::{sdk::trace::Tracer, trace::TraceError};
use tracing::{debug, info, span, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const APP_NAME: &str = clap::crate_name!();

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = GhDashCli::parse();

    get_logging(args.logging.log_level_filter())?;

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

    debug!(dashboard = ?dashboard);

    print!("{dashboard}");

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

fn get_logging(verbosity: log::LevelFilter) -> Result<(), Error> {
    let tracer = init_tracer()?;
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("TRACE"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(
            fmt::Layer::default()
                .pretty()
                .with_filter(EnvFilter::from(verbosity.to_string())),
        )
        .try_init()?;

    let span = span!(Level::INFO, "logging initiatilisation");
    let _guard = span.enter();
    info!("Initialised full tracing and logging to console at {verbosity}");
    Ok(())
}

fn init_tracer() -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("ghdash")
        .install_simple()
}
