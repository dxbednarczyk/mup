use clap::Subcommand;

mod actions;
mod lockfile;

pub const BASE_URL: &str = "https://api.modrinth.com/v2";

#[derive(Debug, Subcommand)]
pub enum Project {
    /// Add a mod or plugin
    Add {
        /// The project ID or slug to target
        #[arg(short, long, required = true)]
        id: String,

        /// Minecraft version to target
        #[arg(short, long, required = true)]
        minecraft_version: String,

        /// Project version ID to target
        #[arg(short, long, default_value = "latest")]
        version_id: Option<String>,

        /// If a project supports multiple loaders, specify which to target
        #[arg(short, long)]
        loader: Option<String>,
    },
}

pub fn action(project: &Project) -> Result<(), anyhow::Error> {
    match project {
        Project::Add {
            id,
            minecraft_version,
            version_id,
            loader
        } => actions::add(id, minecraft_version, version_id, loader)?,
    }

    Ok(())
}
