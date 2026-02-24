use std::path::Path;

use reqwest::{
    header::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, REFERER},
    multipart, Client, Request,
};

use crate::{cli::Args, error::RurlError};

/// Build a `reqwest::Request` from CLI arguments
pub async fn build_request(client: &Client, args: &Args) -> Result<Request, RurlError> {
    let method = args
        .effective_method()
        .parse::<reqwest::Method>()
        .map_err(|_| RurlError::Request(format!("Invalid HTTP method: {}", args.method)))?;

    // Resolve URL (may append query string for -G mode)
    let url = resolve_url(args)?;

    let mut builder = client.request(method, url);

    // Custom headers
    for h in &args.headers {
        let (name, value) = parse_header(h)?;
        builder = builder.header(name, value);
    }

    // Referer
    if let Some(ref referer) = args.referer {
        builder = builder.header(
            REFERER,
            HeaderValue::from_str(referer)
                .map_err(|_| RurlError::InvalidHeader(format!("Invalid referer: {referer}")))?,
        );
    }

    // Basic auth
    if let Some(ref user_pass) = args.user {
        let (user, pass) = parse_user_pass(user_pass)?;
        builder = builder.basic_auth(user, pass);
    }

    // Body: priority is --json > -F > -d/--data-raw > -T
    if let Some(ref json_str) = args.json {
        builder = builder
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(json_str.clone());
    } else if !args.form.is_empty() {
        let form = build_multipart(&args.form).await?;
        builder = builder.multipart(form);
    } else if let Some(ref data) = args.data {
        let body = read_body_data(data)?;
        builder = builder.body(body);
    } else if let Some(ref data_raw) = args.data_raw {
        builder = builder.body(data_raw.clone());
    } else if let Some(ref upload_path) = args.upload_file {
        let bytes = tokio::fs::read(upload_path)
            .await
            .map_err(|e| RurlError::Request(format!("Failed to read upload file: {e}")))?;
        let mime = mime_guess::from_path(upload_path)
            .first_or_octet_stream()
            .to_string();
        builder = builder.header(CONTENT_TYPE, mime.as_str()).body(bytes);
    }

    // Cookie string
    if let Some(ref cookie_str) = args.cookie {
        if !cookie_str.starts_with('@') {
            builder = builder.header(
                reqwest::header::COOKIE,
                HeaderValue::from_str(cookie_str).map_err(|_| {
                    RurlError::InvalidHeader(format!("Invalid cookie: {cookie_str}"))
                })?,
            );
        }
        // @file cookies are handled by the cookie store via cookie_jar
    }

    // Add Accept-Encoding for --compressed
    if args.compressed {
        builder = builder.header(
            reqwest::header::ACCEPT_ENCODING,
            HeaderValue::from_static("gzip, deflate, br"),
        );
    }

    builder
        .build()
        .map_err(|e| RurlError::Request(format!("Failed to build request: {e}")))
}

/// Parse "Name: Value" header string
pub fn parse_header(h: &str) -> Result<(HeaderName, HeaderValue), RurlError> {
    let colon = h
        .find(':')
        .ok_or_else(|| RurlError::InvalidHeader(format!("Header missing colon separator: {h}")))?;
    let name = h[..colon].trim();
    let value = h[colon + 1..].trim();

    let header_name = HeaderName::from_bytes(name.as_bytes())
        .map_err(|_| RurlError::InvalidHeader(format!("Invalid header name: {name}")))?;
    let header_value = HeaderValue::from_str(value)
        .map_err(|_| RurlError::InvalidHeader(format!("Invalid header value: {value}")))?;

    Ok((header_name, header_value))
}

/// Parse "user:password" or "user" (no password)
fn parse_user_pass(s: &str) -> Result<(String, Option<String>), RurlError> {
    match s.find(':') {
        Some(idx) => Ok((s[..idx].to_string(), Some(s[idx + 1..].to_string()))),
        None => Ok((s.to_string(), None)),
    }
}

/// Read body data, expanding @file references
fn read_body_data(data: &str) -> Result<Vec<u8>, RurlError> {
    if let Some(path) = data.strip_prefix('@') {
        std::fs::read(path)
            .map_err(|e| RurlError::Request(format!("Failed to read data file '{}': {e}", path)))
    } else {
        Ok(data.as_bytes().to_vec())
    }
}

/// Resolve URL, optionally appending --data as query string for -G mode
fn resolve_url(args: &Args) -> Result<reqwest::Url, RurlError> {
    let mut url = reqwest::Url::parse(&args.url)?;

    if args.get_mode {
        if let Some(ref data) = args.data {
            let body = read_body_data(data)?;
            let query = String::from_utf8_lossy(&body);
            let existing = url.query().unwrap_or("").to_string();
            let new_query = if existing.is_empty() {
                query.to_string()
            } else {
                format!("{existing}&{query}")
            };
            url.set_query(Some(&new_query));
        }
    }

    Ok(url)
}

/// Build multipart form from -F arguments
async fn build_multipart(form_args: &[String]) -> Result<multipart::Form, RurlError> {
    let mut form = multipart::Form::new();

    for arg in form_args {
        let eq = arg
            .find('=')
            .ok_or_else(|| RurlError::Request(format!("Form field missing '=': {arg}")))?;
        let name = arg[..eq].to_string();
        let value = &arg[eq + 1..];

        if let Some(path_str) = value.strip_prefix('@') {
            let path = Path::new(path_str);
            let bytes = tokio::fs::read(path).await.map_err(|e| {
                RurlError::Request(format!("Failed to read form file '{}': {e}", path_str))
            })?;
            let mime = mime_guess::from_path(path)
                .first_or_octet_stream()
                .to_string();
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path_str)
                .to_string();
            let part = multipart::Part::bytes(bytes)
                .file_name(filename)
                .mime_str(&mime)
                .map_err(|e| RurlError::Request(format!("Invalid MIME type: {e}")))?;
            form = form.part(name, part);
        } else {
            form = form.text(name, value.to_string());
        }
    }

    Ok(form)
}

/// Produce a canonical Authorization header value from args for verbose display
#[allow(dead_code)]
pub fn auth_header_display(args: &Args) -> Option<String> {
    args.user.as_ref().map(|up| {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(up.as_bytes());
        format!("Authorization: Basic {encoded}")
    })
}
