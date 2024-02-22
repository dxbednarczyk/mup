use std::{fs::File, io::Write, path::PathBuf};

use anyhow::anyhow;

pub const FAKE_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.";

pub fn download_with_checksum<T: sha2::Digest + Write>(
    url: &str,
    path: &PathBuf,
    wanted_hash: &str,
) -> Result<(), anyhow::Error> {
    println!("Downloading jarfile from {url}");
    let mut resp = ureq::get(url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_reader();

    if let Some(prefix) = path.parent() {
        std::fs::create_dir_all(prefix).unwrap();
    }

    let mut output = File::create(path)?;

    let digest = {
        let mut hasher = T::new();
        let mut buf = [0; 1024];

        loop {
            let count = resp.read(&mut buf)?;
            if count == 0 {
                break;
            }

            _ = output.write(&buf[..count])?;
            hasher.update(&buf[..count]);
        }

        hasher.finalize()
    };

    let hash = digest
        .as_slice()
        .iter()
        .fold(String::new(), |acc, b| acc + &format!("{b:02x}"));

    if hash != wanted_hash {
        return Err(anyhow!("hashes do not match"));
    }

    Ok(())
}
