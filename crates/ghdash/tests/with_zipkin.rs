use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use futures_util::StreamExt;

use bollard::{
    models::PortBinding,
    query_parameters::{
        CreateContainerOptions, ListContainersOptions, StartContainerOptions, StopContainerOptions,
        WaitContainerOptions,
    },
    secret::HostConfig,
    Docker,
};

const TRACER_NAME: &str = "zipkin_tester";
const TRACER_IMAGE: &str = "openzipkin/zipkin";

#[tokio::test]
async fn with_zipkin_tests() {
    let ghdash::DockerConnection::Connection(docker) = ghdash::connect_docker().await else {
        return;
    };

    let started_container = ensure_container_started(&docker, TRACER_NAME, TRACER_IMAGE).await;

    // trycmd::TestCases::new()
    //     .case("tests/logging/*.trycmd")
    //     .insert_var("[MESSAGE]", "tracing and logging")
    //     .unwrap();

    stop_container(&docker, &started_container).await;
}

async fn ensure_container_started(docker: &Docker, name: &str, image: &str) -> String {
    match zipkin_container_running(docker).await {
        ContainerState::Stopped(container) => {
            start_container(docker, &container).await;
            container
        }
        ContainerState::None => {
            create_container(docker, name, image).await;
            start_container(docker, name).await;
            name.to_string()
        }
        ContainerState::Started(container) => container,
    }
}

enum ContainerState {
    Stopped(String),
    Started(String),
    None,
}

async fn zipkin_container_running(docker: &Docker) -> ContainerState {
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

    if !containers.is_empty() {
        ContainerState::Started(containers[0].names.as_ref().unwrap()[0].clone())
    } else {
        let mut filters = HashMap::new();
        filters.insert(String::from("ancestor"), vec![String::from(TRACER_IMAGE)]);
        let filters = Some(filters);

        let stopped_containers = docker
            .list_containers(Some(ListContainersOptions {
                all: true,
                filters,
                ..Default::default()
            }))
            .await
            .unwrap();
        if !stopped_containers.is_empty() {
            ContainerState::Stopped(stopped_containers[0].names.as_ref().unwrap()[0].clone())
        } else {
            ContainerState::None
        }
    }
}

async fn start_container(docker: &Docker, name: &str) -> bool {
    let name = name.trim_start_matches('/');

    match docker
        .start_container(
            name,
            Some(StartContainerOptions {
                ..Default::default()
            }),
        )
        .await
    {
        Ok(_) => true,
        Err(e) => match e {
            bollard::errors::Error::DockerResponseServerError {
                status_code,
                message: _,
            } => status_code == 304,
            _ => return false,
        },
    };

    let timeout_duration = Duration::from_secs(30); // Timeout duration in seconds

    let start_time = Instant::now();

    let mut not_elapsed = true;
    let mut error_response = true;

    while not_elapsed && error_response {
        not_elapsed = start_time.elapsed() < timeout_duration;
        error_response = reqwest::get("http://localhost:9411/api/v2/services")
            .await
            .is_err();
    }

    not_elapsed
}

async fn create_container(docker: &Docker, container: &str, image: &str) {
    let options = Some(CreateContainerOptions {
        name: Some(container.to_string()),
        ..Default::default()
    });

    let port = "9411";

    let mut mapping = HashMap::new();
    mapping.insert(
        String::from("9411/tcp"),
        Some(vec![PortBinding {
            host_ip: Some("".to_string()),
            host_port: Some(port.to_string()),
        }]),
    );

    let host_config = Some(HostConfig {
        port_bindings: Some(mapping),
        ..Default::default()
    });

    let config = bollard::models::ContainerCreateBody {
        image: Some(String::from(image)),
        // exposed_ports,
        host_config,
        ..Default::default()
    };

    dbg!(docker
        .create_container(options, config)
        .await
        .expect("Trying to create the container"));
}

async fn stop_container(docker: &Docker, name: &str) {
    let name = name.trim_start_matches('/');

    let options = Some(StopContainerOptions {
        t: Some(30),
        ..Default::default()
    });

    docker
        .stop_container(name, options)
        .await
        .expect("Stopping the container");

    let wait_options = Some(WaitContainerOptions {
        condition: "not-running".to_string(),
    });

    let stream = &mut docker.wait_container(name, wait_options).take(1);

    while let Some(_response) = stream.next().await {}
}
