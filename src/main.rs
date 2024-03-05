#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![feature(lazy_cell)]

use std::env;

use clap::{Parser, Subcommand};

mod loader;
mod project;
mod server;

use anyhow::Result;

#[derive(Debug, Parser)]
#[command(author = "Damian Bednarczyk <damian@bednarczyk.xyz>")]
#[command(version = "0.1.0")]
#[command(about = "A swiss army knife for Minecraft servers.")]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, action)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Download a modloader jarfile
    Loader {
        /// Name of the loader to download
        #[arg(value_name = "loader", value_parser = loader::parse)]
        name: String,

        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: String,

        /// Loader version to target
        #[arg(short, long, default_value = "latest")]
        version: String,
    },

    /// Work with Modrinth plugins and mods
    #[command(subcommand)]
    Project(project::Project),

    /// Initialize and configure a server
    #[command(subcommand)]
    Server(server::Server),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.verbose {
        env::set_var("RUST_LOG", String::from("info"));
    }

    pretty_env_logger::init();

    match &cli.command {
        Some(Commands::Loader {
            name,
            minecraft_version,
            version,
        }) => loader::fetch(name, minecraft_version, version)?,
        Some(Commands::Project(p)) => project::action(p)?,
        Some(Commands::Server(s)) => server::action(s)?,
        None => (),
    }

    Ok(())
}
