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
    channel: String,
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
    minecraft_input: &String,
    loader_input: &String,
    allow_experimental: &bool,
) -> Result<(), anyhow::Error> {
    let mut minecraft = minecraft_input.clone();
    let mut loader: Build = Build::default();

    if minecraft_input.as_str() == "latest" {
        minecraft = get_latest_version()?;
    }

    if loader_input.as_str() == "latest" {
        loader = get_latest_loader(&minecraft, allow_experimental)?;
    } else {
        loader.build = loader_input.parse()?;
    }

    let formatted_url = format!(
        "{}/versions/{}/builds/{}/downloads/paper-{}-{}.jar",
        BASE_URL, minecraft, loader.build, minecraft, loader.build,
    );

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_reader();

    let filename = format!("paper-{}-{}.jar", minecraft, loader.build);
    let mut file = File::create(filename)?;

    let mut hasher = Sha256::new();

    let mut tee = TeeReader::new(resp, &mut file);
    io::copy(&mut tee, &mut hasher)?;

    let hash = hasher.finalize();

    if format!("{:x}", hash) != loader.downloads.application.sha256 {
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

fn get_latest_loader(
    minecraft_version: &String,
    allow_experimental: &bool,
) -> Result<Build, anyhow::Error> {
    let formatted_url = format!("{}/versions/{}/builds", BASE_URL, minecraft_version);

    let body: Builds = ureq::get(formatted_url.as_str())
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    let mut latest = body.builds.last();

    if !allow_experimental {
        latest = body
            .builds
            .iter()
            .rev()
            .filter(|x| x.channel.as_str() == "default")
            .next();
    }

    if latest.is_none() {
        return Err(anyhow!("could not get latest loader version"));
    }

    Ok(latest.unwrap().clone())
}
