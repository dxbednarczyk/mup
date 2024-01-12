use std::{collections::HashMap, fs::File, io};

use anyhow::anyhow;
use serde::Deserialize;
use versions::{Chunk, Versioning};

const PROMOS_URL: &str =
    "https://files.minecraftforge.net/maven/net/minecraftforge/forge/promotions_slim.json";
const BASE_MAVEN_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";

#[derive(Debug, Deserialize)]
struct PromosResponse {
    promos: HashMap<String, String>,
}

pub fn fetch(
    minecraft_input: &String,
    loader_input: &String,
    allow_experimental: &bool,
) -> Result<(), anyhow::Error> {
    let minecraft: Versioning;
    let mut loader = loader_input.clone();

    let promos = get_promos()?;

    if minecraft_input == "latest" {
        let mut versions: Vec<Versioning> = promos
            .keys()
            .filter_map(|p| p.split('-').next())
            .filter_map(|p| Versioning::new(p))
            .collect();

        versions.sort_by(|a, b| Versioning::cmp(&a, &b));
        minecraft = versions.last().unwrap().clone();
    } else {
        minecraft = Versioning::new(minecraft_input).unwrap()
    }

    if loader == "latest" {
        let mut formatted_version = format!("{}-recommended", minecraft);

        let promo = promos.get(&formatted_version);

        if promo.is_none() && !allow_experimental {
            return Err(anyhow!(
                "failed to find a recommended loader (tip: to download the latest loader regardless, pass --allow-experimental)"
            ));
        }

        formatted_version = format!("{}-latest", minecraft);
        loader = promos.get(&formatted_version).unwrap().clone();
    }

    let formatted_url = get_formatted_url(&minecraft, &loader)?;

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?;

    let filename = format!("forge-{}-{}.jar", minecraft.to_string(), loader);

    let mut file = File::create(filename)?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(())
}

fn get_promos() -> Result<HashMap<String, String>, anyhow::Error> {
    let resp: PromosResponse = ureq::get(PROMOS_URL)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    return Ok(resp.promos);
}

fn get_formatted_url(minecraft: &Versioning, loader: &String) -> Result<String, anyhow::Error> {
    let mut formatted_url = String::from(BASE_MAVEN_URL);

    formatted_url.push_str(format!("/{}-{}-", minecraft.to_string(), loader).as_str());

    handle_caveats(minecraft, &mut formatted_url)?;

    formatted_url.push_str(format!("/forge-{}-{}-", minecraft.to_string(), loader).as_str());

    handle_caveats(minecraft, &mut formatted_url)?;

    formatted_url.push_str("-installer.jar");

    Ok(formatted_url)
}

fn handle_caveats(minecraft: &Versioning, formatted_url: &mut String) -> Result<(), anyhow::Error> {
    match minecraft {
        Versioning::Ideal(s) => {
            if s.minor <= 5 && s.patch < 2 {
                return Err(anyhow!(
                    "forge does not provide loader jarfiles before Minecraft 1.5.2"
                ));
            }

            if s.minor == 8 || s.minor == 9 {
                formatted_url.push_str(format!("{}.{}.{}", s.major, s.minor, s.patch).as_str());
            }
        }
        Versioning::General(v) => {
            let release = &v.chunks.0;

            let major = release
                .get(0)
                .unwrap_or(&Chunk::Numeric(0))
                .single_digit()
                .unwrap();
            let minor = release
                .get(1)
                .unwrap_or(&Chunk::Numeric(0))
                .single_digit()
                .unwrap();
            let patch = release
                .get(2)
                .unwrap_or(&Chunk::Numeric(0))
                .single_digit()
                .unwrap();

            if minor <= 5 && patch < 2 {
                return Err(anyhow!(
                    "forge does not provide installer jarfiles before Minecraft 1.5.2"
                ));
            }

            if minor == 8 || minor == 9 {
                formatted_url.push_str(format!("{}.{}.{}", major, minor, patch).as_str());
            }
        }
        Versioning::Complex(_) => return Err(anyhow!("complex version numbers are unsupported")),
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_formatted_url() -> Result<(), anyhow::Error> {
        // Test a semver-compatible version
        let mut minecraft = Versioning::new("1.9.4").unwrap();
        let mut loader = String::from("12.17.0.2317");

        let mut formatted_url = get_formatted_url(&minecraft, &loader)?;
        let mut expected = "https://maven.minecraftforge.net/net/minecraftforge/forge/1.9.4-12.17.0.2317-1.9.4/forge-1.9.4-12.17.0.2317-1.9.4-installer.jar";
       
        assert_eq!(expected, formatted_url);

        // Test a non semver-compatible version
        minecraft = Versioning::new("1.9").unwrap();
        loader = String::from("12.16.1.1938");

        formatted_url = get_formatted_url(&minecraft, &loader)?;
        expected = "https://maven.minecraftforge.net/net/minecraftforge/forge/1.9-12.16.1.1938-1.9.0/forge-1.9-12.16.1.1938-1.9.0-installer.jar";

        assert_eq!(expected, formatted_url);

        // Test a complex version
        minecraft = Versioning::new("1.7.10_pre4").unwrap();
        let expected: Result<(), anyhow::Error> = Err(anyhow!("complex version numbers are unsupported"));
        let result = get_formatted_url(&minecraft, &loader);

        assert_eq!(expected.err().unwrap().to_string(), result.err().unwrap().to_string());

        // Test a version before Forge offered installer jarfiles
        minecraft = Versioning::new("1.1").unwrap();
        let expected: Result<(), anyhow::Error> = Err(anyhow!("forge does not provide installer jarfiles before Minecraft 1.5.2"));
        let result = get_formatted_url(&minecraft, &loader);

        assert_eq!(expected.err().unwrap().to_string(), result.err().unwrap().to_string());

        Ok(())
    }

    #[test]
    fn fetch_rejects_latest_without_experimental_flag() {
        let minecraft = String::from("latest");
        let loader = String::from("latest");
        let allow_experimental = false;

        let expected: Result<(), anyhow::Error> = Err(anyhow!("failed to find a recommended loader (tip: to download the latest loader regardless, pass --allow-experimental)"));
        let result = fetch(&minecraft, &loader, &allow_experimental);

        assert_eq!(expected.err().unwrap().to_string(), result.err().unwrap().to_string());
    }
}