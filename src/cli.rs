use std::ffi::OsString;

use clap::{Parser, ValueEnum, builder::TypedValueParser};

use crate::client;
use crate::error::Error;
use crate::output;

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Lens {
    Default,
    Programming,
    Forums,
    Pdfs,
    NonCommercial,
    WorldNews,
}

impl Lens {
    pub fn as_api_value(self) -> &'static str {
        match self {
            Self::Default => "0",
            Self::Programming => "1",
            Self::Forums => "2",
            Self::Pdfs => "3",
            Self::NonCommercial => "4",
            Self::WorldNews => "5",
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Sort {
    Recency,
    Website,
    AdTrackers,
}

impl Sort {
    pub fn as_api_value(self) -> &'static str {
        match self {
            Self::Recency => "2",
            Self::Website => "3",
            Self::AdTrackers => "4",
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}

impl TimeRange {
    pub fn as_api_value(self) -> &'static str {
        match self {
            Self::Day => "1",
            Self::Week => "2",
            Self::Month => "3",
            Self::Year => "4",
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SummaryType {
    Summary,
    Takeaway,
}

impl SummaryType {
    pub fn as_api_value(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::Takeaway => "takeaway",
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum MapsSort {
    Relevance,
    Rating,
    Distance,
    Price,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum MapsOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "kagi-search",
    version,
    about = "Search the web with Kagi",
    arg_required_else_help = true
)]
pub struct SearchArgs {
    #[arg(value_name = "QUERY", num_args = 1.., required = true)]
    pub query: Vec<String>,

    #[arg(long, default_value_t = 10)]
    pub limit: usize,

    #[arg(long)]
    pub region: Option<String>,

    #[arg(long, value_enum)]
    pub lens: Option<Lens>,

    #[arg(long, value_enum)]
    pub sort: Option<Sort>,

    #[arg(long, value_enum, conflicts_with_all = ["from", "to"])]
    pub time: Option<TimeRange>,

    #[arg(long, value_parser = date_parser())]
    pub from: Option<String>,

    #[arg(long, value_parser = date_parser())]
    pub to: Option<String>,

    #[arg(long)]
    pub site: Option<String>,

    #[arg(long)]
    pub filetype: Option<String>,

    #[arg(long, default_value_t = false)]
    pub verbatim: bool,

    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    #[arg(long, short = 'j')]
    pub json: bool,
}

impl SearchArgs {
    /// Returns the effective output format after applying shortcut flags.
    pub fn resolved_output(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            self.output
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "kagi-summarize",
    version,
    about = "Summarize one URL with Kagi",
    arg_required_else_help = true
)]
pub struct SummarizeArgs {
    #[arg(value_name = "URL")]
    pub url: String,

    #[arg(long = "type", value_enum, default_value_t = SummaryType::Summary)]
    pub summary_type: SummaryType,

    #[arg(long, default_value = "EN")]
    pub lang: String,

    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    #[arg(long, short = 'j')]
    pub json: bool,
}

impl SummarizeArgs {
    /// Returns the effective output format after applying shortcut flags.
    pub fn resolved_output(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            self.output
        }
    }
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "kagi-maps",
    version,
    about = "Search Kagi Maps for places and addresses",
    arg_required_else_help = true
)]
pub struct MapsArgs {
    #[arg(value_name = "QUERY", num_args = 1.., required = true)]
    pub query: Vec<String>,

    #[arg(long, default_value_t = 10)]
    pub limit: usize,

    #[arg(long, value_name = "LAT,LON", value_parser = coordinate_parser())]
    pub ll: Option<String>,

    #[arg(long, value_name = "WEST,SOUTH,EAST,NORTH", value_parser = bbox_parser())]
    pub bbox: Option<String>,

    #[arg(long = "zoom", value_name = "N")]
    pub zoom: Option<f64>,

    #[arg(long, value_enum)]
    pub sort: Option<MapsSort>,

    #[arg(long, value_enum)]
    pub order: Option<MapsOrder>,

    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    #[arg(long, short = 'j')]
    pub json: bool,
}

impl MapsArgs {
    /// Returns the effective output format after applying shortcut flags.
    pub fn resolved_output(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else {
            self.output
        }
    }
}

pub async fn run_search_cli(args: impl IntoIterator<Item = OsString>) -> Result<(), Error> {
    let args = match SearchArgs::try_parse_from(args) {
        Ok(args) => args,
        Err(error) => match error.kind() {
            clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                error
                    .print()
                    .map_err(|print_error| Error::new(print_error.to_string()))?;
                return Ok(());
            }
            _ => return Err(Error::new(error.to_string())),
        },
    };

    let output = client::search(&args).await?;
    output::print_search(&output, args.resolved_output())
}

pub async fn run_maps_cli(args: impl IntoIterator<Item = OsString>) -> Result<(), Error> {
    let args = match MapsArgs::try_parse_from(args) {
        Ok(args) => args,
        Err(error) => match error.kind() {
            clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                error
                    .print()
                    .map_err(|print_error| Error::new(print_error.to_string()))?;
                return Ok(());
            }
            _ => return Err(Error::new(error.to_string())),
        },
    };

    let output = client::maps(&args).await?;
    output::print_maps(&output, args.resolved_output())
}

pub async fn run_summarize_cli(args: impl IntoIterator<Item = OsString>) -> Result<(), Error> {
    let args = match SummarizeArgs::try_parse_from(args) {
        Ok(args) => args,
        Err(error) => match error.kind() {
            clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                error
                    .print()
                    .map_err(|print_error| Error::new(print_error.to_string()))?;
                return Ok(());
            }
            _ => return Err(Error::new(error.to_string())),
        },
    };

    let output = client::summarize(&args).await?;
    output::print_summary(&output, args.resolved_output())
}

fn coordinate_parser() -> impl TypedValueParser<Value = String> {
    clap::builder::StringValueParser::new().try_map(|value| {
        let parts = parse_number_list(&value, 2, "coordinate", "LAT,LON")?;
        let lat = parts[0];
        let lon = parts[1];

        if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
            Ok(value)
        } else {
            Err(format!(
                "invalid coordinate '{value}', expected LAT,LON within latitude -90..90 and longitude -180..180"
            ))
        }
    })
}

fn bbox_parser() -> impl TypedValueParser<Value = String> {
    clap::builder::StringValueParser::new().try_map(|value| {
        let parts = parse_number_list(&value, 4, "bounding box", "WEST,SOUTH,EAST,NORTH")?;
        let west = parts[0];
        let south = parts[1];
        let east = parts[2];
        let north = parts[3];

        if !((-180.0..=180.0).contains(&west) && (-180.0..=180.0).contains(&east)) {
            return Err(format!(
                "invalid bounding box '{value}', longitude values must be within -180..180"
            ));
        }
        if !((-90.0..=90.0).contains(&south) && (-90.0..=90.0).contains(&north)) {
            return Err(format!(
                "invalid bounding box '{value}', latitude values must be within -90..90"
            ));
        }
        // west == east is degenerate; west > east is allowed for boxes crossing the antimeridian.
        if west == east {
            return Err(format!(
                "invalid bounding box '{value}', WEST and EAST must differ"
            ));
        }
        if south >= north {
            return Err(format!(
                "invalid bounding box '{value}', expected SOUTH < NORTH"
            ));
        }

        Ok(value)
    })
}

fn parse_number_list(
    value: &str,
    expected_len: usize,
    label: &str,
    expected_format: &str,
) -> Result<Vec<f64>, String> {
    let parts = value
        .split(',')
        .map(|part| part.trim().parse::<f64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("invalid {label} '{value}', expected {expected_format}"))?;

    if parts.len() == expected_len {
        Ok(parts)
    } else {
        Err(format!(
            "invalid {label} '{value}', expected {expected_format}"
        ))
    }
}

fn date_parser() -> impl TypedValueParser<Value = String> {
    clap::builder::StringValueParser::new().try_map(|value| {
        let bytes = value.as_bytes();
        let is_valid = value.len() == 10
            && bytes[4] == b'-'
            && bytes[7] == b'-'
            && bytes.iter().enumerate().all(|(index, byte)| {
                if index == 4 || index == 7 {
                    *byte == b'-'
                } else {
                    byte.is_ascii_digit()
                }
            });

        if is_valid {
            Ok(value)
        } else {
            Err(format!("invalid date '{value}', expected YYYY-MM-DD"))
        }
    })
}
