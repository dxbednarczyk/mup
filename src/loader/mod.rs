use anyhow::anyhow;

mod fabric;
mod forge;
mod neoforge;
mod paper;

pub fn fetch(
    loader: &str,
    minecraft_version: &str,
    version: &str,
) -> Result<(), anyhow::Error> {
    match loader {
        "paper" => paper::fetch(minecraft_version, version),
        "fabric" => fabric::fetch(minecraft_version, version),
        "forge" => forge::fetch(minecraft_version, version),
        "neoforge" => neoforge::fetch(minecraft_version),
        l => Err(anyhow!("{l} is currently unsupported")),
    }
}
