use anyhow::anyhow;
use serde::{Deserialize, Serialize};

mod fabric;
mod forge;
mod paper;
mod neoforge;

#[derive(Debug, Deserialize, Serialize)]
pub struct Loader {
    pub name: String,
    pub minecraft_version: String,
    pub version: String,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            name: String::default(),
            minecraft_version: String::from("latest"),
            version: String::from("latest"),
        }
    }
}

impl Loader {
    pub const VALID_LOADERS: [&'static str; 4] = ["fabric", "forge", "paper", "neoforge"];

    pub fn project_path(&self) -> String {
        match self.name.as_str() {
            "fabric" | "forge" => String::from("./mods/"),
            "paper" => String::from("./plugins/"),
            _ => unimplemented!(),
        }
    }
}

pub fn fetch(
    loader: Option<&String>,
    minecraft_version: &str,
    version: &str,
) -> Result<Loader, anyhow::Error> {
    if loader.is_none() {
        return Err(anyhow!("no loader provided"));
    };

    match loader.unwrap().as_str() {
        "paper" => paper::fetch(minecraft_version, version),
        "fabric" => fabric::fetch(minecraft_version, version),
        "forge" => forge::fetch(minecraft_version, version),
        "neoforge" => neoforge::fetch(minecraft_version),
        l => Err(anyhow!("{l} is currently unsupported")),
    }
}

pub fn list() {
    println!("{}", Loader::VALID_LOADERS.join(", "));
}
