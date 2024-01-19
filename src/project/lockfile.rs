use std::{
    fs::{File, self},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};

use serde::Serialize;
use toml_edit::Document;
use anyhow::anyhow;

use super::actions;

const LOCKFILE_PATH: &str = "pap.lock";

#[derive(Serialize)]
struct LockfileEntry<'a> {
    installed_version: &'a String,
    filename: &'a String,
    remote_url: &'a String,
    sha512: &'a String,
}

pub fn add(
    version: &actions::Version,
    project: &actions::ProjectInfo,
    project_file: &actions::ProjectFile,
) -> Result<(), anyhow::Error> {
    let mut document = if !Path::new(LOCKFILE_PATH).exists() {
        File::create(LOCKFILE_PATH)?;
        toml_edit::Document::new()
    } else {
        let mut current_lockfile = File::open(LOCKFILE_PATH)?;
        let lockfile_size = current_lockfile.metadata()?.size() as usize;

        let mut contents = String::with_capacity(lockfile_size);
        current_lockfile.read_to_string(&mut contents)?;

        contents.parse::<Document>()?
    };

    let entry = LockfileEntry {
        installed_version: &version.version_number,
        filename: &project_file.filename,
        remote_url: &project_file.url,
        sha512: &project_file.hashes.sha512,
    };

    let serialized = Serialize::serialize(&entry, toml_edit::ser::ValueSerializer::new())?;

    if document.get(&project.slug).is_some() {
        return Err(anyhow!("{} already has an entry in the lockfile", &project.slug));
    }

    document[&project.slug] = toml_edit::value(serialized);

    let mut output = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOCKFILE_PATH)?;

    let stringified = document.to_string();

    output.write_all(stringified.as_bytes())?;

    Ok(())
}
