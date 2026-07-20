use std::{env, process};

use graft_logging::{Action, help, init, parse_args};

fn main() {
    if let Err(error) = run() {
        eprintln!("{}: {error}", env!("CARGO_PKG_NAME"));
        process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let binary = env!("CARGO_PKG_NAME");

    match parse_args(env::args().skip(1))? {
        Action::Help => print!("{}", help(binary)),
        Action::Run(options) => {
            init(options)?;
            tracing::info!(component = binary, "logging initialized");
        }
    }

    Ok(())
}
