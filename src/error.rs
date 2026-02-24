use thiserror::Error;

/// curl-compatible exit codes
#[allow(dead_code)]
pub const EXIT_OK: i32 = 0;
pub const EXIT_DNS: i32 = 6;
pub const EXIT_CONNECT: i32 = 7;
pub const EXIT_WRITE: i32 = 23;
pub const EXIT_TIMEOUT: i32 = 28;
pub const EXIT_SSL: i32 = 35;
pub const EXIT_TOO_MANY_REDIRECTS: i32 = 47;
pub const EXIT_GENERIC: i32 = 1;

#[derive(Debug, Error)]
pub enum RurlError {
    #[error("Could not resolve host: {0}")]
    Dns(String),

    #[error("Failed to connect to host: {0}")]
    Connect(String),

    #[error("Operation timed out")]
    Timeout,

    #[error("SSL/TLS error: {0}")]
    Ssl(String),

    #[error("Too many redirects")]
    TooManyRedirects,

    #[error("Write error: {0}")]
    Write(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    #[error("Request error: {0}")]
    Request(String),
}

impl RurlError {
    /// Returns the curl-compatible exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            RurlError::Dns(_) => EXIT_DNS,
            RurlError::Connect(_) => EXIT_CONNECT,
            RurlError::Timeout => EXIT_TIMEOUT,
            RurlError::Ssl(_) => EXIT_SSL,
            RurlError::TooManyRedirects => EXIT_TOO_MANY_REDIRECTS,
            RurlError::Write(_) => EXIT_WRITE,
            _ => EXIT_GENERIC,
        }
    }
}

impl From<reqwest::Error> for RurlError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            return RurlError::Timeout;
        }
        if e.is_redirect() {
            return RurlError::TooManyRedirects;
        }
        if e.is_connect() {
            let msg = e.to_string();
            // Check if it's a DNS-related error
            if msg.contains("dns") || msg.contains("resolve") || msg.contains("No such host") {
                return RurlError::Dns(msg);
            }
            return RurlError::Connect(msg);
        }
        // Check for SSL errors
        let msg = e.to_string();
        if msg.contains("ssl") || msg.contains("tls") || msg.contains("certificate") {
            return RurlError::Ssl(msg);
        }
        RurlError::Http(e.to_string())
    }
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, RurlError>;
