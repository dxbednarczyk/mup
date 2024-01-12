use clap::ValueEnum;

mod fabric;
mod paper;

const FAKE_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.";

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Loader {
    Fabric,
    Paper,
}

pub fn fetch(
    loader: &Loader,
    minecraft_version: &String,
    loader_version: &String,
    allow_experimental: &bool,
) -> Result<(), anyhow::Error> {
    match loader {
        &Loader::Fabric => fabric::fetch(minecraft_version, loader_version, allow_experimental)?,
        &Loader::Paper => paper::fetch(minecraft_version, loader_version, allow_experimental)?,
    }

    Ok(())
}
