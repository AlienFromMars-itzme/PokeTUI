#[derive(Debug, Clone)]
pub enum GameEvent {
    Tick,
    OpenInventory,
    CloseMenu,
    StartBattle,
    EndBattle,
}
