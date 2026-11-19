use super::{Error, Result};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BackupId(Uuid);

impl BackupId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for BackupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEntry {
    original: PathBuf,
    stored: PathBuf,
}

impl BackupEntry {
    pub fn new(original: PathBuf, stored: PathBuf) -> Self {
        Self { original, stored }
    }

    pub fn original(&self) -> &PathBuf {
        &self.original
    }

    pub fn stored(&self) -> &PathBuf {
        &self.stored
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    id: BackupId,
    entries: Vec<BackupEntry>,
    status: BackupStatus,
}

impl Backup {
    pub fn new(id: BackupId) -> Self {
        Self {
            id,
            entries: Vec::new(),
            status: BackupStatus::Pending,
        }
    }

    pub fn id(&self) -> BackupId {
        self.id
    }

    pub fn entries(&self) -> &[BackupEntry] {
        &self.entries
    }

    pub fn add(&mut self, entry: BackupEntry) {
        self.entries.push(entry);
    }

    pub fn set_status(&mut self, status: BackupStatus) {
        self.status = status;
    }

    pub fn status(&self) -> BackupStatus {
        self.status
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Backups {
    backups: Vec<Backup>,
}

impl Backups {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, backup: Backup) -> Result<()> {
        if self.backups().iter().any(|b| b.id() == backup.id()) {
            return Err(Error::DuplicateBackupId(backup.id()));
        }

        self.backups.push(backup);
        Ok(())
    }

    pub fn backups(&self) -> &[Backup] {
        &self.backups
    }

    pub fn get(&self, id: BackupId) -> Option<&Backup> {
        self.backups.iter().rev().find(|b| b.id() == id)
    }

    pub fn latest(&self) -> Option<&Backup> {
        self.backups.last()
    }

    pub fn get_mut(&mut self, id: BackupId) -> Option<&mut Backup> {
        self.backups.iter_mut().find(|backup| backup.id() == id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupStatus {
    Pending,
    Completed,
    RolledBack,
    RecoveryRequired,
}
