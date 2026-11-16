use clap::{Parser, Subcommand};
use engine_zero::{config::Config, copy};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
struct Cli {
    #[arg(long, global = true)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Copy,
    Pull,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let path = cli.config.unwrap_or_else(|| {
        PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
            .join(".config/engine-zero/config.json")
    });
    let result = Config::load(&path).and_then(|config| match cli.command {
        Command::Copy => copy::stamp_now().and_then(|stamp| copy::copy(&config, &stamp)),
        Command::Pull => copy::pull(&config),
    });
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("engine-zero: {error}");
            ExitCode::FAILURE
        }
    }
}
