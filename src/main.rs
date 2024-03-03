mod cli;
mod config;

use std::collections::HashMap;

use crate::cli::GhDashCli;
use crate::config::GhConfig;
use bollard::container::ListContainersOptions;
use bollard::Docker;
use clap::Parser;
use ghdash::{Dashboard, Error};
use opentelemetry::global;
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::Tracer;

use tracing::{info, span, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const APP_NAME: &str = clap::crate_name!();

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = GhDashCli::parse();

    get_logging(args.logging.log_level_filter()).await?;

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

    print!("{dashboard}");

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

async fn get_logging(verbosity: log::LevelFilter) -> Result<(), Error> {
    if zipkin_container_running(connect_docker().await).await {
        let tracer = init_tracer()?;
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new("TRACE"))
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(
                fmt::Layer::default()
                    .pretty()
                    .with_filter(EnvFilter::from(format!("ghdash={verbosity}"))),
            )
            .try_init()?;

        let span = span!(Level::INFO, "logging initiatilisation");
        let _guard = span.enter();
        info!("Initialised tracing and logging to console at {verbosity}");
    } else {
        let filter = EnvFilter::from(format!(
            "ghdash={}",
            if verbosity == log::LevelFilter::Trace {
                log::LevelFilter::Debug
            } else {
                verbosity
            }
        ));

        let log_subscriber = tracing_subscriber::FmtSubscriber::builder()
            .pretty()
            .with_env_filter(filter)
            .finish();

        let _ = tracing::subscriber::set_global_default(log_subscriber)
            .map_err(|_| eprintln!("Unable to set global default subscriber!"));

        info!("Initialised logging to console at {verbosity}");
    }
    Ok(())
}

fn init_tracer() -> Result<Tracer, TraceError> {
    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());
    opentelemetry_zipkin::new_pipeline()
        .with_service_name("ghdash")
        .install_batch(Tokio)
}

#[derive(Debug)]
enum DockerConnection {
    Connection(Docker),
    NoConnection,
}

async fn connect_docker() -> DockerConnection {
    let mut connection = bollard::Docker::connect_with_unix(
        "/home/gorta/.docker/desktop/docker.sock",
        120,
        bollard::API_DEFAULT_VERSION,
    );
    println!("The connection result is {connection:?}");
    let mut docker = connection.unwrap();

    let result = match docker.ping().await {
        Ok(_) => {
            println!("Connected!");
            DockerConnection::Connection(docker)
        }
        Err(_) => {
            connection = bollard::Docker::connect_with_local_defaults();
            println!("The connection result is {connection:?}");
            docker = connection.unwrap();

            match docker.ping().await {
                Ok(_) => {
                    println!("Connected!");
                    DockerConnection::Connection(docker)
                }
                Err(e) => {
                    println!("Connection error: {:?}", e);
                    DockerConnection::NoConnection
                }
            }
        }
    };
    println!("connect_docker result {result:?}");
    result
}

async fn zipkin_container_running(docker: DockerConnection) -> bool {
    let docker = match docker {
        DockerConnection::Connection(d) => d,
        _ => {
            return false;
        }
    };

    const TRACER_IMAGE: &str = "openzipkin/zipkin";

    let mut filters = HashMap::new();
    filters.insert(String::from("ancestor"), vec![String::from(TRACER_IMAGE)]);
    filters.insert(String::from("status"), vec![String::from("running")]);

    let containers = docker
        .list_containers(Some(ListContainersOptions::<String> {
            all: true,
            filters,
            ..Default::default()
        }))
        .await
        .unwrap();

    !containers.is_empty()
}
