#![allow(clippy::cast_possible_truncation)]

use std::{
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use versions::Versioning;

use crate::loader::Loader;
use crate::project::actions;

const LOCKFILE_PATH: &str = "pap.lock";

#[derive(Deserialize, Default, Serialize)]
pub struct Lockfile {
    pub minecraft_version: String,
    pub loader: String,
    project: Vec<Entry>,
}

#[derive(Deserialize, Serialize)]
pub struct Entry {
    slug: String,
    installed_version: String,
    path: PathBuf,
    remote_url: String,
    sha512: String,
}

impl Lockfile {
    pub fn init() -> Result<Self, anyhow::Error> {
        let lf = if PathBuf::from(LOCKFILE_PATH).exists() {
            let mut current_lockfile = File::open(LOCKFILE_PATH)?;

            let lockfile_size = current_lockfile.metadata()?.size();
            let mut contents = String::with_capacity(lockfile_size as usize);

            current_lockfile.read_to_string(&mut contents)?;

            toml::from_str(&contents)?
        } else {
            File::create(LOCKFILE_PATH)?;

            Self {
                minecraft_version: String::from("undefined"),
                loader: String::from("undefined"),
                project: vec![],
            }
        };

        Ok(lf)
    }

    pub fn with_params(minecraft_version: &str, loader: &str) -> Result<Self, anyhow::Error> {
        File::create(LOCKFILE_PATH)?;

        let mut lf = Self {
            minecraft_version: String::from(minecraft_version),
            loader: String::from(loader),
            project: vec![],
        };

        lf.write_out()?;

        Ok(lf)
    }

    pub fn get(&mut self, project_id: &str) -> Result<&Entry, anyhow::Error> {
        self.project
            .iter()
            .find(|p| p.slug == project_id)
            .ok_or_else(|| anyhow!("key {project_id} not found"))
    }

    pub fn add(
        &mut self,
        version: &actions::Version,
        project: &actions::ProjectInfo,
        project_file: &actions::ProjectFile,
        path: PathBuf,
    ) -> Result<(), anyhow::Error> {
        let entry = Entry {
            slug: project.slug.clone(),
            installed_version: version.number.clone(),
            path,
            remote_url: project_file.url.clone(),
            sha512: project_file.hashes.sha512.clone(),
        };

        self.project.push(entry);

        self.write_out()?;

        Ok(())
    }

    pub fn remove(&mut self, slug: &str, keep_jarfile: bool) -> Result<(), anyhow::Error> {
        let entry = self
            .project
            .iter()
            .position(|p| p.slug == slug)
            .ok_or_else(|| anyhow!("{slug} does not exist in the lockfile"))?;

        if !keep_jarfile {
            fs::remove_file(&self.project[entry].path)?;
        }

        self.project.remove(entry);

        self.write_out()?;

        Ok(())
    }

    fn write_out(&mut self) -> Result<(), anyhow::Error> {
        let mut output = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(LOCKFILE_PATH)?;

        output.write_all(toml::to_string(&self)?.as_bytes())?;

        Ok(())
    }

    pub fn is_initialized(&mut self) -> bool {
        let mv = Versioning::new(&self.minecraft_version).unwrap();

        return !mv.is_complex() && Loader::NAMES.contains(&self.loader.as_str());
    }
}
