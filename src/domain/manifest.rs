use crate::domain::ManagedEntry;

use super::Error;
use super::Result;

use std::{collections::HashSet, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    manages: HashSet<ManagedEntry>,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            manages: HashSet::new(),
        }
    }

    pub fn manages(&self) -> &HashSet<ManagedEntry> {
        &self.manages
    }

    pub fn add(&mut self, entry: ManagedEntry) -> Result<()> {
        if let Some(existing) = self
            .manages
            .iter()
            .find(|existing| Self::paths_conflict(existing.target().path(), entry.target().path()))
        {
            return Err(Error::ManagedTargetConflict {
                existing: existing.target().path().to_path_buf(),
                new: entry.target().path().to_path_buf(),
            });
        }

        if let Some(existing) = self
            .manages
            .iter()
            .find(|existing| Self::paths_conflict(existing.source().path(), entry.source().path()))
        {
            return Err(Error::ManagedSourceConflict {
                existing: existing.source().path().to_path_buf(),
                new: entry.source().path().to_path_buf(),
            });
        }

        self.manages.insert(entry);

        Ok(())
    }

    fn paths_conflict(existing: &Path, new: &Path) -> bool {
        existing.starts_with(new) || new.starts_with(existing)
    }

    pub fn remove(&mut self, entry: &ManagedEntry) -> bool {
        self.manages.remove(entry)
    }

    pub fn validate(&self) -> Result<()> {
        let mut validated = Manifest::new();

        for entry in self.manages() {
            validated.add(entry.clone())?;
        }

        Ok(())
    }
}
