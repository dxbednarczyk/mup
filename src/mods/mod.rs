use clap::Subcommand;

mod project;

#[derive(Debug, Subcommand)]
pub enum Mod {
    /// Get information about a project
    Info {
        /// The project ID to look up
        #[arg(short, long, required = true)]
        id: String,
    },
}

pub fn action(arg: &Mod) -> Result<(), anyhow::Error> {
    match arg {
        Mod::Info { id } => project::info(id)?,
    }

    Ok(())
}
