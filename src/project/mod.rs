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

        /// Remove orphans (dependencies which are not required by anything after removal)
        #[arg(long, action)]
        remove_orphans: bool,
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
        } => actions::add(
            &mut lf,
            id,
            version_id.as_ref().map(String::as_str),
            *optional_deps,
            *no_deps,
        )?,
        Project::Remove {
            id,
            keep_jarfile,
            remove_orphans,
        } => lf.remove(id, *keep_jarfile, *remove_orphans)?,
    }

    Ok(())
}
