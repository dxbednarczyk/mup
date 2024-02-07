use std::{
    fs::{self, File},
    io::{self, Write},
};

use anyhow::anyhow;

pub const FAKE_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.";

pub fn download_with_checksum<T: sha2::Digest + Write>(
    url: &str,
    filename: &str,
    wanted_hash: &str,
) -> Result<(), anyhow::Error> {
    let resp = ureq::get(url)
        .set("User-Agent", FAKE_USER_AGENT)
        .call()?
        .into_reader();

    let mut output = File::create(filename)?;

    let mut hasher = T::new();

    let mut tee = tee::tee(resp, &mut output);
    io::copy(&mut tee, &mut hasher)?;

    let hash_bytes = hasher.finalize();

    let hash = hash_bytes
        .as_slice()
        .iter()
        .fold(String::new(), |acc, b| acc + &format!("{b:02x}"));

    if hash != wanted_hash {
        fs::remove_file(filename)?;

        return Err(anyhow!("hashes do not match"));
    }

    Ok(())
}
