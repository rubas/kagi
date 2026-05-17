use crate::cli::OutputFormat;
use crate::error::Error;
use crate::parse::{MapsOutput, SearchOutput, SummarizeOutput};

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

pub fn print_maps(output: &MapsOutput, format: OutputFormat) -> Result<(), Error> {
    match format {
        OutputFormat::Text => {
            for (index, result) in output.results.iter().enumerate() {
                println!("{}. {}", index + 1, result.name);
                if let Some(address) = &result.address {
                    println!("   {address}");
                }
                println!(
                    "   {}, {}",
                    result.coordinates.latitude, result.coordinates.longitude
                );

                let mut details = Vec::new();
                if let Some(rating) = result.rating {
                    let rating = if let Some(review_count) = result.review_count {
                        format!("Rating: {rating} ({review_count})")
                    } else {
                        format!("Rating: {rating}")
                    };
                    details.push(rating);
                }
                if let Some(price) = &result.price {
                    details.push(format!("Price: {price}"));
                }
                if let Some(hours_now) = &result.hours_now {
                    details.push(hours_now.clone());
                }
                if !details.is_empty() {
                    println!("   {}", details.join(" | "));
                }
                if let Some(phone) = &result.phone {
                    println!("   {phone}");
                }
                if let Some(url) = &result.url {
                    println!("   {url}");
                }
                println!();
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
