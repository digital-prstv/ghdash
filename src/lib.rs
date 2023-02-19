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
//! ghdash = "0.1.6"
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

mod dashboard;
mod error;

pub use dashboard::Dashboard;
pub use dashboard::RepoScope;
pub use error::Error;
