use std::{
    fs::{self, File},
    io,
};

use anyhow::anyhow;
use pap::FAKE_USER_AGENT;
use serde::Deserialize;
use sha2::{Digest, Sha512};

use super::lockfile::Lockfile;

#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    game_versions: Vec<String>,
    loaders: Vec<String>,
    pub version_number: String,
    //version_type: String,
    files: Vec<ProjectFile>,
    //dependencies: Vec<Value>,
    project_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProjectFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Hashes {
    pub sha512: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub slug: String,
    server_side: String,
    id: String,
    loaders: Vec<String>,
    game_versions: Vec<String>,
    versions: Vec<String>,
}

pub fn add(
    id: &String,
    minecraft_input: &String,
    project_version: &Option<String>,
    loader_input: &Option<String>,
) -> Result<(), anyhow::Error> {
    let mut lf = Lockfile::new()?;

    if lf.get(id).is_ok() {
        return Err(anyhow!("project {id} already exists in the lockfile"));
    }

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

    let project_version = project_version.as_ref().unwrap();
    if project_version.as_str() != "latest" && !project_info.versions.contains(project_version) {
        return Err(anyhow!("project version {project_version} does not exist"));
    }

    let loader = loader.unwrap();
    if !project_info.loaders.contains(loader) {
        return Err(anyhow!("project does not support {loader} loader"));
    }

    let version_info = if project_version.as_str() == "latest" {
        get_latest_version(&project_info, minecraft_input, loader)?
    } else {
        get_version(&project_info, minecraft_input, project_version)?
    };

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

    lf.add(&version_info, &project_info, file)?;

    Ok(())
}

fn get_version(
    project: &ProjectInfo,
    minecraft_input: &String,
    version_id: &String,
) -> Result<Version, anyhow::Error> {
    let formatted_url = format!("{}/version/{version_id}", super::BASE_URL);

    let resp: Version = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if project.id != resp.project_id {
        return Err(anyhow!(
            "version id {version_id} is not a part of project {}",
            project.slug
        ));
    }

    if !resp.game_versions.contains(minecraft_input) {
        return Err(anyhow!(
            "version id {version_id} does not support minecraft version {minecraft_input}"
        ));
    }

    Ok(resp)
}

fn get_latest_version(
    project: &ProjectInfo,
    minecraft_version: &String,
    loader: &String,
) -> Result<Version, anyhow::Error> {
    let formatted_url = format!("{}/project/{}/version", super::BASE_URL, project.slug);

    let mut req = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .query(
            "game_versions",
            format!("[\"{}\"]", minecraft_version).as_str(),
        );

    if !loader.is_empty() {
        req = req.query("loaders", format!("[\"{}\"]", loader).as_str());
    }

    let resp: Vec<Version> = req.call()?.into_json()?;

    let version = resp
        .iter()
        .find(|p| p.game_versions.contains(minecraft_version))
        .ok_or(anyhow!("could not find a matching version"))?;

    Ok(version.clone())
}
