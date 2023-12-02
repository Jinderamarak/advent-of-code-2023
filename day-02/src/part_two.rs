use super::parsing::{Game, Cubes};

pub fn get_game_powers(data: &str) -> Vec<u32> {
    data.lines()
        .map(Game::parse)
        .map(|game| {
            let max = game.peeks.iter().fold(Cubes::zero(), |a, b| a.max(b));
            max.red * max.green * max.blue
        })
        .collect()
}