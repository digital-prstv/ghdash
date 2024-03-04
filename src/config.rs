//! Configuration data for the application
//!
//! The configuration data is stored by confy in the approproate default configuration location for the OS.
//!
//! ## Configuration fields
//!
//! user: the user github user name
//! token: the token granting access to the repositories
//!

use std::ffi::OsString;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct GhConfig {
    user: String,
    token: String,
}

impl GhConfig {
    pub fn user(&self) -> String {
        self.user.clone()
    }

    pub fn set_user(&mut self, user: &str) -> &mut Self {
        self.user = user.to_string();
        self
    }

    pub fn try_env_user(&mut self, prefix: &str) -> &mut Self {
        let key = OsString::from(format!("{prefix}_USER").to_uppercase());

        if let Ok(env_user) = std::env::var(key) {
            self.user = env_user;
        };

        self
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn set_token(&mut self, token: &str) -> &mut Self {
        self.token = token.to_string();
        self
    }

    pub fn try_env_token(&mut self, prefix: &str) -> &mut Self {
        let key = OsString::from(format!("{prefix}_TOKEN").to_uppercase());

        if let Ok(env_token) = std::env::var(key) {
            self.token = env_token;
        };

        self
    }
}
