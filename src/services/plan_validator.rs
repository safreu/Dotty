use crate::{
    domain::Plan,
    utils::{ActionValidator, SnapshotBuilder},
};

use super::Result;

pub struct PlanValidator;

impl PlanValidator {
    pub fn validate(plan: &Plan) -> Result<()> {
        let mut simulated = SnapshotBuilder::new().build(plan)?;

        for action in plan.actions() {
            ActionValidator::validate(&simulated, action)?;
            simulated.apply(action)?;
        }

        Ok(())
    }
}
