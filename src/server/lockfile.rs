use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Result};
use log::info;
use serde::{Deserialize, Serialize};
use versions::Versioning;

use crate::{loader, plugin};

const LOCKFILE_PATH: &str = "pap.lock";

#[derive(Deserialize, Default, Serialize)]
pub struct Lockfile {
    pub loader: Loader,
    pub projects: Vec<plugin::Info>,
}

#[derive(Deserialize, Serialize)]
pub struct Loader {
    pub name: String,
    pub minecraft_version: String,
    pub version: String,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            name: String::default(),
            minecraft_version: String::from("latest"),
            version: String::from("latest"),
        }
    }
}

impl Lockfile {
    pub fn init() -> Result<Self> {
        if PathBuf::from(LOCKFILE_PATH).exists() {
            let mut current_lockfile = File::open(LOCKFILE_PATH)?;

            let mut contents = String::new();
            current_lockfile.read_to_string(&mut contents)?;

            return Ok(serde_json::from_str(&contents)?);
        }

        File::create(LOCKFILE_PATH)?;

        Ok(Self {
            loader: Loader::default(),
            projects: vec![],
        })
    }

    pub fn with_params(minecraft_version: &str, loader: &str) -> Result<Self> {
        let mv = Versioning::new(minecraft_version).unwrap();
        if mv.is_complex() {
            return Err(anyhow!(
                "minecraft version {} is invalid",
                minecraft_version
            ));
        }

        let l = Loader {
            name: loader.to_string(),
            minecraft_version: minecraft_version.to_string(),
            version: String::from("latest"),
        };

        File::create(LOCKFILE_PATH)?;

        let mut lf = Self {
            loader: l,
            projects: vec![],
        };

        lf.save()?;

        Ok(lf)
    }

    pub fn get(&self, project_id: &str) -> Result<&plugin::Info> {
        self.projects
            .iter()
            .find(|p| p.slug == project_id)
            .ok_or_else(|| anyhow!("key {project_id} not found"))
    }

    pub fn add(&mut self, info: plugin::Info) -> Result<()> {
        self.projects.push(info);

        self.save()?;

        Ok(())
    }

    pub fn remove(&mut self, slug: &str, keep_jarfile: bool, remove_orphans: bool) -> Result<()> {
        if self.get(slug).is_err() {
            return Err(anyhow!("project {slug} does not exist in the lockfile"));
        }

        let mut projects = self.projects.iter();

        let idx = projects
            .position(|p| p.slug == slug)
            .ok_or_else(|| anyhow!("{slug} does not exist in the lockfile"))?;

        let entry = self.projects[idx].clone();

        let mut to_remove = vec![entry.slug];

        for dep in entry.dependencies {
            if !remove_orphans {
                break;
            }

            let cant_be_removed = projects.any(|p| {
                let contains = p.dependencies.iter().find(|d| **d == dep);

                contains.is_some() && p.slug != slug
            });

            if !cant_be_removed {
                to_remove.push(dep.id);
            }
        }

        for slug in to_remove {
            let idx = self
                .projects
                .iter()
                .position(|p| p.slug == slug || p.id == slug)
                .ok_or_else(|| anyhow!("{slug} does not exist in the lockfile"))?;

            if !keep_jarfile {
                fs::remove_file(&self.projects[idx].get_file_path(&self.loader.name))?;
            }

            self.projects.remove(idx);
        }

        self.save()?;

        Ok(())
    }

    pub fn is_initialized(&mut self) -> bool {
        let minecraft_version = &self.loader.minecraft_version;

        let version = Versioning::new(minecraft_version).unwrap();

        !version.is_complex() && loader::parse(&self.loader.name).is_ok()
    }

    pub fn save(&mut self) -> Result<()> {
        info!("saving transaction to lockfile");

        let mut output = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(LOCKFILE_PATH)?;

        output.write_all(serde_json::to_string(&self)?.as_bytes())?;

        Ok(())
    }
}
