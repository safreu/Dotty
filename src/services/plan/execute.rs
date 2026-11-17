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
}
