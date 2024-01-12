use clap::{Parser, Subcommand};

mod loader;

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
    #[command(subcommand)]
    Loader(loader::Loader),
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Loader(l)) => loader::fetch(l)?,
        None => (),
    }

    Ok(())
}
