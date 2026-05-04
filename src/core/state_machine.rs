#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    MainMenu,
    Overworld,
    Battle,
    Inventory,
    Dialogue,
    Cutscene,
}

#[derive(Debug)]
pub struct StateMachine {
    current: GameState,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            current: GameState::MainMenu,
        }
    }
    pub fn current(&self) -> GameState {
        self.current
    }
    pub fn transition_to(&mut self, next: GameState) {
        self.current = next;
    }
}
