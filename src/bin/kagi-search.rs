use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match kagi::run_search_cli(std::env::args_os()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("kagi-search: {error}");
            ExitCode::FAILURE
        }
    }
}
