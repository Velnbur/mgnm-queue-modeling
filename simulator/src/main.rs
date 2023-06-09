use clap::Parser;
use cli::Cli;

mod actions;
mod broadcaster;
mod cli;
mod config;
mod stats;

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    cli.run()
}
