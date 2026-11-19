use crate::domain::RollbackPlan;
use crate::filesystem::RealFileSystem;
use crate::services::{ActionExecutor, ActionValidator, Error, Result};

pub struct RollbackService;

impl RollbackService {
    pub fn execute(plan: &RollbackPlan) -> Result<()> {
        let mut fs = RealFileSystem;
        let mut first_error = None;

        for entry in plan.entries_rev() {
            for action in entry.inverse() {
                let result = ActionValidator::validate(&fs, action)
                    .and_then(|_| ActionExecutor::execute(&mut fs, action));

                if let Err(error) = result {
                    if first_error.is_none() {
                        first_error = Some(error)
                    }

                    break;
                }
            }
        }

        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    pub fn recover(plan: &RollbackPlan, original: Error) -> Error {
        match Self::execute(plan) {
            Ok(()) => original,
            Err(rollback) => Error::RollbackFailed {
                original: Box::new(original),
                rollback: Box::new(rollback),
            },
        }
    }
}
