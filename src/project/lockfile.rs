#![allow(clippy::cast_possible_truncation)]

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use super::actions;

const LOCKFILE_PATH: &str = "pap.lock";

#[derive(Default)]
pub struct Lockfile {
    items: HashMap<String, Entry>,
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    installed_version: String,
    filename: String,
    remote_url: String,
    sha512: String,
}

impl Lockfile {
    pub fn new() -> Result<Self, anyhow::Error> {
        let items = if Path::new(LOCKFILE_PATH).exists() {
            let mut current_lockfile = File::open(LOCKFILE_PATH)?;

            let lockfile_size = current_lockfile.metadata()?.size();
            let mut contents = String::with_capacity(lockfile_size as usize);

            current_lockfile.read_to_string(&mut contents)?;

            toml::from_str(&contents)?
        } else {
            File::create(LOCKFILE_PATH)?;

            HashMap::default()
        };

        Ok(Self { items })
    }

    pub fn get(&mut self, project_id: &str) -> Result<&Entry, anyhow::Error> {
        self.items
            .get(project_id)
            .ok_or_else(|| anyhow!("key {project_id} not found"))
    }

    pub fn add(
        &mut self,
        version: &actions::Version,
        project: &actions::ProjectInfo,
        project_file: &actions::ProjectFile,
    ) -> Result<(), anyhow::Error> {
        let entry = Entry {
            installed_version: version.number.clone(),
            filename: project_file.filename.clone(),
            remote_url: project_file.url.clone(),
            sha512: project_file.hashes.sha512.clone(),
        };

        self.items.insert(project.slug.clone(), entry);

        let mut output = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(LOCKFILE_PATH)?;

        output.write_all(toml::to_string(&self.items)?.as_bytes())?;

        Ok(())
    }
}
