#[derive(Debug, Clone)]
pub struct OwnedPokemon {
    pub species: String,
    pub level: u8,
    pub hp: i32,
    pub max_hp: i32,
    pub speed: u16,
    pub attack: u16,
    pub defense: u16,
    pub moves: Vec<String>,
}
