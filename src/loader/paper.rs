use std::{fs::File, io};

use anyhow::anyhow;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tee::TeeReader;

const BASE_URL: &str = "https://api.papermc.io/v2/projects/paper";

#[derive(Clone, Debug, Deserialize)]
struct Versions {
    versions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct Builds {
    builds: Vec<Build>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct Build {
    build: u16,
    downloads: Downloads,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct Downloads {
    application: Application,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct Application {
    sha256: String,
}

pub fn fetch(
    minecraft_input: &Option<String>,
    loader_input: &Option<String>,
) -> Result<(), anyhow::Error> {
    let mut minecraft = minecraft_input.as_deref().unwrap();
    let mut build: Build = Build::default();

    let latest: String;
    if minecraft == "latest" {
        latest = get_latest_version()?;
        minecraft = latest.as_str();
    }

    let loader = loader_input.as_deref().unwrap();
    if loader == "latest" {
        build = get_latest_loader(minecraft)?;
    } else {
        build.build = loader.parse()?;
    }

    let formatted_url = format!(
        "{BASE_URL}/versions/{minecraft}/builds/{}/downloads/paper-{minecraft}-{}.jar",
        build.build, build.build,
    );

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_reader();

    let filename = format!("paper-{minecraft}-{}.jar", build.build);
    let mut file = File::create(filename)?;

    let mut hasher = Sha256::new();

    let mut tee = TeeReader::new(resp, &mut file);
    io::copy(&mut tee, &mut hasher)?;

    let hash = hasher.finalize();

    if format!("{hash:x}") != build.downloads.application.sha256 {
        return Err(anyhow!("hashes do not match"));
    }

    Ok(())
}

fn get_latest_version() -> Result<String, anyhow::Error> {
    let body: Versions = ureq::get(BASE_URL)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    let latest = body.versions.last();
    if latest.is_none() {
        return Err(anyhow!("could not get latest minecraft version"));
    }

    Ok(latest.unwrap().clone())
}

fn get_latest_loader(minecraft_version: &str) -> Result<Build, anyhow::Error> {
    let formatted_url = format!("{BASE_URL}/versions/{minecraft_version}/builds");

    let body: Builds = ureq::get(formatted_url.as_str())
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    let latest = body.builds.last();

    if latest.is_none() {
        return Err(anyhow!("could not get latest loader version"));
    }

    Ok(latest.unwrap().clone())
}
