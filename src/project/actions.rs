#![allow(clippy::case_sensitive_file_extension_comparisons)]

use std::path::PathBuf;

use anyhow::anyhow;
use log::{info, warn};
use pap::{download_with_checksum, FAKE_USER_AGENT};
use serde::{Deserialize, Serialize};
use sha2::Sha512;

use crate::server::lockfile::Lockfile;

pub const BASE_URL: &str = "https://api.modrinth.com/v2";

#[derive(Clone, Deserialize)]
pub struct Version {
    game_versions: Vec<String>,
    loaders: Vec<String>,
    pub id: String,
    pub project_id: String,
    files: Vec<ProjectFile>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dependency {
    pub project_id: String,
    pub dependency_type: String,
}

#[derive(Clone, Deserialize)]
pub struct ProjectFile {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
}

#[derive(Clone, Deserialize)]
pub struct Hashes {
    pub sha512: String,
}

#[derive(Deserialize)]
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
    id: &str,
    version_input: Option<&str>,
) -> Result<(Version, ProjectInfo, ProjectFile, PathBuf), anyhow::Error> {
    let formatted_url = format!("{BASE_URL}/project/{id}");

    info!("Fetching project info for {id}");

    let project_info: ProjectInfo = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if project_info.server_side == "unsupported" {
        warn!("project {id} does not support server side, skipping");
        return Err(anyhow!("client side"));
    }

    if !project_info.loaders.contains(&lockfile.loader.name) {
        return Err(anyhow!(
            "project {id} does not support {}",
            lockfile.loader.name
        ));
    }

    if !project_info
        .game_versions
        .contains(&lockfile.loader.minecraft_version)
    {
        return Err(anyhow!(
            "project does not support Minecraft version {}",
            lockfile.loader.minecraft_version
        ));
    }

    let version = version_input.unwrap();
    if version != "latest" && !project_info.versions.contains(&version.to_string()) {
        return Err(anyhow!("project version {version} does not exist"));
    }

    if !project_info
        .game_versions
        .contains(&lockfile.loader.minecraft_version)
    {
        return Err(anyhow!(
            "project does not support minecraft version {}",
            lockfile.loader.minecraft_version
        ));
    }

    let version_info = if version == "latest" {
        get_latest_version(
            &project_info.slug,
            &lockfile.loader.minecraft_version,
            &lockfile.loader.name,
        )?
    } else {
        get_version(
            &project_info,
            version,
            &lockfile.loader.minecraft_version,
            &lockfile.loader.name,
        )?
    };

    let (project_file, save_to) = save(&lockfile.loader.project_path(), &version_info)?;

    Ok((version_info, project_info, project_file, save_to))
}

pub fn add(
    lockfile: &mut Lockfile,
    id: &str,
    version_input: Option<&str>,
    optional_deps: bool,
    no_deps: bool,
) -> Result<(), anyhow::Error> {
    if lockfile.get(id).is_ok() || lockfile.projects.iter().any(|p| p.project_id == id) {
        warn!("project id {id} already has an entry in the lockfile, skipping");
        return Ok(());
    }

    let (mut version_info, project_info, file, save_to) = match fetch(lockfile, id, version_input) {
        Ok(r) => r,
        Err(e) => {
            if e.to_string() == "client_side" {
                return Ok(());
            }

            return Err(e);
        }
    };

    if no_deps {
        version_info.dependencies.clear();
    }

    if !optional_deps {
        version_info.dependencies = version_info
            .dependencies
            .iter()
            .filter_map(|d| {
                if d.dependency_type.as_str() == "required" {
                    Some(d.to_owned())
                } else {
                    None
                }
            })
            .collect();
    }

    lockfile.add(&version_info, &project_info.slug, &file, save_to)?;

    for dep in version_info.dependencies {
        add(lockfile, &dep.project_id, Some("latest"), false, false)?;
    }

    Ok(())
}

fn save(project_path: &str, version: &Version) -> Result<(ProjectFile, PathBuf), anyhow::Error> {
    let project_file: &ProjectFile = version
        .files
        .iter()
        .find(|f| f.filename.ends_with(".jar"))
        .unwrap();

    let save_to = PathBuf::from(&format!("{}{}", project_path, project_file.filename));

    download_with_checksum::<Sha512>(&project_file.url, &save_to, &project_file.hashes.sha512)?;

    Ok((project_file.clone(), save_to))
}

fn get_version(
    project: &ProjectInfo,
    version_id: &str,
    minecraft_input: &String,
    loader: &String,
) -> Result<Version, anyhow::Error> {
    let formatted_url = format!("{BASE_URL}/version/{version_id}");

    info!("fetching version {version_id} of {}", project.slug);

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
    slug: &str,
    minecraft_version: &String,
    loader: &String,
) -> Result<Version, anyhow::Error> {
    let formatted_url = format!("{BASE_URL}/project/{slug}/version");

    let mut req = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .query(
            "game_versions",
            format!("[\"{minecraft_version}\"]").as_str(),
        );

    if !loader.is_empty() {
        req = req.query("loaders", format!("[\"{loader}\"]").as_str());
    }

    info!("fetching latest version of {slug}");

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
