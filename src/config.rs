//! Configuration data for the application
//!
//! The configuration data is stored by confy in the approproate default configuration location for the OS.
//!
//! ## Configuration fields
//!
//! user: the user github user name
//! token: the token granting access to the repositories
//!

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GhConfig {
    user: String,
    token: String,
}

impl GhConfig {
    pub fn user(&self) -> String {
        self.user.clone()
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }
}
