use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let tracer_provider = kagi::init_tracing("kagi-search");
    let result = kagi::run_search_cli(std::env::args_os()).await;
    kagi::shutdown_tracing(tracer_provider);

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("kagi-search: {error}");
            ExitCode::FAILURE
        }
    }
}
