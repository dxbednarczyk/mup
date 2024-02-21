use anyhow::anyhow;
use clap::Subcommand;

mod eula;
pub mod lockfile;

use lockfile::Lockfile;

use crate::{loader, project};

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

    /// Install all mods from the current lockfile
    Install,
}

pub fn action(server: &Server) -> Result<(), anyhow::Error> {
    match server {
        Server::Init {
            minecraft_version,
            loader,
        } => init(minecraft_version, loader),
        Server::Sign => eula::sign(),
        Server::Install => install(),
    }
}

fn init(minecraft_version: &str, loader: &str) -> Result<(), anyhow::Error> {
    let mut lf = Lockfile::with_params(minecraft_version, loader)?;

    if !lf.is_initialized() {
        return Err(anyhow!(
            "lockfile was initialized with invalid configuration"
        ));
    }

    loader::fetch(
        Some(&lf.loader.name),
        &lf.loader.minecraft_version,
        &lf.loader.version,
    )?;

    eula::sign()?;

    Ok(())
}

fn install() -> Result<(), anyhow::Error> {
    let mut lf = Lockfile::init()?;
    if !lf.is_initialized() {
        return Err(anyhow!("failed to read lockfile"));
    }

    _ = loader::fetch(
        Some(&lf.loader.name),
        &lf.loader.minecraft_version,
        &lf.loader.version,
    )?;

    for entry in &lf.project {
        _ = project::actions::fetch(&lf, &entry.slug, Some(entry.installed_version.clone()))?;
    }

    eula::sign()?;

    Ok(())
}
