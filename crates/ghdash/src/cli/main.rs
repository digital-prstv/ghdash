use clap::Parser;
use ghdash::{get_logging, Dashboard, DockerConnection, Error, GhConfig};
use ghdash::{Cli, Commands};

const APP_NAME: &str = clap::crate_name!();

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    get_logging(args.logging.log_level_filter()).await?;

    let config_name = args.config;
    let config_name = config_name.as_deref();

    //1. Load the configuration file for default (known) settings
    let mut cfg: GhConfig = confy::load(APP_NAME, config_name)?;

    //2. Check environment for values of user and token
    cfg.try_env_user(APP_NAME);
    cfg.try_env_token(APP_NAME);

    //3. Check command line for values of user and token (save if found)
    if let Some(user) = args.user {
        cfg.set_user(user.as_str());
        confy::store(APP_NAME, config_name, cfg.clone())?;
    }

    if let Some(token) = args.token {
        cfg.set_token(token.as_str());
        confy::store(APP_NAME, config_name, cfg.clone())?;
    }

    match args.command {
        Some(command) => {
            match command {
                Commands::List => {
                    let docker_connection = ghdash::connect_docker().await;
                    match docker_connection {
                        DockerConnection::Connection(docker) => {
                            println!("Got a connection: {docker:#?}");
                            let containers = docker
                                .list_containers(Some(
                                    bollard::query_parameters::ListContainersOptions {
                                        all: true,
                                        // filters,
                                        ..Default::default()
                                    },
                                ))
                                .await
                                .unwrap();

                            println!("List of Containers and Status");

                            for container in containers {
                                println!(
                                    "-> Name: {:?}\tStatus: {:?},\tImage: {:?}",
                                    container.names.unwrap(),
                                    container.status.unwrap(),
                                    container.image.unwrap()
                                );
                            }
                            println!();
                        }
                        DockerConnection::NoConnection => println!("No docker connection"),
                    }
                }
            }
        }
        None => {
            let mut dashboard = Dashboard::new_with_personal_access_token(
                cfg.user().as_str(),
                cfg.token().as_str(),
            )?;
            dashboard
                .set_repo_scope(args.repositories.unwrap_or_default())
                .generate()
                .await?;

            print!("{dashboard}");
        }
    }
    // opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
