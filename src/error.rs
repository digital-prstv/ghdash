//! Error types for ghdash
use thiserror::Error;

/// The error type for ghdash.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// User name cannot be empty.
    #[error("User name must be specified, cannot be an empty string")]
    MustHaveUser,
    /// Token name cannot be empty.
    #[error("User name must be specified, cannot be an empty string")]
    MustHaveToken,
    /// Feature not yet implemented.
    #[error("Feautre has not been implemented yet.")]
    FeatureNotImplemented,
    /// Error passed up from confy
    #[error("0:?")]
    Confy(#[from] confy::ConfyError),
    /// Error passed up from anyhow
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    /// Error passed up from opentelemetry trace
    #[error("0:?")]
    OpentelemetryTrace(#[from] opentelemetry::trace::TraceError),
    /// Error passed up from tracing_subscriber try inita
    #[error("0:?")]
    TracingSubscriberTryInit(#[from] tracing_subscriber::util::TryInitError),
}
