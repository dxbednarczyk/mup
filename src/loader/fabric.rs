use std::{fs::File, io};

use anyhow::anyhow;
use serde::Deserialize;

const BASE_URL: &str = "https://meta.fabricmc.net/v2/versions";

#[derive(Clone, Debug, Deserialize)]
struct Version {
    version: String,
    stable: bool,
}

pub fn fetch(
    minecraft_input: &Option<String>,
    loader_input: &Option<String>,
    installer_input: &Option<String>,
    allow_experimental: &bool,
) -> Result<(), anyhow::Error> {
    let minecraft = minecraft_input.as_deref().unwrap();
    let loader = loader_input.as_deref().unwrap();
    let installer = installer_input.as_deref().unwrap();

    let formatted_url = get_formatted_url(minecraft, loader, installer, allow_experimental)?;

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?;

    let filename = format!("fabric.jar");

    let mut file = File::create(filename)?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(())
}

fn get_formatted_url(
    minecraft: &str,
    loader: &str,
    installer: &str,
    allow_experimental: &bool,
) -> Result<String, anyhow::Error> {
    let formatted_url = format!(
        "{}/loader/{}/{}/{}/server/jar",
        BASE_URL,
        get_specific_version("/game", minecraft, allow_experimental)?.version,
        get_specific_version("/loader", loader, allow_experimental)?.version,
        get_specific_version("/installer", installer, allow_experimental)?.version
    );

    return Ok(formatted_url);
}

fn get_specific_version(
    path: &str,
    version: &str,
    allow_experimental: &bool,
) -> Result<Version, anyhow::Error> {
    let versions: Vec<Version> = ureq::get(&format!("{}{}", BASE_URL, path))
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if version == "latest" {
        return get_latest_version(&versions, allow_experimental);
    }

    let specific_version = versions.iter().filter(|p| &p.version == version).next();

    if specific_version.is_none() {
        let formatted = format!(
            "{} version {} does not exist",
            path.strip_prefix('/').unwrap(),
            version
        );
        return Err(anyhow!(formatted));
    }

    Ok(specific_version.unwrap().clone())
}

fn get_latest_version(
    versions: &Vec<Version>,
    allow_experimental: &bool,
) -> Result<Version, anyhow::Error> {
    let mut latest_version = versions.get(0);
    if !allow_experimental {
        latest_version = versions.iter().filter(|x| x.stable).next();
    }

    if latest_version.is_none() {
        return Err(anyhow!("failed to fetch requested minecraft version"));
    }

    Ok(latest_version.unwrap().clone())
}
