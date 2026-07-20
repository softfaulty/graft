use std::str::FromStr;

use tracing::Level;

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

pub fn parse_args<I, S>(args: I) -> Result<Action, String>
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

pub fn init(options: Options) -> Result<(), String> {
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

    result.map_err(|error| format!("failed to init logger: {error}"))
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
            parse_args(Vec::<String>::new()),
            Ok(Action::Run(Options::default()))
        );
    }

    #[test]
    fn parses_logging_options() {
        assert_eq!(
            parse_args(["--log-level", "debug", "--log-format=json"]),
            Ok(Action::Run(Options {
                level: Level::DEBUG,
                format: LogFormat::Json
            }))
        );
    }

    #[test]
    fn rejects_missing_values() {
        assert_eq!(
            parse_args(["--log-level"]),
            Err("--log-level requires a value".to_owned())
        );
    }

    #[test]
    fn rejected_unknown_arguments() {
        assert_eq!(
            parse_args(["--definitely-not-real-hello-guys-how-are-you-this-is-a-stupid-test-god-i-hate-writing-these"]),
            Err("unknown argument: --definitely-not-real-hello-guys-how-are-you-this-is-a-stupid-test-god-i-hate-writing-these".to_owned())
        )
    }
}
