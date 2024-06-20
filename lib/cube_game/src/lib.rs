mod cube;
mod round;

use std::str::FromStr;

use cube::Limits;
use round::{Round, RoundError};

#[derive(Debug)]
pub struct GameError(RoundError);

impl From<RoundError> for GameError {
    fn from(value: RoundError) -> Self {
        Self(value)
    }
}

pub struct Game {
    rounds: Vec<Round>,
    limits: Limits,
}

impl Game {
    pub fn new(string: &str, limits: Limits) -> Result<Self, GameError> {
        let rounds = string
            .split('\n')
            .map(Round::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rounds, limits })
    }

    pub fn get_id_sums(&self) -> i32 {
        self.rounds
            .iter()
            .filter_map(|round| {
                if round.is_valid(&self.limits) {
                    return Some(round.id());
                }
                None
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use cube::Color;

    use super::*;

    const GAME: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_solution_1() {
        let mut limits = Limits::new();
        limits.insert(Color::Red, 12);
        limits.insert(Color::Green, 13);
        limits.insert(Color::Blue, 14);

        let game = Game::new(GAME, limits).expect("Failed to create game");
        assert_eq!(game.get_id_sums(), 8);
    }
}
