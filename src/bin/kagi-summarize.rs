use std::process::ExitCode;

// One sequential HTTP request per invocation; a worker thread per core
// would be pure startup overhead for an agent-invoked CLI.
#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match kagi::run_summarize_cli(std::env::args_os()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("kagi-summarize: {error}");
            ExitCode::FAILURE
        }
    }
}
