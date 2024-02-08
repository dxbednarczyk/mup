use clap::Subcommand;

mod eula;
pub mod lockfile;

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
}

pub fn action(server: &Server) -> Result<(), anyhow::Error> {
    match server {
        Server::Init {
            minecraft_version,
            loader,
        } => {
            lockfile::Lockfile::with_params(minecraft_version, loader)?;
            Ok(())
        }
        Server::Sign => eula::sign(),
    }
}
