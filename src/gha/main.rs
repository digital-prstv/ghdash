use std::env;

use base64::{engine::general_purpose::STANDARD, Engine};
use octocrate::{APIConfig, AppAuthorization, GitHubAPI};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app_id = env::var("GHDASH_GH_APP_ID").unwrap();
    // let app_id = app_id_str.parse::<i64>().unwrap();

    // let app_installation_id_str = env::var("GHDASH_GH_APP_INSTALLATION_ID").unwrap();
    // let app_installation_id = app_installation_id_str.parse::<i64>().unwrap();

    let encoded_private_key = std::env::var("GHDASH_GH_APP_PRIVATE_KEY").unwrap();
    let private_key = STANDARD.decode(encoded_private_key).unwrap();
    let private_key = String::from_utf8(private_key).unwrap();

    // Create a Github App authorization.
    let app_authorization = AppAuthorization::new(app_id, private_key);

    // Use Github App authorization to create an API configuration
    let config = APIConfig::with_token(app_authorization).shared();

    let api = GitHubAPI::new(&config);

    // Get the installation for a repoistory.
    let installation = api
        .apps
        .get_repo_installation("jerusdp", "ghdash")
        .send()
        .await
        .unwrap();

    // Get the Installation Acccess Token for this Installation
    let installation_token = api
        .apps
        .create_installation_access_token(installation.id)
        .send()
        .await
        .unwrap();

    // Use the Installation Access Token to create a new API configuration
    let config = APIConfig::with_token(installation_token).shared();

    let api = GitHubAPI::new(&config);

    let _repository = api.repos.get("jerusdp", "ghdash").send().await.unwrap();
}
