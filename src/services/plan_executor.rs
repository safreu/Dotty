use crate::{
    domain::Plan,
    services::plan_validator::PlanValidator,
    utils::{ActionExecutor, RealFileSystem},
};

use super::Result;
pub struct PlanExecutor;

impl PlanExecutor {
    pub fn execute(plan: &Plan) -> Result<()> {
        PlanValidator::validate(plan)?;

        let mut fs = RealFileSystem;

        for action in plan.actions() {
            ActionExecutor::execute(&mut fs, action)?;
        }

        Ok(())
    }
}
