use std::time::Duration;

use reqwest::{Client, Proxy};

use crate::{cli::Args, error::RurlError};

/// Build a `reqwest::Client` from CLI arguments
pub fn build_client(args: &Args) -> Result<Client, RurlError> {
    let mut builder = Client::builder()
        .danger_accept_invalid_certs(args.insecure)
        .redirect(if args.location {
            reqwest::redirect::Policy::limited(args.max_redirs as usize)
        } else {
            reqwest::redirect::Policy::none()
        });

    // Timeouts
    if let Some(max_time) = args.max_time {
        builder = builder.timeout(Duration::from_secs_f64(max_time));
    }
    if let Some(connect_timeout) = args.connect_timeout {
        builder = builder.connect_timeout(Duration::from_secs_f64(connect_timeout));
    }

    // Proxy
    if let Some(proxy_url) = &args.proxy {
        let proxy = Proxy::all(proxy_url.as_str())
            .map_err(|e| RurlError::Request(format!("Invalid proxy URL: {e}")))?;
        builder = builder.proxy(proxy);
    }

    // User-Agent
    let user_agent = args
        .user_agent
        .clone()
        .unwrap_or_else(|| format!("rurl/{}", env!("CARGO_PKG_VERSION")));
    builder = builder.user_agent(user_agent);

    builder
        .build()
        .map_err(|e| RurlError::Request(format!("Failed to build HTTP client: {e}")))
}
