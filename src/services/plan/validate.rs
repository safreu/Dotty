use crate::{
    domain::{Action, Plan},
    services::{ActionExecutor, ActionValidator, snapshot::SnapshotBuilder},
};

use crate::services::{Error, Result};

pub struct PlanValidator;

impl PlanValidator {
    pub fn validate(plan: &Plan) -> Result<()> {
        let mut simulated = SnapshotBuilder::new().build(plan)?;

        for action in plan.actions() {
            if matches!(action, Action::RemoveDirectoryAll(_)) {
                return Err(Error::RollbackOnlyAction);
            }
            ActionValidator::validate(&simulated, action)?;
            ActionExecutor::execute(&mut simulated, action)?;
        }

        Ok(())
    }
}
