use std::path::PathBuf;

use super::Result;

pub struct TestHandler;

impl TestHandler {
    pub fn execute(_path: Option<PathBuf>) -> Result<()> {
        todo!()
    }
}
