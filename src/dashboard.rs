//! Dashboard module represents that data presented in the dashboard
//!

use crate::error::Error;

/// Struct Representing a Dashboard and key data required to create the dashboard
///
/// ## Fields
/// - user - the github user for which the dashboard is created
/// - token - a personal access token for the user that provides access to the github API
/// - repositories - a list of the user's repositories presented in the dashboard
///
#[derive(Debug)]
pub struct Dashboard {
    user: String,
    token: String,
    repositories: Vec<String>,
}

impl Dashboard {
    /// Create a dashboard by setting the user and token strings to access Github
    /// Without a user and token to get data from Github the dashboard is meaningless
    /// therefore a new struct without this data is not meaningful
    ///
    pub fn new(user: &str, token: &str) -> Result<Dashboard, Error> {
        if user.is_empty() {
            return Err(Error::MustHaveUser);
        }

        Ok(Dashboard {
            user: user.to_string(),
            token: token.to_string(),
            repositories: vec![],
        })
    }

    /// Get the user
    ///
    pub fn user(&self) -> &str {
        &self.user
    }

    /// Set the user name in the Dashboard struct
    ///
    pub fn set_user(&mut self, user: &str) -> &mut Self {
        self.user = user.to_string();
        self
    }

    /// Set the token in the Dashboard
    ///
    pub fn set_token(&mut self, token: &str) -> &mut Self {
        self.token = token.to_string();
        self
    }

    /// Add a repo to the Dashboard
    ///
    pub fn add_repo(&mut self, repo: &str) -> &mut Self {
        self.repositories.push(repo.to_string());
        self
    }
}
