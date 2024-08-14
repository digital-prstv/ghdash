use std::env;

use ghdash::{Dashboard, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app_id = env::var("GHDASH_GH_APP_ID")?;

    let user = env::var("GHDASH_USER")?;

    let private_key = std::env::var("GHDASH_GH_APP_PRIVATE_KEY")?;

    let mut dashboard =
        Dashboard::new_with_github_app(user.as_str(), app_id.as_str(), private_key.as_str())
            .await?;
    dashboard.generate().await?;

    print!("{dashboard}");

    Ok(())
}
