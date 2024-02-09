#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(lazy_cell)]

use clap::{Parser, Subcommand};

mod loader;
mod project;
mod server;

#[derive(Debug, Parser)]
#[command(author = "Damian Bednarczyk <damian@bednarczyk.xyz>")]
#[command(version = "0.1.0")]
#[command(about = "A swiss army knife for Minecraft servers.")]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Download a modloader jarfile
    #[command(subcommand)]
    Loader(loader::Loader),

    /// Work with Modrinth plugins and mods
    #[command(subcommand)]
    Project(project::Project),

    /// Initialize and configure a server
    #[command(subcommand)]
    Server(server::Server),
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Loader(l)) => {
            loader::fetch(l)?;
            return Ok(());
        }
        Some(Commands::Project(p)) => project::action(p)?,
        Some(Commands::Server(s)) => server::action(s)?,
        None => (),
    }

    Ok(())
}
