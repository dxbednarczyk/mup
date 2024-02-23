use std::{fs::File, io, sync::LazyLock};

use anyhow::anyhow;
use serde::Deserialize;
use versions::Versioning;

static MINECRAFT_CUTOFF: LazyLock<Versioning> =
    LazyLock::new(|| Versioning::new("1.20.1").unwrap());

const BASE_API_URL: &str = "https://maven.neoforged.net/api/maven/latest/version/releases";
const BASE_DOWNLOAD_URL: &str = "https://maven.neoforged.net/releases";

#[derive(Deserialize)]
struct Installer {
    version: String,
}

// see https://github.com/neoforged/websites/blob/main/assets/js/neoforge.js
pub fn fetch(minecraft_version: &str) -> Result<(), anyhow::Error> {
    if minecraft_version == "latest" {
        return Err(anyhow!(
            "for neoforge, you must specify a minecraft version to target"
        ));
    }

    let parsed_version = Versioning::new(minecraft_version).unwrap();

    if parsed_version < *MINECRAFT_CUTOFF {
        return Err(anyhow!(
            "neoforge does not support Minecraft versions before 1.20.1"
        ));
    }

    let gav = if parsed_version == *MINECRAFT_CUTOFF {
        "/net/neoforged/forge"
    } else {
        "/net/neoforged/neoforge"
    };

    let formatted_url = if parsed_version == *MINECRAFT_CUTOFF {
        format!("{BASE_API_URL}{gav}?filter=1.20.1")
    } else {
        format!("{BASE_API_URL}{gav}")
    };

    println!("Fetching latest installer version for Minecraft {minecraft_version}");
    let installer: Installer = ureq::get(&formatted_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?
        .into_json()?;

    let installer_url = format!(
        "{BASE_DOWNLOAD_URL}{gav}/{}/{}-{}-installer.jar",
        installer.version,
        gav.rsplit_once('/').unwrap().1,
        installer.version
    );

    println!("Downloading installer jarfile");
    let resp = ureq::get(&installer_url)
        .set("User-Agent", pap::FAKE_USER_AGENT)
        .call()?;

    let filename = format!("neoforge-{minecraft_version}-{}.jar", installer.version);

    let mut file = File::create(filename)?;
    io::copy(&mut resp.into_reader(), &mut file)?;

    eprintln!("This is an installer, not a server loader! Please run it and install the server before proceeding.");

    Ok(())
}
