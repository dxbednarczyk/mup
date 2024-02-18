use clap::Subcommand;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, VariantNames};

mod fabric;
mod forge;
mod paper;

#[derive(Clone, Debug, Default, Display, Subcommand, VariantNames)]
#[strum(serialize_all = "snake_case")]
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
    #[default]
    None,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Generic {
    pub name: String,
    pub minecraft_version: String,
    pub version: String,
}

impl Default for Generic {
    fn default() -> Self {
        Self {
            name: String::default(),
            minecraft_version: String::from("latest"),
            version: String::from("latest"),
        }
    }
}

impl From<Loader> for Generic {
    fn from(value: Loader) -> Self {
        match value {
            Loader::Fabric {
                minecraft_version,
                loader_version,
            } => Self {
                name: String::from("fabric"),
                minecraft_version,
                version: loader_version,
            },
            Loader::Forge {
                minecraft_version,
                installer_version,
            } => Self {
                name: String::from("forge"),
                minecraft_version,
                version: installer_version,
            },
            Loader::Paper {
                minecraft_version,
                build,
            } => Self {
                name: String::from("paper"),
                minecraft_version,
                version: build,
            },
            Loader::None => unimplemented!(),
        }
    }
}
impl Generic {
    pub fn project_path(&self) -> String {
        match self.name.as_str() {
            "fabric" | "forge" => String::from("./mods/"),
            "paper" => String::from("./plugins/"),
            _ => unimplemented!(),
        }
    }
}

pub fn fetch(loader: &Generic) -> Result<Loader, anyhow::Error> {
    match loader.name.as_str() {
        "paper" => paper::fetch(&loader.minecraft_version, &loader.version),
        "fabric" => fabric::fetch(&loader.minecraft_version, &loader.version),
        "forge" => forge::fetch(&loader.minecraft_version, &loader.version),
        _ => Ok(Loader::None),
    }
}
