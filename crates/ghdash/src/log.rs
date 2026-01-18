use std::collections::HashMap;

use crate::Error;
use bollard::query_parameters::ListContainersOptions;
use bollard::Docker;
use opentelemetry::global::{self, BoxedTracer};
use opentelemetry_sdk::trace::{BatchConfigBuilder, BatchSpanProcessor, SdkTracerProvider};
use opentelemetry_sdk::Resource;

use opentelemetry_zipkin::ZipkinExporter;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// # Get Logging
///
/// Get the logging setup
///
/// Test for the availability of a running zipkin container to capture tracing output.
///
/// - If tracing is available start logging and tracing.
/// - Else start logging only.
///
/// In either case logging is set according to the `verbosity`. Tracing traces to `Trace` level.
///
/// #### Returns
///
/// Returns a null result if successful or passes up the error in the event of an error condition.
///
pub async fn get_logging(verbosity: log::LevelFilter) -> Result<(), Error> {
    if zipkin_container_running(connect_docker().await).await {
        let _tracer = init_tracer()?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new("TRACE"))
            // .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(
                fmt::Layer::default()
                    .pretty()
                    .with_filter(EnvFilter::from(format!("ghdash={verbosity}"))),
            )
            .try_init()?;

        // global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());

        // let exporter = ZipkinExporter::builder().build()?;
        // let provider = SdkTracerProvider::builder()
        //     .with_simple_exporter(exporter)
        //     .with_resource(
        //         Resource::builder_empty()
        //             .with_service_name("trace-demo")
        //             .build(),
        //     )
        // .build();

        //     let tracer = provider.

        // global::set_tracer_provider(provider.clone());

        // let tracer = global::tracer("zipkin-tracer");

        //     let span = span!(Level::INFO, "logging initialisation");
        //     let _guard = span.enter();
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

fn init_tracer() -> Result<BoxedTracer, crate::Error> {
    global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());
    // opentelemetry_zipkin::new_pipeline()
    //     .with_service_name("ghdash")
    //     .install_batch(Tokio)

    let exporter = ZipkinExporter::builder().build()?;

    let batch = BatchSpanProcessor::builder(exporter)
        .with_batch_config(
            BatchConfigBuilder::default()
                .with_max_queue_size(4096)
                .build(),
        )
        .build();

    let provider = SdkTracerProvider::builder()
        .with_span_processor(batch)
        .with_resource(
            Resource::builder_empty()
                .with_service_name("ghdash")
                .build(),
        )
        .build();

    global::set_tracer_provider(provider.clone());

    let tracer = global::tracer("ghdash");

    Ok(tracer)
}

/// # DockerConnection
///
/// Enum Representing a Docker Connection
///
#[derive(Debug)]
pub enum DockerConnection {
    /// wraps the verified Docker connection configuration.
    Connection(Docker),
    /// No verified configuration has been found.
    NoConnection,
}

/// Connect to docker
///
/// The connection is configured either by the default (dev) configuration or using
/// a default setup. Once created the connection is verified by pinging.
///
/// #### Return
///
/// The result is a variant of the [DockerConnection] enum.
///
/// - `NoConnection` is returned if a verified connection cannot be found.
/// - `Connection` wraps the verified connection if verification is successful.
///
pub async fn connect_docker() -> DockerConnection {
    let mut connection = bollard::Docker::connect_with_unix(
        "/home/gorta/.docker/desktop/docker.sock",
        120,
        bollard::API_DEFAULT_VERSION,
    );

    let mut ret = DockerConnection::NoConnection;

    let docker_res = connection;

    if let Ok(docker) = docker_res {
        match docker.ping().await {
            Ok(_) => ret = DockerConnection::Connection(docker),
            Err(_) => {
                connection = bollard::Docker::connect_with_local_defaults();
                let docker_res = connection;

                if let Ok(docker) = docker_res {
                    if (docker.ping().await).is_ok() {
                        ret = DockerConnection::Connection(docker)
                    }
                }
            }
        }
    }

    ret
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
    let filters = Some(filters);

    let containers = docker
        .list_containers(Some(ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        }))
        .await
        .unwrap();

    !containers.is_empty()
}
