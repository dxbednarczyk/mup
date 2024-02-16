use std::{fs::File, io};

use anyhow::anyhow;
use serde::Deserialize;

use super::Loader;

const BASE_URL: &str = "https://meta.fabricmc.net/v2/versions";

#[derive(Clone, Deserialize)]
struct Version {
    version: String,
}

#[derive(Deserialize)]
struct Installers(Vec<Installer>);

#[derive(Clone, Deserialize)]
struct Installer {
    pub version: String,
}

pub fn fetch(minecraft_version: &str, loader_version: &str) -> Result<Loader, anyhow::Error> {
    let game = get_version("/game", minecraft_version)?.version;
    let loader = get_version("/loader", loader_version)?.version;
    let installer = get_installer()?.version;

    let formatted_url = format!("{BASE_URL}/loader/{game}/{loader}/{installer}/server/jar");

    println!("Downloading jarfile");
    let resp = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?;

    let mut file = File::create("fabric.jar")?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(Loader::Fabric {
        minecraft_version: game,
        loader_version: loader,
    })
}

fn get_version(path: &str, version: &str) -> Result<Version, anyhow::Error> {
    let stripped = path.strip_prefix('/').unwrap();

    println!("Fetching information for {stripped} version {version}",);

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

    let resp: Installers = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    resp.0
        .first()
        .ok_or_else(|| anyhow!("failed to retrieve latest installer"))
        .cloned()
}
