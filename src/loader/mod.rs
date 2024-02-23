use anyhow::anyhow;

mod fabric;
mod forge;
mod neoforge;
mod paper;

pub fn fetch(
    loader: Option<&String>,
    minecraft_version: &str,
    version: &str,
) -> Result<(), anyhow::Error> {
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
