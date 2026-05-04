use crate::core::state_machine::GameState;

pub fn title_for(state: GameState) -> &'static str {
    match state {
        GameState::MainMenu => "Main Menu",
        GameState::Overworld => "Overworld",
        GameState::Battle => "Battle",
        GameState::Inventory => "Inventory",
        GameState::Dialogue => "Dialogue",
        GameState::Cutscene => "Cutscene",
    }
}
