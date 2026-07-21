use std::{env, process};

use graft_error::ErrorReport;
use graft_logging::{Action, help, init, parse_args};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(2);
    }
}

fn run() -> Result<(), ErrorReport> {
    let binary = env!("CARGO_PKG_NAME");

    match parse_args(binary, env::args().skip(1))? {
        Action::Help => print!("{}", help(binary)),
        Action::Run(options) => {
            init(binary, options)?;
            tracing::info!(component = binary, "logging initialized");
        }
    }

    Ok(())
}
