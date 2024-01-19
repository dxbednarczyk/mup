use clap::{Parser, Subcommand};

mod eula;
mod loader;
mod project;

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
    /// Download a server or modloader jarfile
    #[command(subcommand)]
    Loader(loader::Loader),

    /// Work with Modrinth plugins and mods
    #[command(subcommand)]
    Project(project::Project),

    /// Sign the eula in the current directory
    Eula,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Loader(l)) => loader::fetch(l)?,
        Some(Commands::Project(p)) => project::action(p)?,
        Some(Commands::Eula) => eula::sign()?,
        None => (),
    }

    Ok(())
}
