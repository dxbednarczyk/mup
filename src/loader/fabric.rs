use std::{fs::File, io};

use anyhow::anyhow;
use serde::Deserialize;

use super::Loader;

const BASE_URL: &str = "https://meta.fabricmc.net/v2/versions";

#[derive(Clone, Debug, Deserialize)]
struct Version {
    version: String,
    stable: bool,
}

pub fn fetch(
    minecraft_version: &str,
    loader_version: &str,
    installer_version: &str,
    allow_experimental: bool,
) -> Result<Loader, anyhow::Error> {
    let mc = get_specific_version("/game", minecraft_version, allow_experimental)?.version;
    let l = get_specific_version("/loader", loader_version, allow_experimental)?.version;
    let i = get_specific_version("/installer", installer_version, allow_experimental)?.version;

    let formatted_url = format!("{BASE_URL}/loader/{mc}/{l}/{i}/server/jar",);

    println!("Downlaoding jarfile");
    let resp = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?;

    let mut file = File::create("fabric.jar")?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(Loader::Fabric {
        minecraft_version: mc,
        loader_version: l,
        installer_version: i,
        allow_experimental,
    })
}

fn get_specific_version(
    path: &str,
    version: &str,
    allow_experimental: bool,
) -> Result<Version, anyhow::Error> {
    println!("Fetching information for {} version {version}", path.strip_prefix('/').unwrap());
    let versions: Vec<Version> = ureq::get(&format!("{BASE_URL}{path}"))
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if version == "latest" {
        return get_latest_version(&versions, allow_experimental);
    }

    let specific_version = versions.iter().find(|p| p.version == version);

    if specific_version.is_none() {
        return Err(anyhow!(
            "{} version {version} does not exist",
            path.strip_prefix('/').unwrap()
        ));
    }

    Ok(specific_version.unwrap().clone())
}

fn get_latest_version(
    versions: &[Version],
    allow_experimental: bool,
) -> Result<Version, anyhow::Error> {
    let latest_version = if allow_experimental {
        versions.first()
    } else {
        versions.iter().find(|x| x.stable)
    };

    if latest_version.is_none() {
        return Err(anyhow!("failed to fetch requested minecraft version"));
    }

    Ok(latest_version.unwrap().clone())
}
