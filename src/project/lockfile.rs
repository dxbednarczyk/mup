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

pub struct Lockfile {
    items: HashMap<String, LockfileEntry>,
}

impl Lockfile {
    pub fn new() -> Result<Self, anyhow::Error> {
        let mut lf = Self {
            items: Default::default(),
        };

        lf.items = if !Path::new(LOCKFILE_PATH).exists() {
            File::create(LOCKFILE_PATH)?;
            HashMap::new()
        } else {
            let mut current_lockfile = File::open(LOCKFILE_PATH)?;
            let lockfile_size = current_lockfile.metadata()?.size() as usize;

            let mut contents = String::with_capacity(lockfile_size);
            current_lockfile.read_to_string(&mut contents)?;

            toml::from_str(&contents)?
        };

        Ok(lf)
    }

    pub fn get(&mut self, project_id: &str) -> Result<&LockfileEntry, anyhow::Error> {
        self.items
            .get(project_id)
            .ok_or(anyhow!("key {project_id} not found"))
    }

    pub fn add(
        &mut self,
        version: &actions::Version,
        project: &actions::ProjectInfo,
        project_file: &actions::ProjectFile,
    ) -> Result<(), anyhow::Error> {
        if self.get(&project.slug).is_ok() {
            return Err(anyhow!(
                "{} already has an entry in the lockfile",
                &project.slug
            ));
        }

        let entry = LockfileEntry {
            installed_version: version.version_number.clone(),
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

#[derive(Deserialize, Serialize)]
pub struct LockfileEntry {
    installed_version: String,
    filename: String,
    remote_url: String,
    sha512: String,
}
