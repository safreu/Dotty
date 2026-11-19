use crate::{
    domain::{Action, BackupId, BackupStatus, Backups},
    filesystem::{PathKind, RealFileSystem},
    persistence::{read_toml, serialize_toml},
    services::{
        ActionExecutor, Error, PlanBuilder, Result, backup::build::BackupBuilder,
        snapshot::SnapshotBuilder,
    },
};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::{
    domain::{Backup, Plan},
    filesystem::Filesystem,
};

pub struct BackupService;

impl BackupService {
    pub fn prepare(plan: &Plan, backup_root: &Path) -> Result<(Backup, Plan)> {
        let backup_id = BackupId::new();
        let backup_dir = backup_root.join("operations").join(backup_id.to_string());

        let sources = Self::backup_sources(plan)?;

        let mut backup = Backup::new(backup_id);
        let mut builder = PlanBuilder::new();

        if sources.is_empty() {
            return Ok((backup, builder.build()));
        }

        let fs = RealFileSystem;
        let home = PathBuf::from(std::env::var("HOME")?);

        builder = builder.ensure_directories(&fs, &home, &backup_dir)?;

        for (index, (original, kind)) in sources.iter().enumerate() {
            let destination = backup_dir.join(index.to_string());

            let (entry, action) = BackupBuilder::build(original, &destination, *kind)?;

            backup.add(entry);
            builder = builder.push(action);
        }

        Ok((backup, builder.build()))
    }

    pub fn save(backup: &Backup, backups_toml: &Path) -> Result<()> {
        let mut backups: Backups = read_toml(backups_toml)?;

        backups.add(backup.clone())?;

        let content = serialize_toml(&backups)?;

        let mut fs = RealFileSystem;
        fs.write_file(backups_toml, &content)?;

        Ok(())
    }

    fn backup_sources(plan: &Plan) -> Result<Vec<(PathBuf, PathKind)>> {
        let mut simulated = SnapshotBuilder::new().build(plan)?;
        let mut seen = HashSet::new();
        let mut sources = Vec::new();

        for action in plan.actions() {
            if let Some(path) = Self::backup_path(action)
                && let Some(original) = simulated.origin(path)
            {
                let original = original.to_path_buf();

                if seen.insert(original.clone())
                    && let Some(kind) = simulated.kind(path)?
                {
                    sources.push((original, kind))
                }
            }
            ActionExecutor::execute(&mut simulated, action)?;
        }

        Ok(sources)
    }

    fn backup_path(action: &Action) -> Option<&Path> {
        match action {
            Action::Rename { from, .. } => Some(from),

            Action::WriteFile { path, .. }
            | Action::RemoveFile(path)
            | Action::RemoveSymlink(path) => Some(path),

            _ => None,
        }
    }

    pub fn update_status(id: BackupId, status: BackupStatus, backups_toml: &Path) -> Result<()> {
        let mut backups: Backups = read_toml(backups_toml)?;

        let backup = backups.get_mut(id).ok_or(Error::BackupNotFound(id))?;

        backup.set_status(status);

        let content = serialize_toml(&backups)?;

        let mut fs = RealFileSystem;
        fs.write_file(backups_toml, &content)?;

        Ok(())
    }
}
