use std::{collections::HashMap, fs::File, io};

use anyhow::anyhow;
use once_cell::sync::Lazy;
use serde::Deserialize;
use versions::{Chunk, Versioning};

const PROMOS_URL: &str =
    "https://files.minecraftforge.net/maven/net/minecraftforge/forge/promotions_slim.json";
const BASE_MAVEN_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";

static CUTOFF: Lazy<Versioning> = Lazy::new(|| Versioning::new("1.5.2").unwrap());

#[derive(Debug, Deserialize)]
struct PromosResponse {
    promos: HashMap<String, String>,
}

pub fn fetch(
    minecraft_input: &Option<String>,
    installer_input: &Option<String>,
    force_latest: &bool,
) -> Result<(), anyhow::Error> {
    let mut installer = installer_input.as_deref().unwrap();

    let promos = get_promos()?;

    let minecraft_input = minecraft_input.as_deref().unwrap();
    let minecraft = if minecraft_input == "latest" {
        let mut versions: Vec<Versioning> = promos
            .keys()
            .filter_map(|p| p.split('-').next())
            .filter_map(Versioning::new)
            .collect();

        versions.sort_by(Versioning::cmp);
        versions.last().unwrap().clone()
    } else {
        Versioning::new(minecraft_input).unwrap()
    };

    if installer == "latest" {
        let mut installer_type = "recommended";
        if *force_latest {
            installer_type = "latest";
        }

        let formatted_version = format!("{minecraft}-{installer_type}");
        let promo = promos.get(&formatted_version);

        if promo.is_none() {
            return Err(anyhow!(
                "failed to find a recommended installer (tip: to download the latest installer regardless, pass --force-latest)"
            ));
        }

        installer = promo.unwrap();
    }

    let formatted_url = get_formatted_url(&minecraft, installer)?;

    let resp = ureq::get(&formatted_url)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?;

    let filename = format!("forge-{minecraft}-{installer}.jar");

    let mut file = File::create(filename)?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    Ok(())
}

fn get_promos() -> Result<HashMap<String, String>, anyhow::Error> {
    let resp: PromosResponse = ureq::get(PROMOS_URL)
        .set("User-Agent", super::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    Ok(resp.promos)
}

fn get_formatted_url(minecraft: &Versioning, loader: &str) -> Result<String, anyhow::Error> {
    let prefix = get_version_tag(minecraft, loader)?;
    let suffix = get_version_tag(minecraft, loader)?;

    let formatted_url = format!("{BASE_MAVEN_URL}/{prefix}/forge-{suffix}-installer.jar");

    Ok(formatted_url)
}

// Did I mention already how much I hate the Forge versioning scheme?
// If not, reading this function equates to an essay on it.
fn get_version_tag(minecraft: &Versioning, loader: &str) -> Result<String, anyhow::Error> {
    match minecraft {
        Versioning::Ideal(s) => {
            if minecraft < &CUTOFF {
                return Err(anyhow!(
                    "forge does not provide loader jarfiles before Minecraft 1.5.2"
                ));
            }
            
            let stringified = s.to_string();

            let wonky_range = 7..10;
            if !wonky_range.contains(&s.minor) {
                return Ok(format!("{stringified}-{loader}"));
            }

            if s.minor == 7 && s.patch == 2 {
                return Ok(format!("1.7.2-{loader}-mc172"))
            }

            Ok(format!("{stringified}-{loader}-{stringified}"))
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

            if minor <= 5 {
                return Err(anyhow!(
                    "invalid minecraft version provided"
                ));
            }

            let wonky_range = 9..11;
            if wonky_range.contains(&minor) {
                return Ok(format!("{major}.{minor}-{loader}-{major}.{minor}.0"));
            }
                
            Ok(format!("{major}.{minor}-{loader}"))
        }
        Versioning::Complex(_) => return Err(anyhow!("complex version numbers are unsupported")),
    }
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
        let expected: Result<(), anyhow::Error> =
            Err(anyhow!("complex version numbers are unsupported"));
        let result = get_formatted_url(&minecraft, &loader);

        assert_eq!(
            expected.err().unwrap().to_string(),
            result.err().unwrap().to_string()
        );

        // Test a generally invalid version
        minecraft = Versioning::new("1.1").unwrap();
        let expected: Result<(), anyhow::Error> = Err(anyhow!(
            "invalid minecraft version provided"
        ));
        let result = get_formatted_url(&minecraft, &loader);

        assert_eq!(
            expected.err().unwrap().to_string(),
            result.err().unwrap().to_string()
        );

        // Test a version before 1.5.2
        minecraft = Versioning::new("1.2.5").unwrap();
        let expected: Result<(), anyhow::Error> = Err(anyhow!(
            "forge does not provide loader jarfiles before Minecraft 1.5.2"
        ));
        let result = get_formatted_url(&minecraft, &loader);

        assert_eq!(
            expected.err().unwrap().to_string(),
            result.err().unwrap().to_string()
        );

        Ok(())
    }
}
