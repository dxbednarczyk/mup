use clap::Subcommand;
use serde::{Deserialize, Serialize};

mod fabric;
mod forge;
mod paper;

#[derive(Debug, Deserialize, Serialize, Subcommand)]
pub enum Loader {
    /// Performance-optimized Spigot server
    Paper {
        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: String,

        /// Build to target
        #[arg(short, long, default_value = "latest")]
        build: String,
    },
    /// Lightweight, flexible mod loader
    Fabric {
        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: String,

        /// Loader version to target
        #[arg(short, long, default_value = "latest")]
        loader_version: String,

        /// Installer version to target
        #[arg(short, long, default_value = "latest")]
        installer_version: String,

        /// Allow nightly builds and Minecraft snapshots to be targeted
        #[arg(short, long, action)]
        allow_experimental: bool,
    },
    /// The most popular Minecraft mod loader
    Forge {
        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: String,

        /// Installer version to target
        #[arg(short, long, default_value = "recommended")]
        installer_version: String,

        /// Use the latest installer, regardless of if there is a recommended version
        #[arg(short, long, action)]
        force_latest: bool,
    },
    #[clap(skip)]
    None,
}

impl Loader {
    pub const NAMES: [&'static str; 3] = ["paper", "fabric", "forge"];

    pub fn minecraft_version(&self) -> &String {
        match self {
            Self::Fabric {
                minecraft_version, ..
            }
            | Self::Forge {
                minecraft_version, ..
            }
            | Self::Paper {
                minecraft_version, ..
            } => minecraft_version,
            Self::None => unimplemented!(),
        }
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fabric { .. } => f.write_str("fabric"),
            Self::Paper { .. } => f.write_str("paper"),
            Self::Forge { .. } => f.write_str("forge"),
            Self::None => f.write_str("none"),
        }
    }
}

pub fn fetch(loader: &Loader) -> Result<Loader, anyhow::Error> {
    match loader {
        Loader::Paper {
            minecraft_version,
            build,
        } => paper::fetch(minecraft_version, build),
        Loader::Fabric {
            minecraft_version,
            loader_version,
            installer_version,
            allow_experimental,
        } => fabric::fetch(
            minecraft_version,
            loader_version,
            installer_version,
            *allow_experimental,
        ),
        Loader::Forge {
            minecraft_version,
            installer_version,
            force_latest,
        } => forge::fetch(minecraft_version, installer_version, *force_latest),
        Loader::None => Ok(Loader::None),
    }
}
