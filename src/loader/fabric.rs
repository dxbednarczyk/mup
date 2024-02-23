use std::{fs::File, io};

use anyhow::anyhow;
use log::info;
use serde::Deserialize;

const BASE_URL: &str = "https://meta.fabricmc.net/v2/versions";

#[derive(Clone, Deserialize)]
struct Version {
    version: String,
}

#[derive(Clone, Deserialize)]
struct Installer {
    pub version: String,
}

pub fn fetch(minecraft_version: &str, loader_version: &str) -> Result<(), anyhow::Error> {
    let game = get_version("/game", minecraft_version)?.version;
    let loader = get_version("/loader", loader_version)?.version;
    let installer = get_installer()?.version;

    let formatted_url = format!("{BASE_URL}/loader/{game}/{loader}/{installer}/server/jar");

    info!("downloading jarfile");

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?;

    let mut file = File::create("fabric.jar")?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(())
}

fn get_version(path: &str, version: &str) -> Result<Version, anyhow::Error> {
    let stripped = path.strip_prefix('/').unwrap();

    info!("fetching information for {stripped} version {version}");

    let versions: Vec<Version> = ureq::get(&format!("{BASE_URL}{path}"))
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if version == "latest" {
        return versions
            .first()
            .ok_or_else(|| anyhow!("failed to fetch requested minecraft version"))
            .cloned();
    }

    versions
        .iter()
        .find(|p| p.version == version)
        .ok_or_else(|| anyhow!("{stripped} version {version} does not exist"))
        .cloned()
}

fn get_installer() -> Result<Installer, anyhow::Error> {
    let formatted_url = format!("{BASE_URL}/installer");

    info!("fetching latest installer");

    let resp: Vec<Installer> = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    return resp
        .first()
        .ok_or_else(|| anyhow!("failed to retrieve latest installer"))
        .cloned();
}
