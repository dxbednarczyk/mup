use anyhow::anyhow;
use clap::Subcommand;

mod eula;
pub mod lockfile;

use lockfile::Lockfile;

#[derive(Debug, Subcommand)]
pub enum Server {
    /// Initialize a server in the current directory
    Init {
        /// Minecraft version of the server
        #[arg(short, long, required = true)]
        minecraft_version: String,

        /// Which loader to use
        #[arg(short, long, required = true)]
        loader: String,
    },

    /// Sign the eula.txt
    Sign,
}

pub fn action(server: &Server) -> Result<(), anyhow::Error> {
    match server {
        Server::Init {
            minecraft_version,
            loader,
        } => init(minecraft_version, loader),
        Server::Sign => eula::sign(),
    }
}

fn init(minecraft_version: &str, loader: &str) -> Result<(), anyhow::Error> {
    let mut lf = Lockfile::with_params(minecraft_version, loader)?;
    if !lf.is_initialized() {
        return Err(anyhow!(
            "lockfile was initialized with invalid configuration"
        ));
    }

    Ok(())
}
