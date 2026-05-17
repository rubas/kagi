//! Binary entry point for `kagi-maps`.

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match kagi::run_maps_cli(std::env::args_os()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("kagi-maps: {error}");
            ExitCode::FAILURE
        }
    }
}
