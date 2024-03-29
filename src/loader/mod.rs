use anyhow::{anyhow, Result};

mod fabric;
mod forge;
mod neoforge;
mod paper;

const VALID_LOADERS: [&str; 4] = ["fabric", "forge", "paper", "neoforge"];

pub fn fetch(loader: &str, minecraft_version: &str, version: &str) -> Result<()> {
    match loader {
        "paper" => paper::fetch(minecraft_version, version),
        "fabric" => fabric::fetch(minecraft_version, version),
        "forge" => forge::fetch(minecraft_version, version),
        "neoforge" => neoforge::fetch(minecraft_version),
        l => Err(anyhow!("{l} is currently unsupported")),
    }
}

pub fn location(loader: &str) -> &str {
    match loader {
        "paper" => "plugins",
        "fabric" | "forge" | "neoforge" => "mods",
        _ => unreachable!(),
    }
}

pub fn parse(input: &str) -> Result<String> {
    if !VALID_LOADERS.contains(&input) {
        return Err(anyhow!("try one of {VALID_LOADERS:?}"));
    }

    Ok(input.to_string())
}
