use pap::FAKE_USER_AGENT;
use serde::Deserialize;
use ureq;

const BASE_URL: &str = "https://api.modrinth.com/v2"; 

#[derive(Debug, Deserialize)]
struct ProjectInfo {
    title: String,
    description: String,
    downloads: i64,
    project_type: String,
    server_side: String,
    license: License,
    id: String,
    slug: String,
    categories: Vec<String>,
    loaders: Vec<String>,
    game_versions: Vec<String>,
    versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub name: String,
}

pub fn info(id: &String) -> Result<(), anyhow::Error> {
    let formatted_url = format!("{BASE_URL}/project/{id}");

    let resp: ProjectInfo = ureq::get(&formatted_url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    println!("{resp:#?}");

    Ok(())
}