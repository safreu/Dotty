use std::path::Path;

use super::{PlanExecutor, PlanValidator};
use crate::{
    domain::{BackupStatus, Plan},
    services::{Error, Result, backup::BackupService},
};
pub struct PlanService;

impl PlanService {
    pub fn run(plan: &Plan, backup_root: &Path, backups_toml: &Path) -> Result<()> {
        PlanValidator::validate(plan)?;

        let (backup, backup_plan) = BackupService::prepare(plan, backup_root)?;

        PlanValidator::validate(&backup_plan)?;
        PlanExecutor::execute(&backup_plan)?;

        if !backup.entries().is_empty() {
            BackupService::save(&backup, backups_toml)?;
        }

        let result = PlanExecutor::execute_with_rollback(plan, &backup);

        if backup.entries().is_empty() {
            return result;
        }

        let status = match &result {
            Ok(()) => BackupStatus::Completed,
            Err(Error::RollbackFailed { .. }) => BackupStatus::RecoveryRequired,
            Err(_) => BackupStatus::RolledBack,
        };
        let status_result = BackupService::update_status(backup.id(), status, backups_toml);

        match (result, status_result) {
            (Ok(()), Ok(())) => Ok(()),

            (Err(original), Ok(())) => Err(original),

            (Ok(()), Err(status)) => Err(status),

            (Err(original), Err(status)) => Err(Error::BackupStatusUpdateFailed {
                original: Box::new(original),
                status: Box::new(status),
            }),
        }
    }
}
