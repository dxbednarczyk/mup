#![allow(clippy::case_sensitive_file_extension_comparisons)]

use std::path::PathBuf;

use anyhow::anyhow;
use pap::{download_with_checksum, FAKE_USER_AGENT};
use serde::Deserialize;
use sha2::Sha512;

use crate::server::lockfile::Lockfile;

#[derive(Clone, Debug, Deserialize)]
pub struct Version {
    game_versions: Vec<String>,
    loaders: Vec<String>,
    pub id: String,
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

pub fn fetch(
    lockfile: &Lockfile,
    id: &String,
    version_input: &Option<String>,
) -> Result<(Version, ProjectInfo, ProjectFile, PathBuf), anyhow::Error> {
    let formatted_url = format!("{}/project/{id}", super::BASE_URL);

    println!("Fetching project info for {id}");
    let project_info: ProjectInfo = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if project_info.server_side == "unsupported" {
        return Err(anyhow!("project {id} does not support server side"));
    }

    if !project_info
        .game_versions
        .contains(lockfile.loader.minecraft_version())
    {
        return Err(anyhow!(
            "project does not support Minecraft version {}",
            lockfile.loader.minecraft_version(),
        ));
    }

    let version = version_input.as_ref().unwrap();
    if version.as_str() != "latest" && !project_info.versions.contains(version) {
        return Err(anyhow!("project version {version} does not exist"));
    }

    if !project_info.loaders.contains(&lockfile.loader.to_string()) {
        return Err(anyhow!("project does not support {}", lockfile.loader));
    }

    let version_info = if version.as_str() == "latest" {
        get_latest_version(
            &project_info,
            lockfile.loader.minecraft_version(),
            &lockfile.loader.to_string(),
        )?
    } else {
        get_version(
            &project_info,
            version,
            lockfile.loader.minecraft_version(),
            &lockfile.loader.to_string(),
        )?
    };

    let project_file: &ProjectFile = version_info
        .files
        .iter()
        .find(|f| f.filename.ends_with(".jar"))
        .unwrap();

    let save_to = match lockfile.loader.to_string().as_str() {
        "paper" => PathBuf::from(&format!("./plugins/{}", project_file.filename)),
        "fabric" | "forge" => PathBuf::from(&format!("./mods/{}", project_file.filename)),
        _ => unreachable!(),
    };

    download_with_checksum::<Sha512>(&project_file.url, &save_to, &project_file.hashes.sha512)?;

    Ok((version_info.clone(), project_info, project_file.clone(), save_to))
}

pub fn add(
    lockfile: &mut Lockfile,
    id: &String,
    version_input: &Option<String>,
) -> Result<(), anyhow::Error> {
    if lockfile.get(id).is_ok() {
        return Err(anyhow!(
            "specified project already has an entry in the lockfile"
        ));
    }

    let (version_info, project_info, file, save_to) = fetch(lockfile, id, version_input)?;

    lockfile.add(&version_info, &project_info, &file, save_to)
}

pub fn remove(
    lockfile: &mut Lockfile,
    slug: &str,
    keep_jarfile: bool,
) -> Result<(), anyhow::Error> {
    if lockfile.get(slug).is_err() {
        return Err(anyhow!("project {slug} does not exist in the lockfile"));
    }

    lockfile.remove(slug, keep_jarfile)?;

    Ok(())
}

fn get_version(
    project: &ProjectInfo,
    version_id: &str,
    minecraft_input: &String,
    loader: &String,
) -> Result<Version, anyhow::Error> {
    let formatted_url = format!("{}/version/{version_id}", super::BASE_URL);

    println!("Fetching version {version_id} of {}", project.slug);
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

    if !resp.loaders.contains(loader) {
        return Err(anyhow!(
            "project version {version_id} does not support loader {loader}",
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
            format!("[\"{minecraft_version}\"]").as_str(),
        );

    if !loader.is_empty() {
        req = req.query("loaders", format!("[\"{loader}\"]").as_str());
    }

    println!("Fetching latest version of {}", project.slug);
    let resp: Vec<Version> = req.call()?.into_json()?;

    let version = resp
        .iter()
        .find(|p| p.game_versions.contains(minecraft_version))
        .ok_or_else(|| anyhow!("could not find a matching version"))?;

    if !version.loaders.contains(loader) {
        return Err(anyhow!(
            "project version ID {} does not support loader {loader}",
            version.id
        ));
    }

    Ok(version.clone())
}
