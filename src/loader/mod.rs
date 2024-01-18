use clap::Subcommand;

mod fabric;
mod forge;
mod paper;

#[derive(Debug, Subcommand)]
pub enum Loader {
    /// Performance-optimized Spigot server
    Paper {
        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: Option<String>,

        /// Build to target
        #[arg(short, long, default_value = "latest")]
        build_version: Option<String>,
    },
    /// Lightweight, flexible mod loader
    Fabric {
        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: Option<String>,

        /// Loader version to target
        #[arg(short, long, default_value = "latest")]
        loader_version: Option<String>,

        /// Installer version to target
        #[arg(short, long, default_value = "latest")]
        installer_version: Option<String>,

        /// Allow nightly builds and Minecraft snapshots to be targeted
        #[arg(short, long, action)]
        allow_experimental: bool,
    },
    /// The most popular Minecraft mod loader
    Forge {
        /// Minecraft version to target
        #[arg(short, long, default_value = "latest")]
        minecraft_version: Option<String>,

        /// Installer version to target
        #[arg(short, long, default_value = "recommended")]
        installer_version: Option<String>,

        /// Use the latest installer, regardless of if there is a recommended version
        #[arg(short, long, action)]
        force_latest: bool,
    },
}

pub fn fetch(loader: &Loader) -> Result<(), anyhow::Error> {
    match loader {
        Loader::Paper {
            minecraft_version,
            build_version,
        } => paper::fetch(minecraft_version, build_version),
        Loader::Fabric {
            minecraft_version,
            loader_version,
            installer_version,
            allow_experimental,
        } => fabric::fetch(
            minecraft_version,
            loader_version,
            installer_version,
            allow_experimental,
        ),
        Loader::Forge {
            minecraft_version,
            installer_version,
            force_latest,
        } => forge::fetch(minecraft_version, installer_version, force_latest),
    }
}
