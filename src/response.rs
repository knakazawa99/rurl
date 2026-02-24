use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::Response;

use crate::{
    cli::Args,
    error::RurlError,
    output::{expand_write_out, print_response_headers, print_response_verbose},
    progress::create_progress_bar,
};

/// Process and output the HTTP response
pub async fn handle_response(resp: Response, args: &Args) -> Result<i32, RurlError> {
    // Verbose: print response headers to stderr
    if args.verbose {
        print_response_verbose(&resp);
    }

    // Capture write-out info before consuming response
    let write_out_format = args.write_out.clone();
    let write_out_str = write_out_format
        .as_deref()
        .map(|fmt| expand_write_out(fmt, &resp, args));

    // -i: print response headers to stdout
    if args.include {
        print_response_headers(&resp);
    }

    // Determine output destination
    let output_path = resolve_output_path(args, &resp);

    // Handle HEAD request (no body)
    if args.head {
        if let Some(s) = write_out_str {
            print!("{s}");
        }
        return Ok(0);
    }

    // Determine whether to show progress bar
    let content_length = resp
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());

    // Progress bar only when writing to file and not in silent mode
    let show_progress = output_path.is_some() && !args.silent;

    // Stream response body
    // -s (silent) suppresses progress/errors but body still goes to stdout
    let exit_code = if let Some(path) = &output_path {
        write_to_file(resp, path, content_length, show_progress).await?
    } else {
        write_to_stdout(resp).await?
    };

    // -w write-out
    if let Some(s) = write_out_str {
        print!("{s}");
    }

    Ok(exit_code)
}

/// Determine output file path from -o / -O flags
fn resolve_output_path(args: &Args, resp: &Response) -> Option<String> {
    if let Some(ref path) = args.output {
        return Some(path.clone());
    }
    if args.remote_name {
        let url = resp.url();
        let filename = url
            .path_segments()
            .and_then(|mut segs| segs.next_back())
            .filter(|s| !s.is_empty())
            .unwrap_or("index");
        return Some(filename.to_string());
    }
    None
}

/// Stream response body to stdout
async fn write_to_stdout(resp: Response) -> Result<i32, RurlError> {
    use std::io::Write;
    let mut stream = resp.bytes_stream();
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    while let Some(chunk) = stream.next().await {
        let chunk: Bytes = chunk.map_err(RurlError::from)?;
        handle.write_all(&chunk).map_err(RurlError::Write)?;
    }
    handle.flush().map_err(RurlError::Write)?;
    Ok(0)
}

/// Stream response body to a file, with optional progress bar
async fn write_to_file(
    resp: Response,
    path: &str,
    content_length: Option<u64>,
    show_progress: bool,
) -> Result<i32, RurlError> {
    use tokio::io::AsyncWriteExt;

    let pb = if show_progress {
        Some(create_progress_bar(content_length))
    } else {
        None
    };

    let mut file = tokio::fs::File::create(path)
        .await
        .map_err(RurlError::Write)?;

    let mut stream = resp.bytes_stream();
    let mut written: u64 = 0;

    while let Some(chunk) = stream.next().await {
        let chunk: Bytes = chunk.map_err(RurlError::from)?;
        file.write_all(&chunk).await.map_err(RurlError::Write)?;
        written += chunk.len() as u64;
        if let Some(ref pb) = pb {
            pb.set_position(written);
        }
    }

    file.flush().await.map_err(RurlError::Write)?;

    if let Some(pb) = pb {
        pb.finish_with_message(format!("Saved to {}", path));
    }

    Ok(0)
}

