use std::{num::ParseIntError, str::FromStr};

/// An error that can occur when parsing a Race
#[derive(Debug)]
pub enum ParseRaceError {
    ParseInt(ParseIntError),
    Invalid(String),
}

impl std::error::Error for ParseRaceError {}

impl From<ParseIntError> for ParseRaceError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl std::fmt::Display for ParseRaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseInt(e) => write!(f, "Failed to parse integer: {}", e),
            Self::Invalid(e) => write!(f, "Invalid input: {}", e),
        }
    }
}

pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    /// Create a new Race
    #[inline(always)]
    pub const fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    /// Check if the boat has won the
    #[inline(always)]
    pub const fn has_won(&self, boat: &Boat) -> bool {
        boat.distance_traveled > self.distance
    }

    /// Find the amount of time the boat can hold the button and still win
    #[inline(always)]
    pub fn find_winning_conditions_amount(&self) -> u64 {
        // We are not interested in holding the button for 0 ms,
        // as this would result in 0 distance
        let mut start = 1;

        // Try to increment start until we win and stop loop once we do
        loop {
            let mut boat = Boat::default();

            boat.hold_button_for(start);
            boat.travel_for(self.time - start);

            if self.has_won(&boat) {
                break;
            }

            start += 1;
        }

        // Get the absolute difference between the start and the (max_time - start) as this + 1 would result
        // in the maximum amount of time we can hold the button, and still win.
        start.abs_diff(self.time - start) + 1
    }
}

/// A boat for racing
#[derive(Default)]
pub struct Boat {
    speed: u64,
    distance_traveled: u64,
}

impl Boat {
    /// Hold the button for a certain amount of time
    #[inline(always)]
    pub fn hold_button_for(&mut self, millis: u64) {
        self.speed += millis
    }

    /// Travel for a certain amount of time
    #[inline(always)]
    pub fn travel_for(&mut self, millis: u64) {
        self.distance_traveled = self.speed * millis
    }
}

/// A collection of Races
pub struct Races {
    races: Vec<Race>,
}

impl FromStr for Races {
    type Err = ParseRaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .map(|line| {
                line.split(':')
                    .last()
                    .ok_or_else(|| ParseRaceError::Invalid("Failed to get numbers".to_string()))?
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u64>().map_err(ParseRaceError::ParseInt))
                    .collect::<Result<Vec<u64>, _>>()
            })
            .collect::<Result<Vec<Vec<u64>>, _>>()?;

        let races = numbers[0]
            .iter()
            .zip(numbers[1].iter())
            .map(|x| Race::new(*x.0, *x.1))
            .collect::<Vec<Race>>();

        Ok(Self { races })
    }
}

impl Races {
    /// Get the winning product of all Races
    #[inline(always)]
    pub fn get_winning_product(&self) -> u64 {
        self.races.iter().map(Race::find_winning_conditions_amount).product()
    }

    /// Combine Races into a single Race
    #[inline(always)]
    pub fn as_single_race(self) -> Result<Self, ParseRaceError> {
        let (time, distance) = self
            .races
            .into_iter()
            .fold((String::new(), String::new()), |(time, distance), race| {
                (time + &race.time.to_string(), distance + &race.distance.to_string())
            });

        let race = Race::new(time.parse()?, distance.parse()?);

        Ok(Self { races: vec![race] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn solution_1() {
        let product = Races::from_str(EXAMPLE)
            .expect("Failed to parse races")
            .get_winning_product();
        assert_eq!(product, 288)
    }

    #[test]
    fn solution_2() {
        let product = Races::from_str(EXAMPLE)
            .expect("Failed to parse race")
            .as_single_race()
            .expect("Failed to combine races")
            .get_winning_product();
        assert_eq!(product, 71503);
    }
}
