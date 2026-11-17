use std::path::Path;

use crate::filesystem::PathKind;
use crate::services::Result;
use crate::services::manage::ManageRules;

pub struct ManageValidator;

impl ManageValidator {
    pub fn validate(path: &Path, kind: Option<PathKind>) -> Result<()> {
        ManageRules::require_manageable(path, kind)
    }
}
