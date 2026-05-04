use crate::core::state_machine::StateMachine;

pub struct Engine {
    pub state: StateMachine,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: StateMachine::new(),
        }
    }
}
