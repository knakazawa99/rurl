use clap::Parser;

/// A curl-compatible HTTP client written in Rust
#[derive(Parser, Debug, Clone)]
#[command(name = "rurl", version, about, long_about = None)]
pub struct Args {
    /// URL to request
    pub url: String,

    // ── Request control ──────────────────────────────────────────────────

    /// HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
    #[arg(short = 'X', long = "request", default_value = "GET")]
    pub method: String,

    /// Custom request header (can be specified multiple times)
    #[arg(short = 'H', long = "header", value_name = "HEADER")]
    pub headers: Vec<String>,

    /// Send HEAD request
    #[arg(short = 'I', long = "head")]
    pub head: bool,

    // ── Request body ─────────────────────────────────────────────────────

    /// Request body (prefix with @ to read from file: @filename)
    #[arg(short = 'd', long = "data", value_name = "DATA")]
    pub data: Option<String>,

    /// Request body without @file expansion
    #[arg(long = "data-raw", value_name = "DATA")]
    pub data_raw: Option<String>,

    /// JSON body (sets Content-Type: application/json and Accept: application/json)
    #[arg(long = "json", value_name = "DATA")]
    pub json: Option<String>,

    /// Multipart form field (name=value or name=@file)
    #[arg(short = 'F', long = "form", value_name = "CONTENT")]
    pub form: Vec<String>,

    /// Upload file (PUT by default)
    #[arg(short = 'T', long = "upload-file", value_name = "FILE")]
    pub upload_file: Option<String>,

    // ── Output control ───────────────────────────────────────────────────

    /// Write response to file
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    pub output: Option<String>,

    /// Write response to file named from URL
    #[arg(short = 'O', long = "remote-name")]
    pub remote_name: bool,

    /// Include response headers in output
    #[arg(short = 'i', long = "include")]
    pub include: bool,

    /// Silent mode (no progress or error output)
    #[arg(short = 's', long = "silent")]
    pub silent: bool,

    /// Verbose output (shows request and response headers)
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,

    /// Write-out format string (e.g. "%{http_code}")
    #[arg(short = 'w', long = "write-out", value_name = "FORMAT")]
    pub write_out: Option<String>,

    // ── Redirect ─────────────────────────────────────────────────────────

    /// Follow redirects
    #[arg(short = 'L', long = "location")]
    pub location: bool,

    /// Maximum number of redirects
    #[arg(long = "max-redirs", value_name = "NUM", default_value = "30")]
    pub max_redirs: u32,

    // ── Authentication ───────────────────────────────────────────────────

    /// Basic authentication (user:password)
    #[arg(short = 'u', long = "user", value_name = "USER:PASSWORD")]
    pub user: Option<String>,

    // ── Request headers ──────────────────────────────────────────────────

    /// User-Agent string
    #[arg(short = 'A', long = "user-agent", value_name = "STRING")]
    pub user_agent: Option<String>,

    /// Referer URL
    #[arg(short = 'e', long = "referer", value_name = "URL")]
    pub referer: Option<String>,

    // ── TLS / Security ───────────────────────────────────────────────────

    /// Skip TLS certificate verification (insecure)
    #[arg(short = 'k', long = "insecure")]
    pub insecure: bool,

    /// CA certificate file
    #[arg(long = "cacert", value_name = "FILE")]
    pub cacert: Option<String>,

    /// Client certificate file
    #[arg(long = "cert", value_name = "FILE")]
    pub cert: Option<String>,

    /// Client private key file
    #[arg(long = "key", value_name = "FILE")]
    pub key: Option<String>,

    // ── Timeouts ─────────────────────────────────────────────────────────

    /// Maximum time (seconds) for the entire operation
    #[arg(short = 'm', long = "max-time", value_name = "SECONDS")]
    pub max_time: Option<f64>,

    /// Connection timeout (seconds)
    #[arg(long = "connect-timeout", value_name = "SECONDS")]
    pub connect_timeout: Option<f64>,

    // ── Proxy ────────────────────────────────────────────────────────────

    /// HTTP/HTTPS proxy URL
    #[arg(short = 'x', long = "proxy", value_name = "URL")]
    pub proxy: Option<String>,

    // ── Cookies ──────────────────────────────────────────────────────────

    /// Send cookies from string or file
    #[arg(short = 'b', long = "cookie", value_name = "DATA")]
    pub cookie: Option<String>,

    /// Save cookies to file
    #[arg(short = 'c', long = "cookie-jar", value_name = "FILE")]
    pub cookie_jar: Option<String>,

    // ── Encoding ─────────────────────────────────────────────────────────

    /// Request compressed response
    #[arg(long = "compressed")]
    pub compressed: bool,

    // ── Query params (GET mode) ──────────────────────────────────────────

    /// Append --data to URL as query string
    #[arg(short = 'G', long = "get")]
    pub get_mode: bool,
}

impl Args {
    /// Determine the effective HTTP method
    pub fn effective_method(&self) -> &str {
        if self.head {
            return "HEAD";
        }
        if self.upload_file.is_some() && self.method == "GET" {
            return "PUT";
        }
        if (self.data.is_some()
            || self.data_raw.is_some()
            || self.json.is_some()
            || !self.form.is_empty())
            && self.method == "GET"
        {
            return "POST";
        }
        &self.method
    }
}
