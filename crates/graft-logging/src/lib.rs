use std::str::FromStr;

use graft_error::ErrorReport;
use tracing::Level;

pub const INVALID_ARGUMENTS: &str = "GRAFT-CLI-0001";
pub const LOGGING_INIT_FAILED: &str = "GRAFT-LOG-0001";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogFormat {
    Human,
    Json,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Options {
    pub level: Level,
    pub format: LogFormat,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            format: LogFormat::Human,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Help,
    Run(Options),
}

pub fn parse_args<I, S>(binary: &str, args: I) -> Result<Action, ErrorReport>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    parse_args_inner(args).map_err(|cause| {
        ErrorReport::new(
            INVALID_ARGUMENTS,
            format!("{binary} rejected its command line"),
            format!("run '{binary} --help' and correct the arguments"),
        )
        .with_cause(cause)
    })
}

fn parse_args_inner<I, S>(args: I) -> Result<Action, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut options = Options::default();
    let mut args = args.into_iter().map(Into::into);

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "-h" | "--help" => return Ok(Action::Help),

            "--log-level" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--log-level requires a value".to_owned())?;

                options.level = parse_level(&value)?;
            }

            "--log-format" => {
                let value = args
                    .next()
                    .ok_or_else(|| "--log-format requires a value".to_owned())?;

                options.format = parse_format(&value)?;
            }

            _ => {
                if let Some(value) = argument.strip_prefix("--log-level=") {
                    options.level = parse_level(value)?;
                } else if let Some(value) = argument.strip_prefix("--log-format=") {
                    options.format = parse_format(value)?;
                } else {
                    return Err(format!("unknown argument: {argument}"));
                }
            }
        }
    }

    Ok(Action::Run(options))
}

pub fn init(binary: &str, options: Options) -> Result<(), ErrorReport> {
    let result = match options.format {
        LogFormat::Human => tracing_subscriber::fmt()
            .compact()
            .with_max_level(options.level)
            .try_init(),
        LogFormat::Json => tracing_subscriber::fmt()
            .json()
            .with_max_level(options.level)
            .try_init(),
    };

    result.map_err(|error| {
        ErrorReport::new(
            LOGGING_INIT_FAILED,
            format!("{binary} could not initialize logging"),
            "check the logging options and ensure logging is initialized only once",
        )
        .with_cause(error.to_string())
    })
}

pub fn help(binary: &str) -> String {
    format!(
        "usage: {binary} [--log-level <level>] [--log-format <format>]\n\
        \n\
        logging:\n\
            --log-level <level>    trace, debug, info, warn or error [default: info]\n\
            --log-format <format>  human or json [default: human]\n"
    )
}

fn parse_level(value: &str) -> Result<Level, String> {
    Level::from_str(&value.to_ascii_uppercase()).map_err(|_| {
        format!("invalid log level: {value}; expected trace, debug, info, warn or error")
    })
}

fn parse_format(value: &str) -> Result<LogFormat, String> {
    match value.to_ascii_lowercase().as_str() {
        "human" => Ok(LogFormat::Human),
        "json" => Ok(LogFormat::Json),
        _ => Err(format!(
            "invalid log format: {value}; expected human or json"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_human_info_defaults() {
        assert_eq!(
            parse_args("graftd", Vec::<String>::new()),
            Ok(Action::Run(Options::default()))
        );
    }

    #[test]
    fn parses_logging_options() {
        assert_eq!(
            parse_args("graftd", ["--log-level", "debug", "--log-format=json"]),
            Ok(Action::Run(Options {
                level: Level::DEBUG,
                format: LogFormat::Json
            }))
        );
    }

    #[test]
    fn rejects_missing_values() {
        let error = parse_args("graftd", ["--log-level"]).unwrap_err();

        assert_eq!(error.code(), INVALID_ARGUMENTS);
        assert_eq!(error.causes(), ["--log-level requires a value"]);
    }

    #[test]
    fn rejected_unknown_arguments() {
        let error = parse_args(
            "graftd",
            ["--definitely-not-real-hello-guys-how-are-you-this-is-a-stupid-test-god-i-hate-writing-these"],
        )
        .unwrap_err();

        assert_eq!(error.code(), INVALID_ARGUMENTS);
        assert_eq!(
            error.causes(),
            [
                "unknown argument: --definitely-not-real-hello-guys-how-are-you-this-is-a-stupid-test-god-i-hate-writing-these"
            ]
        );
    }
}
