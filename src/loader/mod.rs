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
    },
    /// The most popular Minecraft mod loader
    Forge {
        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: String,

        /// Installer version to target
        #[arg(short, long, default_value = "latest")]
        installer_version: String,
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

impl Loader {
    pub fn with_params(loader: &str, minecraft_version: String) -> Self {
        match loader {
            "paper" => Self::Paper {
                minecraft_version,
                build: String::new(),
            },
            "fabric" => Self::Fabric {
                minecraft_version,
                loader_version: String::new(),
            },
            "forge" => Self::Forge {
                minecraft_version,
                installer_version: String::new(),
            },
            _ => unimplemented!(),
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
        } => fabric::fetch(minecraft_version, loader_version),
        Loader::Forge {
            minecraft_version,
            installer_version,
        } => forge::fetch(minecraft_version, installer_version),
        Loader::None => Ok(Loader::None),
    }
}
