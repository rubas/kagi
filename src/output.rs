use std::borrow::Cow;
use std::io::{self, Write};

use serde::Serialize;

use crate::cli::OutputFormat;
use crate::error::Error;
use crate::parse::{MapsOutput, SearchOutput, SummarizeOutput};

/// Strips control characters (keeping `\n` and `\t`) from server-derived
/// text before it reaches a terminal, where embedded ANSI/OSC sequences
/// could rewrite output, retitle the window, or write to the clipboard.
/// JSON output does not need this; serde_json escapes control characters.
pub(crate) fn sanitize(text: &str) -> Cow<'_, str> {
    let keep = |c: char| !c.is_control() || c == '\n' || c == '\t';
    if text.chars().all(keep) {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(text.chars().filter(|&c| keep(c)).collect())
    }
}

pub fn print_search(output: &SearchOutput, format: OutputFormat) -> Result<(), Error> {
    let mut stdout = io::stdout().lock();
    let result = match format {
        OutputFormat::Text => write_search_text(&mut stdout, output),
        OutputFormat::Json => writeln!(stdout, "{}", encode_json(output)?),
    };
    finish(result)
}

pub fn print_summary(output: &SummarizeOutput, format: OutputFormat) -> Result<(), Error> {
    let mut stdout = io::stdout().lock();
    let result = match format {
        OutputFormat::Text => writeln!(stdout, "{}", sanitize(&output.summary)),
        OutputFormat::Json => writeln!(stdout, "{}", encode_json(output)?),
    };
    finish(result)
}

pub fn print_maps(output: &MapsOutput, format: OutputFormat) -> Result<(), Error> {
    let mut stdout = io::stdout().lock();
    let result = match format {
        OutputFormat::Text => write_maps_text(&mut stdout, output),
        OutputFormat::Json => writeln!(stdout, "{}", encode_json(output)?),
    };
    finish(result)
}

fn write_search_text(out: &mut impl Write, output: &SearchOutput) -> io::Result<()> {
    for (index, result) in output.results.iter().enumerate() {
        writeln!(out, "{}. {}", index + 1, sanitize(&result.title))?;
        writeln!(out, "   {}", sanitize(&result.url))?;
        if !result.snippet.is_empty() {
            writeln!(out, "   {}", sanitize(&result.snippet))?;
        }
        writeln!(out)?;
    }

    if !output.related.is_empty() {
        let related: Vec<Cow<'_, str>> = output.related.iter().map(|term| sanitize(term)).collect();
        writeln!(out, "Related: {}", related.join(", "))?;
    }

    Ok(())
}

fn write_maps_text(out: &mut impl Write, output: &MapsOutput) -> io::Result<()> {
    for (index, result) in output.results.iter().enumerate() {
        writeln!(out, "{}. {}", index + 1, sanitize(&result.name))?;
        if let Some(address) = &result.address {
            writeln!(out, "   {}", sanitize(address))?;
        }
        writeln!(
            out,
            "   {}, {}",
            result.coordinates.latitude, result.coordinates.longitude
        )?;

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
            details.push(format!("Price: {}", sanitize(price)));
        }
        if let Some(hours_now) = &result.hours_now {
            details.push(sanitize(hours_now).into_owned());
        }
        if !details.is_empty() {
            writeln!(out, "   {}", details.join(" | "))?;
        }
        if let Some(phone) = &result.phone {
            writeln!(out, "   {}", sanitize(phone))?;
        }
        if let Some(url) = &result.url {
            writeln!(out, "   {}", sanitize(url))?;
        }
        writeln!(out)?;
    }

    Ok(())
}

fn encode_json(value: &impl Serialize) -> Result<String, Error> {
    serde_json::to_string(value)
        .map_err(|error| Error::new(format!("Failed to encode JSON: {error}")))
}

/// A closed read end (e.g. `kagi-search … | head`) is a normal way for a
/// pipeline to finish, not an error; anything else writing to stdout is.
fn finish(result: io::Result<()>) -> Result<(), Error> {
    match result {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(Error::new(format!("failed to write output: {error}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::SearchResult;

    #[test]
    fn sanitize_strips_escape_and_control_bytes() {
        // Removing ESC and BEL defuses the OSC sequence; the leftover
        // payload is harmless plain text.
        assert_eq!(sanitize("\u{1b}]0;owned\u{7}title"), "]0;ownedtitle");
        assert_eq!(sanitize("plain text"), "plain text");
        assert_eq!(
            sanitize("keeps\nnewline\tand tab"),
            "keeps\nnewline\tand tab"
        );
    }

    #[test]
    fn search_text_output_strips_escape_sequences() {
        let output = SearchOutput {
            results: vec![SearchResult {
                url: "https://example.com".into(),
                title: "\u{1b}]0;owned\u{7}Example".into(),
                snippet: String::new(),
            }],
            related: Vec::new(),
        };

        let mut buffer = Vec::new();
        write_search_text(&mut buffer, &output).unwrap();
        let text = String::from_utf8(buffer).unwrap();

        assert!(text.contains("Example"));
        assert!(!text.contains('\u{1b}'));
        assert!(!text.contains('\u{7}'));
    }
}
