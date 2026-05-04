use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonStats {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub sp_attack: u16,
    pub sp_defense: u16,
    pub speed: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnMove {
    pub level: u8,
    pub r#move: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evolution {
    pub evolves_to: Option<String>,
    pub level: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u16,
    pub name: String,
    pub types: Vec<String>,
    pub stats: PokemonStats,
    pub abilities: Vec<String>,
    pub learnset: Vec<LearnMove>,
    pub evolution: Evolution,
    pub catch_rate: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub name: String,
    pub r#type: String,
    pub category: String,
    pub power: u16,
    pub accuracy: u8,
    pub pp: u8,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub effect: String,
}

pub fn load_json_vec<T: for<'de> Deserialize<'de>>(
    path: impl AsRef<Path>,
) -> anyhow::Result<Vec<T>> {
    let path_ref = path.as_ref();
    let raw =
        fs::read_to_string(path_ref).with_context(|| format!("read {}", path_ref.display()))?;
    let parsed = serde_json::from_str::<Vec<T>>(&raw)
        .with_context(|| format!("parse {}", path_ref.display()))?;
    Ok(parsed)
}
