use std::str::FromStr;

pub mod cube;
pub mod round;

pub use cube::Cubes;
use round::{Round, RoundError};

/// An error describing something wrong with the game
#[derive(Debug)]
pub struct GameError(RoundError);

impl std::error::Error for GameError {}

impl From<RoundError> for GameError {
    fn from(value: RoundError) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game error! {}", self.0)
    }
}

/// A game of "cube conundrum"
pub struct Game {
    rounds: Vec<Round>,
    limits: Cubes,
}

impl Game {
    /// Creates a new game
    #[inline(always)]
    pub fn new(string: &str, limits: Cubes) -> Result<Self, GameError> {
        let rounds = string.split('\n').map(Round::from_str).collect::<Result<Vec<_>, _>>()?;

        Ok(Self { rounds, limits })
    }

    /// Returns the sum of all valid round IDs
    #[inline(always)]
    pub fn get_ids_sum(&self) -> i32 {
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

    /// Returns the sum of the minimum power of all rounds
    #[inline(always)]
    pub fn get_minimum_powers_sum(&self) -> i32 {
        self.rounds.iter().map(|round| round.get_minimum_set().power()).sum()
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
        let mut limits = Cubes::new();
        limits.insert(Color::Red, 12);
        limits.insert(Color::Green, 13);
        limits.insert(Color::Blue, 14);

        let game = Game::new(GAME, limits).expect("Failed to create game");
        assert_eq!(game.get_ids_sum(), 8);
    }

    #[test]
    fn test_solution_2() {
        let limits = Cubes::new();
        let game = Game::new(GAME, limits).expect("Failed to create game");

        assert_eq!(game.get_minimum_powers_sum(), 2286)
    }
}
