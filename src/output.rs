use crate::cli::OutputFormat;
use crate::error::Error;
use crate::parse::{SearchOutput, SummarizeOutput};

pub fn print_search(output: &SearchOutput, format: OutputFormat) -> Result<(), Error> {
    match format {
        OutputFormat::Text => {
            for (index, result) in output.results.iter().enumerate() {
                println!("{}. {}", index + 1, result.title);
                println!("   {}", result.url);
                if !result.snippet.is_empty() {
                    println!("   {}", result.snippet);
                }
                println!();
            }

            if !output.related.is_empty() {
                println!("Related: {}", output.related.join(", "));
            }

            Ok(())
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string(output)
                    .map_err(|error| Error::new(format!("Failed to encode JSON: {error}")))?
            );
            Ok(())
        }
    }
}

pub fn print_summary(output: &SummarizeOutput, format: OutputFormat) -> Result<(), Error> {
    match format {
        OutputFormat::Text => {
            println!("{}", output.summary);
            Ok(())
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string(output)
                    .map_err(|error| Error::new(format!("Failed to encode JSON: {error}")))?
            );
            Ok(())
        }
    }
}
