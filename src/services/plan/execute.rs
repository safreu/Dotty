use crate::domain::{Backup, RollbackEntry, RollbackPlan};
use crate::services::rollback::{RollbackBuilder, RollbackService};
use crate::{domain::Plan, filesystem::RealFileSystem, services::ActionExecutor};

use crate::services::Result;
pub struct PlanExecutor;

impl PlanExecutor {
    pub fn execute(plan: &Plan) -> Result<()> {
        let mut fs = RealFileSystem;

        for action in plan.actions() {
            ActionExecutor::execute(&mut fs, action)?;
        }

        Ok(())
    }

    pub fn execute_with_rollback(plan: &Plan, backup: &Backup) -> Result<()> {
        let mut fs = RealFileSystem;
        let mut rollback = RollbackPlan::default();

        for action in plan.actions() {
            let inverse = match RollbackBuilder::build(&mut fs, action, backup) {
                Ok(inverse) => inverse,
                Err(error) => {
                    return Err(RollbackService::recover(&rollback, error));
                }
            };

            if let Err(error) = ActionExecutor::execute(&mut fs, action) {
                return Err(RollbackService::recover(&rollback, error));
            }

            rollback.push(RollbackEntry::new(action.clone(), inverse));
        }

        Ok(())
    }
}
