use std::io::Write;

use console::style;
use reqwest::{Request, Response};

use crate::cli::Args;

/// Print verbose request information (> lines)
pub fn print_request_verbose(req: &Request, _args: &Args) {
    let method = req.method();
    let url = req.url();
    let path = if url.query().is_some() {
        format!("{}?{}", url.path(), url.query().unwrap())
    } else {
        url.path().to_string()
    };

    eprintln!(
        "{} {} {} HTTP/1.1",
        style(">").cyan().bold(),
        style(method).cyan(),
        path
    );
    eprintln!(
        "{} Host: {}",
        style(">").cyan().bold(),
        url.host_str().unwrap_or("")
    );

    for (name, value) in req.headers() {
        eprintln!(
            "{} {}: {}",
            style(">").cyan().bold(),
            name,
            value.to_str().unwrap_or("<binary>")
        );
    }

    // Basic auth from args (already in Authorization header)
    if let Some(body) = req.body() {
        if let Some(bytes) = body.as_bytes() {
            if !bytes.is_empty() {
                eprintln!("{}", style(">").cyan().bold());
                eprintln!(
                    "{}",
                    style(format!("> [body: {} bytes]", bytes.len())).dim()
                );
            }
        }
    }

    eprintln!("{}", style(">").cyan().bold());
    let _ = std::io::stderr().flush();
}

/// Print verbose response information (< lines)
pub fn print_response_verbose(resp: &Response) {
    let status = resp.status();
    eprintln!(
        "{} HTTP/1.1 {} {}",
        style("<").green().bold(),
        style(status.as_u16()).green(),
        style(status.canonical_reason().unwrap_or("")).green()
    );

    for (name, value) in resp.headers() {
        eprintln!(
            "{} {}: {}",
            style("<").green().bold(),
            name,
            value.to_str().unwrap_or("<binary>")
        );
    }

    eprintln!("{}", style("<").green().bold());
    let _ = std::io::stderr().flush();
}

/// Print response headers to stdout (for -i flag)
pub fn print_response_headers(resp: &Response) {
    let status = resp.status();
    println!(
        "HTTP/1.1 {} {}",
        status.as_u16(),
        status.canonical_reason().unwrap_or("")
    );
    for (name, value) in resp.headers() {
        println!("{}: {}", name, value.to_str().unwrap_or("<binary>"));
    }
    println!();
}

/// Expand -w format string with response variables
pub fn expand_write_out(format: &str, resp: &Response, _args: &Args) -> String {
    let status = resp.status();
    let url = resp.url().to_string();

    let mut result = format.to_string();

    // Replace known variables
    result = result.replace("%{http_code}", &status.as_u16().to_string());
    result = result.replace("%{response_code}", &status.as_u16().to_string());
    result = result.replace("%{url_effective}", &url);
    result = result.replace(
        "%{content_type}",
        resp.headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or(""),
    );

    // Handle escape sequences
    result = result.replace("\\n", "\n");
    result = result.replace("\\t", "\t");
    result = result.replace("\\r", "\r");

    result
}
