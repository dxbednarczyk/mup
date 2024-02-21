use anyhow::anyhow;
use serde::{Deserialize, Serialize};

mod fabric;
mod forge;
mod paper;

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
    pub fn project_path(&self) -> String {
        match self.name.as_str() {
            "fabric" | "forge" => String::from("./mods/"),
            "paper" => String::from("./plugins/"),
            _ => unimplemented!(),
        }
    }
}

pub fn fetch(
    loader: &str,
    minecraft_version: &str,
    version: &str,
) -> Result<Loader, anyhow::Error> {
    match loader {
        "paper" => paper::fetch(minecraft_version, version),
        "fabric" => fabric::fetch(minecraft_version, version),
        "forge" => forge::fetch(minecraft_version, version),
        _ => Err(anyhow!("{loader} is currently unsupported")),
    }
}
