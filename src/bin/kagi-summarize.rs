use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    match kagi::run_summarize_cli(std::env::args_os()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("kagi-summarize: {error}");
            ExitCode::FAILURE
        }
    }
}
