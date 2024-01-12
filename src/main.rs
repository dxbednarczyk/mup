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
    /// Downloads a server jarfile
    Loader {
        /// Which loader to download
        #[arg(short, long, value_enum)]
        name: loader::Loader,

        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: Option<String>,

        /// Version of the loader to target
        #[arg(short, long, default_value = "latest")]
        loader_version: Option<String>,

        /// Get the very latest build, including Minecraft snapshots or nightly loader builds
        #[arg(short, long, action)]
        allow_experimental: bool,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Loader {
            name,
            minecraft_version,
            loader_version,
            allow_experimental,
        }) => {
            loader::fetch(
                name,
                minecraft_version.as_ref().unwrap(),
                loader_version.as_ref().unwrap(),
                allow_experimental,
            )?;
        }
        None => (),
    }

    Ok(())
}
