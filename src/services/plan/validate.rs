use crate::{
    domain::Plan,
    services::{ActionExecutor, ActionValidator, snapshot::SnapshotBuilder},
};

use crate::services::Result;

pub struct PlanValidator;

impl PlanValidator {
    pub fn validate(plan: &Plan) -> Result<()> {
        let mut simulated = SnapshotBuilder::new().build(plan)?;

        for action in plan.actions() {
            ActionValidator::validate(&simulated, action)?;
            ActionExecutor::execute(&mut simulated, action)?;
        }

        Ok(())
    }
}
