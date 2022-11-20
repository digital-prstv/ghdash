//! Error types for ghdash
use thiserror::Error;

/// The error type for ghdash.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// User name cannot be empty.
    #[error("User name must be specified, cannot be an empty string")]
    MustHaveUser,
    /// Error passed up from confy
    #[error("0:?")]
    Confy(#[from] confy::ConfyError),
}
