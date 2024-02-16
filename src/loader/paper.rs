use std::path::PathBuf;

use super::Loader;
use anyhow::anyhow;
use pap::download_with_checksum;
use serde::Deserialize;
use serde_json::Value;
use sha2::Sha256;

const BASE_URL: &str = "https://api.papermc.io/v2/projects/paper";

#[derive(Clone, Debug, Deserialize)]
struct Builds {
    builds: Vec<Build>,
}

#[derive(Clone, Debug, Default, Deserialize)]
struct Build {
    build: usize,
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

pub fn fetch(minecraft_version: &str, build: &str) -> Result<Loader, anyhow::Error> {
    let minecraft = if minecraft_version == "latest" {
        get_latest_version()?
    } else {
        minecraft_version.to_string()
    };

    let build = get_build(&minecraft, build)?;

    let formatted_url = format!(
        "{BASE_URL}/versions/{minecraft}/builds/{}/downloads/paper-{minecraft}-{}.jar",
        build.build, build.build,
    );

    let filename = format!("paper-{minecraft}-{}.jar", build.build);

    download_with_checksum::<Sha256>(
        &formatted_url,
        &PathBuf::from(filename),
        &build.downloads.application.sha256,
    )?;

    Ok(Loader::Paper {
        minecraft_version: minecraft,
        build: build.build.to_string(),
    })
}

fn get_latest_version() -> Result<String, anyhow::Error> {
    println!("Fetching latest Minecraft version");
    let body: Value = ureq::get(BASE_URL)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    let latest = body["versions"]
        .as_array()
        .unwrap()
        .last()
        .ok_or_else(|| anyhow!("could not get latest minecraft version"))?
        .to_string();

    Ok(latest)
}

fn get_build(minecraft_version: &str, build: &str) -> Result<Build, anyhow::Error> {
    let formatted_url = format!("{BASE_URL}/versions/{minecraft_version}/builds");

    println!("Fetching build {build} for {minecraft_version}");
    let body: Builds = ureq::get(formatted_url.as_str())
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if build == "latest" {
        return Ok(body.builds.first().unwrap().clone());
    }

    let latest = body
        .builds
        .iter()
        .find(|p| p.build == build.parse::<usize>().unwrap())
        .ok_or_else(|| anyhow!("could not get specific loader version"))?;

    Ok(latest.clone())
}
