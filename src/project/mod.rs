use anyhow::anyhow;
use clap::Subcommand;

use crate::server::lockfile::Lockfile;

pub mod actions;

#[derive(Debug, Subcommand)]
pub enum Project {
    /// Add mods or plugins, including its dependencies
    Add {
        /// The project ID or slug
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
    /// Remove mods or plugins
    Remove {
        /// The project ID or slug
        id: String,

        /// Keep the downloaded jarfile
        #[arg(long, action)]
        keep_jarfile: bool,
    },
    /// Update all mods or plogins
    Update,
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
        Project::Remove { id, keep_jarfile } => actions::remove(&mut lf, id, *keep_jarfile)?,
        Project::Update => actions::update(&mut lf),
    }

    Ok(())
}
