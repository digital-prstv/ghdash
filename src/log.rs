use std::collections::HashMap;

use crate::Error;
use bollard::container::ListContainersOptions;
use bollard::Docker;
use opentelemetry::global;
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::Tracer;

use tracing::{info, span, Level};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Get the logging setup
///
/// In the case wheren tracing is available (ZIpkin container running) configure tracing
/// and logging in line with the verbosity specifeid when the programme was run, otherwise,
/// enable logging in line with the verbosity specifeid.
///
pub async fn get_logging(verbosity: log::LevelFilter) -> Result<(), Error> {
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

/// Enum Representing a Docker Connection
///
/// ## Variants
/// - Connection(Docker) - a connection has been configured and verified. Supplied within the tupple.
/// - NoConnection - No connection has been succesfully created and verified.
///
#[derive(Debug)]
pub enum DockerConnection {
    /// connection is successful and supplied within the variant.
    Connection(Docker),
    /// No connection has been created and verified.
    NoConnection,
}

/// Connect to docker
///
/// The connection is configured either by the default (dev) configuration or using
/// a default setup. Once created the connection is verified by pinging.
/// If the ping fails then [NoConnection] is returned else the successful connection is
/// returned wrapped in the [Connection] variant of the [DockerConnection] enum.
///
pub async fn connect_docker() -> DockerConnection {
    let mut connection = bollard::Docker::connect_with_unix(
        "/home/gorta/.docker/desktop/docker.sock",
        120,
        bollard::API_DEFAULT_VERSION,
    );
    let mut docker = connection.unwrap();

    match docker.ping().await {
        Ok(_) => DockerConnection::Connection(docker),
        Err(_) => {
            connection = bollard::Docker::connect_with_local_defaults();
            docker = connection.unwrap();

            match docker.ping().await {
                Ok(_) => DockerConnection::Connection(docker),
                Err(_) => DockerConnection::NoConnection,
            }
        }
    }
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
