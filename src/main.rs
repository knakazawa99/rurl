mod cli;
mod client;
mod error;
mod output;
mod progress;
mod request;
mod response;

use clap::Parser;

use cli::Args;
use output::print_request_verbose;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let exit_code = run(args).await;
    std::process::exit(exit_code);
}

async fn run(args: Args) -> i32 {
    match execute(&args).await {
        Ok(code) => code,
        Err(e) => {
            if !args.silent {
                eprintln!("rurl: {e}");
            }
            e.exit_code()
        }
    }
}

async fn execute(args: &Args) -> Result<i32, error::RurlError> {
    // Build client
    let client = client::build_client(args)?;

    // Build request
    let req = request::build_request(&client, args).await?;

    // Verbose: print request info before sending
    if args.verbose {
        print_request_verbose(&req, args);
    }

    // Execute request
    let resp = client
        .execute(req)
        .await
        .map_err(error::RurlError::from)?;

    // Handle response output
    response::handle_response(resp, args).await
}
