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
    allow_experimental: bool,
) -> Result<(), anyhow::Error> {
    let minecraft = minecraft_input.as_deref().unwrap();
    let loader = loader_input.as_deref().unwrap();
    let installer = installer_input.as_deref().unwrap();

    let formatted_url = get_formatted_url(minecraft, loader, installer, allow_experimental)?;

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?;

    let mut file = File::create("fabric.jar")?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(())
}

fn get_formatted_url(
    minecraft: &str,
    loader: &str,
    installer: &str,
    allow_experimental: bool,
) -> Result<String, anyhow::Error> {
    let formatted_url = format!(
        "{BASE_URL}/loader/{}/{}/{}/server/jar",
        get_specific_version("/game", minecraft, allow_experimental)?.version,
        get_specific_version("/loader", loader, allow_experimental)?.version,
        get_specific_version("/installer", installer, allow_experimental)?.version
    );

    Ok(formatted_url)
}

fn get_specific_version(
    path: &str,
    version: &str,
    allow_experimental: bool,
) -> Result<Version, anyhow::Error> {
    let versions: Vec<Version> = ureq::get(&format!("{BASE_URL}{path}"))
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if version == "latest" {
        return get_latest_version(&versions, allow_experimental);
    }

    let specific_version = versions.iter().find(|p| p.version == version);

    if specific_version.is_none() {
        let formatted = format!(
            "{} version {version} does not exist",
            path.strip_prefix('/').unwrap(),
        );
        return Err(anyhow!(formatted));
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
