use super::{PlanExecutor, PlanValidator};
use crate::{domain::Plan, services::Result};
pub struct PlanService;

impl PlanService {
    pub fn run(plan: &Plan) -> Result<()> {
        PlanValidator::validate(plan)?;
        PlanExecutor::execute(plan)?;

        Ok(())
    }
}
