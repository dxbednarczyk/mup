use pap::FAKE_USER_AGENT;
use anyhow::anyhow;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Version {
    game_versions: Vec<String>,
    loaders: Vec<String>,
    version_number: String,
    version_type: String,
    files: Vec<File>,
    dependencies: Vec<Value>,
}

#[derive(Debug, Deserialize)]
struct File {
    hashes: Hashes,
    url: String,
    filename: String,
}

#[derive(Debug, Deserialize)]
struct Hashes {
    sha512: String,
}

#[derive(Debug, Deserialize)]
struct ProjectInfo {
    server_side: String,
    loaders: Vec<String>,
    game_versions: Vec<String>,
    versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub name: String,
}

pub fn add(id: &String, minecraft_input: &Option<String>, project_version: &Option<String>, loader_input: &Option<String>) -> Result<(), anyhow::Error> {
    let formatted_url = format!("{}/project/{id}", super::BASE_URL);

    let resp: ProjectInfo = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    if resp.server_side == "unsupported" {
        return Err(anyhow!("project {id} does not support server side"));
    }

    let project = project_version.as_ref().unwrap();
    if project.as_str() != "latest" && !resp.versions.contains(project) {
        return Err(anyhow!("project version {project} does not exist"));
    }

    let version_info = get_version(&resp, project)?;

    let minecraft = minecraft_input.as_ref().unwrap();
    if minecraft.as_str() != "latest" && !resp.game_versions.contains(minecraft) {
        return Err(anyhow!("project {project} does not support Minecraft version {minecraft}"));
    }

    if !version_info.game_versions.contains(minecraft) {
        return Err(anyhow!("project version {} does not support Minecraft version {minecraft}", version_info.version_number));
    }

    let loader = loader_input.as_ref();
    if loader.is_some() && resp.loaders.len() > 1 {
        return Err(anyhow!("project supports more than one loader, please specify which to target"));
    }

    let loader = loader.unwrap();
    if !resp.loaders.contains(loader) {
        return Err(anyhow!("project does not support {loader} loader"));
    }

    if !version_info.loaders.contains(loader) {
        return Err(anyhow!("project version {} does not support loader {loader}", version_info.version_number));
    }

    // match loader.as_str() {
    //     "fabric" => download_fabric(&resp, minecraft, project)?,
    //     _ => unimplemented!()
    // }

    Ok(())
}

fn get_version(project: &ProjectInfo, wanted_version: &str) -> Result<Version, anyhow::Error> {
    let version = if wanted_version == "latest" {
        project.versions.iter().last().unwrap()
    } else {
        wanted_version
    };
    
    let formatted_url = format!("{}/version/{version}", super::BASE_URL);
    
    let resp: Version = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    Ok(resp)
}