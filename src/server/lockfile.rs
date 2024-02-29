#![allow(clippy::cast_possible_truncation)]

use std::{
    fs::{self, File},
    io::{Read, Write},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use anyhow::anyhow;
use log::info;
use serde::{Deserialize, Serialize};
use versions::Versioning;

use crate::{loader, project::actions};

const LOCKFILE_PATH: &str = "pap.lock";

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Lockfile {
    pub loader: Loader,
    pub projects: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize)]
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

impl Loader {
    pub fn project_path(&self) -> String {
        match self.name.as_str() {
            "fabric" | "forge" => String::from("./mods/"),
            "paper" => String::from("./plugins/"),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry {
    pub slug: String,
    pub project_id: String,
    pub version_id: String,
    pub path: PathBuf,
    pub remote: String,
    pub sha512: String,
    pub requires: Vec<String>,
}

impl Lockfile {
    pub fn init() -> Result<Self, anyhow::Error> {
        if PathBuf::from(LOCKFILE_PATH).exists() {
            let mut current_lockfile = File::open(LOCKFILE_PATH)?;

            let lockfile_size = current_lockfile.metadata()?.size();
            let mut contents = String::with_capacity(lockfile_size as usize);

            current_lockfile.read_to_string(&mut contents)?;

            return Ok(serde_json::from_str(&contents)?);
        }

        File::create(LOCKFILE_PATH)?;

        Ok(Self {
            loader: Loader::default(),
            projects: vec![],
        })
    }

    pub fn with_params(minecraft_version: &str, loader: &str) -> Result<Self, anyhow::Error> {
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

    pub fn get(&self, project_id: &str) -> Result<&Entry, anyhow::Error> {
        self.projects
            .iter()
            .find(|p| p.slug == project_id)
            .ok_or_else(|| anyhow!("key {project_id} not found"))
    }

    pub fn add(
        &mut self,
        version: &actions::Version,
        slug: &str,
        project_file: &actions::ProjectFile,
    ) -> Result<(), anyhow::Error> {
        let entry = Entry {
            slug: slug.to_string(),
            project_id: version.project_id.clone(),
            version_id: version.id.clone(),
            path: project_file.path.clone(),
            remote: project_file.url.clone(),
            sha512: project_file.hashes.sha512.clone(),
            requires: version
                .dependencies
                .iter()
                .map(|d| d.project_id.clone())
                .collect(),
        };

        self.projects.push(entry);

        self.save()?;

        Ok(())
    }

    pub fn remove(
        &mut self,
        slug: &str,
        keep_jarfile: bool,
        remove_orphans: bool,
    ) -> Result<(), anyhow::Error> {
        if self.get(slug).is_err() {
            return Err(anyhow!("project {slug} does not exist in the lockfile"));
        }

        let mut projects = self.projects.iter();
        let mut to_remove = vec![];

        let idx = projects
            .position(|p| p.slug == slug)
            .ok_or_else(|| anyhow!("{slug} does not exist in the lockfile"))?;

        let entry = self.projects[idx].clone();

        to_remove.push(entry.slug);

        if remove_orphans {
            for dep in entry.requires {
                let cant_be_removed = projects.any(|p| {
                    if p.slug == slug {
                        return false;
                    }

                    let contains = p.requires.iter().find(|d| **d == dep);

                    contains.is_some()
                });

                if !cant_be_removed {
                    to_remove.push(dep);
                }
            }
        }

        for slug in to_remove {
            let idx = self
                .projects
                .iter()
                .position(|p| p.slug == slug || p.project_id == slug)
                .ok_or_else(|| anyhow!("{slug} does not exist in the lockfile"))?;

            if !keep_jarfile {
                fs::remove_file(&self.projects[idx].path)?;
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

    pub fn save(&mut self) -> Result<(), anyhow::Error> {
        info!("saving transaction to lockfile");

        let mut output = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(LOCKFILE_PATH)?;

        output.write_all(serde_json::to_string(&self)?.as_bytes())?;

        Ok(())
    }
}
