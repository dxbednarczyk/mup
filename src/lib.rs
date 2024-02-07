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

    let mut hash = String::new();
    for i in hash_bytes.as_slice() {
        let mut formatted = format!("{i:x}");

        // sometimes LowerHex returns i.e. 7 instead of 07 when formatting
        // we need to include that 0 in the context of the hash
        if formatted.len() == 1 {
            formatted.insert(0, '0');
        }

        hash.push_str(&formatted);
    }

    if hash != wanted_hash {
        fs::remove_file(filename)?;

        return Err(anyhow!("hashes do not match"));
    }

    Ok(())
}
