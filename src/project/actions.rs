use std::{
    fs::{self, File},
    io,
};

use anyhow::anyhow;
use pap::FAKE_USER_AGENT;
use serde::Deserialize;
use sha2::{Digest, Sha512};
use versions::Versioning;

use super::lockfile;

#[derive(Debug, Deserialize)]
pub struct Version {
    game_versions: Vec<String>,
    loaders: Vec<String>,
    pub version_number: String,
    //version_type: String,
    files: Vec<ProjectFile>,
    //dependencies: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct ProjectFile {
    hashes: Hashes,
    url: String,
    pub filename: String,
}

#[derive(Debug, Deserialize)]
struct Hashes {
    sha512: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub slug: String,
    server_side: String,
    loaders: Vec<String>,
    game_versions: Vec<String>,
    versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub name: String,
}

pub fn add(
    id: &String,
    minecraft_input: &String,
    project_version: &Option<String>,
    loader_input: &Option<String>,
) -> Result<(), anyhow::Error> {
    let formatted_url = format!("{}/project/{id}", super::BASE_URL);

    let project_info: ProjectInfo = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if project_info.server_side == "unsupported" {
        return Err(anyhow!("project {id} does not support server side"));
    }

    let mut loader = loader_input.as_ref();
    if loader.is_none() {
        if project_info.loaders.len() > 1 {
            return Err(anyhow!(
                "project supports more than one loader, please specify which to target"
            ));
        }

        loader = Some(project_info.loaders.first().unwrap())
    }

    if minecraft_input.as_str() != "latest" && !project_info.game_versions.contains(minecraft_input)
    {
        return Err(anyhow!(
            "project does not support Minecraft version {minecraft_input}"
        ));
    }

    let project = project_version.as_ref().unwrap();
    if project.as_str() != "latest" && !project_info.versions.contains(project) {
        return Err(anyhow!("project version {project} does not exist"));
    }

    let loader = loader.unwrap();
    if !project_info.loaders.contains(loader) {
        return Err(anyhow!("project does not support {loader} loader"));
    }

    let version_info = get_version(&project_info, minecraft_input, project)?;

    if !version_info.loaders.contains(loader) {
        return Err(anyhow!(
            "project version {} does not support loader {loader}",
            version_info.version_number
        ));
    }

    let file = version_info
        .files
        .iter()
        .find(|f| f.filename.ends_with(".jar"))
        .unwrap();

    let resp = ureq::get(&file.url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_reader();

    let mut output = File::create(&file.filename)?;

    let mut hasher = Sha512::new();

    let mut tee = tee::tee(resp, &mut output);
    io::copy(&mut tee, &mut hasher)?;

    let hash = hasher.finalize();

    if format!("{hash:x}") != file.hashes.sha512 {
        fs::remove_file(&file.filename)?;

        return Err(anyhow!("hashes do not match"));
    }

    lockfile::add(&version_info, &project_info, file)?;

    Ok(())
}

fn get_version(
    project: &ProjectInfo,
    minecraft_version: &String,
    wanted_version: &String,
) -> Result<Version, anyhow::Error> {
    if wanted_version == "latest" {
        return get_latest_version(project, minecraft_version);
    }

    let formatted_url = format!("{}/version/{wanted_version}", super::BASE_URL);

    let resp: Version = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if !resp.game_versions.contains(minecraft_version) {
        return Err(anyhow!(
            "project version {} does not support Minecraft version {minecraft_version}",
            resp.version_number
        ));
    }

    Ok(resp)
}

fn get_latest_version(
    project: &ProjectInfo,
    minecraft_version: &String,
) -> Result<Version, anyhow::Error> {
    let parsed = Versioning::new(minecraft_version).unwrap();

    let mut found_version: Option<Version> = None;
    for version in project.versions.iter().rev() {
        let formatted_url = format!("{}/version/{version}", super::BASE_URL);

        let resp: Version = ureq::get(&formatted_url)
            .set("User-Agent", FAKE_USER_AGENT)
            .call()?
            .into_json()?;

        if resp.game_versions.contains(minecraft_version) {
            found_version = Some(resp);
            break;
        }

        if resp
            .game_versions
            .iter()
            .all(|v| Versioning::new(v).unwrap() < parsed)
        {
            return Err(anyhow!(
                "failed to find a version compatible with Minecraft version {parsed}"
            ));
        }
    }

    if found_version.is_none() {
        return Err(anyhow!("could not find a compatible version"));
    }

    Ok(found_version.unwrap())
}
