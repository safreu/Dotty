use crate::domain::Action;

pub struct Plan {
    actions: Vec<Action>,
}

impl Plan {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn into_actions(self) -> Vec<Action> {
        self.actions
    }

    pub fn extend(&mut self, other: Plan) {
        self.actions.extend(other.actions);
    }

    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
}
