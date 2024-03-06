use crate::server::lockfile::Lockfile;

use anyhow::{anyhow, Result};

pub fn fetch(lockfile: &Lockfile, project_id: &str, version: &str) -> Result<super::Info> {
    todo!()
}
