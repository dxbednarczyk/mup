use std::{collections::HashMap, fs::File, io, sync::LazyLock};

use super::Loader;
use anyhow::anyhow;
use serde::Deserialize;
use versions::Versioning;

const PROMOS_URL: &str =
    "https://files.minecraftforge.net/maven/net/minecraftforge/forge/promotions_slim.json";
const BASE_MAVEN_URL: &str = "https://maven.minecraftforge.net/net/minecraftforge/forge";

// Forge does not provide installer jarfiles before Minecraft version 1.5.2
static MINECRAFT_CUTOFF: LazyLock<Versioning> = LazyLock::new(|| Versioning::new("1.5.2").unwrap());

// The cutoff in 1.9 builds after which versions are formatted as 1.X-{installer}-1.X.0
static INSTALLER_CUTOFF_TRIPLE: LazyLock<Versioning> =
    LazyLock::new(|| Versioning::new("12.16.1.1938").unwrap());

// The cutoff in 1.9 builds before which versions are formatted as 1.9-{installer}
static INSTALLER_CUTOFF_DOUBLE: LazyLock<Versioning> =
    LazyLock::new(|| Versioning::new("12.16.0.1885").unwrap());

#[derive(Deserialize)]
struct PromosResponse {
    promos: HashMap<String, String>,
}

pub fn fetch(minecraft_version: &str, installer_version: &str) -> Result<Loader, anyhow::Error> {
    println!("fetching promos");
    let promos = ureq::get(PROMOS_URL)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json::<PromosResponse>()?
        .promos;

    let minecraft = if minecraft_version == "latest" {
        let mut versions: Vec<Versioning> = promos
            .keys()
            .filter_map(|p| p.split('-').next())
            .filter_map(Versioning::new)
            .collect();

        versions.sort_by(Versioning::cmp);

        versions.last().unwrap().clone()
    } else {
        Versioning::new(minecraft_version).unwrap()
    };

    let promo = promos.get(&format!("{minecraft}-{installer_version}"));


    let installer = if installer_version == "latest" {
       promo.ok_or_else(|| anyhow!("invalid or unsupported minecraft version"))?
    } else {
        installer_version
    };

    let version_tag = get_version_tag(&minecraft, installer)?;

    let formatted_url = format!("{BASE_MAVEN_URL}/{version_tag}/forge-{version_tag}-installer.jar");

    println!("Downloading installer jarfile");
    let resp = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?;

    let filename = format!("forge-{minecraft}-{installer}.jar");

    let mut file = File::create(filename)?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    eprintln!("This is an installer, not a server loader! Please run it and install the server before proceeding.");

    Ok(Loader::Forge {
        minecraft_version: minecraft.to_string(),
        installer_version: installer.to_string(),
    })
}

fn get_version_tag(minecraft: &Versioning, installer: &str) -> Result<String, anyhow::Error> {
    if minecraft < &MINECRAFT_CUTOFF {
        return Err(anyhow!(
            "forge does not provide installer jarfiles before Minecraft 1.5.2"
        ));
    }

    // Lots of edge cases here
    match minecraft {
        Versioning::Ideal(s) => {
            if !(7..10).contains(&s.minor) {
                return Ok(format!("{s}-{installer}"));
            }

            if s.minor == 7 && s.patch == 2 {
                return Ok(format!("1.7.2-{installer}-mc172"));
            }

            Ok(format!("{s}-{installer}-{s}"))
        }
        Versioning::General(v) => {
            let minor: u32 = v.chunks.0[1].to_string().parse()?;

            let installer = Versioning::new(installer).unwrap();

            if (9..11).contains(&minor) && &installer >= &INSTALLER_CUTOFF_TRIPLE {
                return Ok(format!("{v}-{installer}-{v}.0"));
            }

            if minor == 9 && &installer <= &INSTALLER_CUTOFF_DOUBLE {
                return Ok(format!("{v}-{installer}-{v}"));
            }

            Ok(format!("{v}-{installer}"))
        }
        // This is currently the only release that ends up down here...
        Versioning::Complex(_) => Ok(format!("1.7.10_pre4-{installer}-prerelease")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideal_version() -> Result<(), anyhow::Error> {
        let minecraft = Versioning::new("1.9.4").unwrap();
        let installer = "12.17.0.2317";
        let expected = "https://maven.minecraftforge.net/net/minecraftforge/forge/1.9.4-12.17.0.2317-1.9.4/forge-1.9.4-12.17.0.2317-1.9.4-installer.jar";

        let version_tag = get_version_tag(&minecraft, installer)?;

        let formatted_url =
            format!("{BASE_MAVEN_URL}/{version_tag}/forge-{version_tag}-installer.jar");

        assert_eq!(expected, formatted_url);

        Ok(())
    }

    #[test]
    fn test_general_version() -> Result<(), anyhow::Error> {
        let minecraft = Versioning::new("1.9").unwrap();
        let installer = "12.16.1.1938";
        let expected = "https://maven.minecraftforge.net/net/minecraftforge/forge/1.9-12.16.1.1938-1.9.0/forge-1.9-12.16.1.1938-1.9.0-installer.jar";

        let version_tag = get_version_tag(&minecraft, installer)?;

        let formatted_url =
            format!("{BASE_MAVEN_URL}/{version_tag}/forge-{version_tag}-installer.jar");

        assert_eq!(expected, formatted_url);

        Ok(())
    }

    #[test]
    fn test_cutoff() -> Result<(), anyhow::Error> {
        let minecraft = Versioning::new("1.2.5").unwrap();
        let installer = "who cares";
        let expected: Result<(), anyhow::Error> = Err(anyhow!(
            "forge does not provide installer jarfiles before Minecraft 1.5.2"
        ));

        let resp = get_version_tag(&minecraft, installer)
            .err()
            .unwrap()
            .to_string();

        assert_eq!(expected.err().unwrap().to_string(), resp);

        Ok(())
    }
}
