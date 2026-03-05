#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]

//! Github Dashboard
//!
//! Provides a library of functions to build a dashboard for a github user.
//!
//! ## Usage
//!
//! Add the dependency to Cargo.toml
//!
//! ```toml
//!
//! [dependencies]
//! ghdash = "0.2.0"
//!
//! ```
//!
//! ```no_run
//! # type Error = confy::ConfyError;
//! # fn main() -> Result<(), Error> {
//!
//! # Ok(())
//! # }
//! ```
//!

mod cli;
mod config;
mod dashboard;
mod error;
mod log;

pub use cli::Cli;
pub use cli::Commands;
pub use config::GhConfig;
pub use dashboard::Dashboard;
pub use dashboard::RepoScope;
pub use error::Error;
pub use log::connect_docker;
pub use log::get_logging;
pub use log::DockerConnection;
