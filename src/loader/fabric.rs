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
    minecraft_input: &String,
    loader_input: &String,
    allow_experimental: &bool,
) -> Result<(), anyhow::Error> {
    let mut minecraft = minecraft_input.clone();
    let mut loader = loader_input.clone();

    let formatted_url = get_formatted_url(&mut minecraft, &mut loader, allow_experimental)?;

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?;

    let filename = format!("fabric-{}-{}.jar", minecraft, loader);

    let mut file = File::create(filename)?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(())
}

fn get_formatted_url(minecraft: &mut String, loader: &mut String, allow_experimental: &bool) -> Result<String, anyhow::Error> {
    let installer = get_latest_version("/installer", allow_experimental)?.version;

    if minecraft.as_str() == "latest" {
        *minecraft = get_latest_version("/game", allow_experimental)?.version;
    }

    if loader.as_str() == "latest" {
        *loader = get_latest_version("/loader", allow_experimental)?.version;
    }

    let formatted_url = format!(
        "{}/loader/{}/{}/{}/server/jar",
        BASE_URL, minecraft, loader, installer
    );

    return Ok(formatted_url)
}
 
fn get_latest_version(path: &str, allow_experimental: &bool) -> Result<Version, anyhow::Error> {
    let body: Vec<Version> = ureq::get(&format!("{}{}", BASE_URL, path))
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    let mut latest_version = body.get(0);
    if !allow_experimental {
        latest_version = body.iter().filter(|x| x.stable).next();
    }

    if latest_version.is_none() {
        return Err(anyhow!("failed to fetch requested minecraft version"));
    }

    Ok(latest_version.unwrap().clone())
}