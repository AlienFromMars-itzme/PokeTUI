use super::pokemon::OwnedPokemon;

pub fn damage(attacker: &OwnedPokemon, defender: &OwnedPokemon, power: u16) -> i32 {
    let atk = attacker.attack.max(1) as i32;
    let def = defender.defense.max(1) as i32;
    (((2 * attacker.level as i32 / 5 + 2) * power as i32 * atk / def) / 50 + 2).max(1)
}
