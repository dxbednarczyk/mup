use anyhow::anyhow;
use clap::Subcommand;

use crate::server::lockfile::Lockfile;

pub mod actions;

pub const BASE_URL: &str = "https://api.modrinth.com/v2";

#[derive(Debug, Subcommand)]
pub enum Project {
    /// Add a mod or plugin, including its dependencies
    Add {
        /// The project's ID or slug
        id: String,

        /// The version ID to target
        #[arg(short, long, default_value = "latest")]
        version_id: Option<String>,

        /// Also install optional dependencies
        #[arg(short, long, action)]
        optional_deps: bool,

        /// Do not install any dependencies
        #[arg(short, long, action)]
        no_deps: bool,
    },
    /// Remove a mod or plugin
    Remove {
        /// The slug of the project to remove
        #[arg(short, long, required = true)]
        slug: String,

        /// Keep the downloaded jarfile
        #[arg(long, action)]
        keep_jarfile: bool,
    },
}

pub fn action(project: &Project) -> Result<(), anyhow::Error> {
    let mut lf = Lockfile::init()?;

    if !lf.is_initialized() {
        return Err(anyhow!(
            "you must initialize a server before modifying projects"
        ));
    }

    match project {
        Project::Add {
            id,
            version_id,
            optional_deps,
            no_deps,
        } => actions::add(&mut lf, id, version_id.as_ref(), *optional_deps, *no_deps)?,
        Project::Remove { slug, keep_jarfile } => actions::remove(&mut lf, slug, *keep_jarfile)?,
    }

    Ok(())
}
