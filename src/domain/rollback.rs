use crate::domain::Action;

#[derive(Debug)]
pub struct RollbackEntry {
    original: Action,
    inverse: Vec<Action>,
}

impl RollbackEntry {
    pub fn new(original: Action, inverse: Vec<Action>) -> Self {
        Self { original, inverse }
    }

    pub fn original(&self) -> &Action {
        &self.original
    }

    pub fn inverse(&self) -> &[Action] {
        &self.inverse
    }
}

#[derive(Debug, Default)]
pub struct RollbackPlan {
    entries: Vec<RollbackEntry>,
}

impl RollbackPlan {
    pub fn new(entries: Vec<RollbackEntry>) -> Self {
        Self { entries }
    }

    pub fn push(&mut self, entry: RollbackEntry) {
        self.entries.push(entry);
    }

    pub fn entries_rev(&self) -> impl Iterator<Item = &RollbackEntry> {
        self.entries.iter().rev()
    }
}
