use thiserror::Error;

/// Error type returned by the CLI's stable agent-facing interface.
#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "no supported browser endpoint was found; enable Chrome remote debugging at chrome://inspect/#remote-debugging or pass --cdp"
    )]
    BrowserNotFound,

    #[error(
        "choose your browser first: lsearch connect --existing (Chrome approval) or lsearch connect --managed (separate persistent profile)"
    )]
    BrowserNotConfigured,

    #[error("selected browser is disconnected: {0}. No other browser was opened")]
    BrowserDisconnected(String),

    #[error(
        "Chrome approval was not completed within {timeout_ms}ms; choose Allow in Chrome, then retry lsearch connect --existing. No connection was saved"
    )]
    BrowserApprovalTimeout { timeout_ms: u64 },

    #[error(
        "Chrome refused the debugging connection; check its permission prompt and retry lsearch connect --existing. No connection was saved"
    )]
    BrowserApprovalDenied,

    #[error(
        "Chrome connection could not be established: {0}. Retry lsearch connect --existing; no connection was saved"
    )]
    BrowserConnectionFailed(String),

    #[error(
        "another command is using or updating this browser connection; retry when it finishes or increase --timeout"
    )]
    BrowserBusy,

    #[error("update check failed: {0}")]
    UpdateCheck(String),

    #[error("browser target not found: {0}")]
    TargetNotFound(String),

    #[error("browser backend does not support {feature}: {backend}")]
    Unsupported { backend: String, feature: String },

    #[error("browser protocol error in {method}: {message}")]
    Protocol { method: String, message: String },

    #[error("browser operation timed out after {timeout_ms}ms: {operation}")]
    Timeout { operation: String, timeout_ms: u64 },

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("browser JavaScript failed: {0}")]
    JavaScript(String),

    #[error("I/O error at {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error("HTTP request failed for {url}: {message}")]
    Http { url: String, message: String },

    #[error(transparent)]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
}

impl Error {
    /// Returns a stable machine-readable failure code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::BrowserNotFound => "browser_not_found",
            Self::BrowserNotConfigured => "browser_not_configured",
            Self::BrowserDisconnected(_) => "browser_disconnected",
            Self::BrowserApprovalTimeout { .. } => "browser_approval_timeout",
            Self::BrowserApprovalDenied => "browser_approval_denied",
            Self::BrowserConnectionFailed(_) => "browser_connection_failed",
            Self::BrowserBusy => "browser_busy",
            Self::UpdateCheck(_) => "update_check_failed",
            Self::TargetNotFound(_) => "target_not_found",
            Self::Unsupported { .. } => "unsupported",
            Self::Protocol { .. } => "protocol_error",
            Self::Timeout { .. } => "timeout",
            Self::InvalidArgument(_) => "invalid_argument",
            Self::JavaScript(_) => "javascript_error",
            Self::Io { .. } => "io_error",
            Self::Json(_) => "json_error",
            Self::Url(_) => "url_error",
            Self::Http { .. } => "http_error",
            Self::WebSocket(_) => "websocket_error",
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Adds a file path to an [`std::io::Error`].
pub trait IoContext<T> {
    /// Converts an I/O result into the CLI's typed error.
    ///
    /// # Errors
    /// Returns [`Error::Io`] when the underlying operation fails.
    fn at(self, path: impl Into<String>) -> Result<T>;
}

impl<T> IoContext<T> for std::io::Result<T> {
    fn at(self, path: impl Into<String>) -> Result<T> {
        let path = path.into();
        self.map_err(|source| Error::Io { path, source })
    }
}
