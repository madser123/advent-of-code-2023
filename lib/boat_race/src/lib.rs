use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseRaceError {
    ParseInt(ParseIntError),
    Invalid(String),
}

pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub const fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    pub const fn has_won(&self, boat: &Boat) -> bool {
        boat.distance_traveled > self.distance
    }

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

#[derive(Default)]
pub struct Boat {
    speed: u64,
    distance_traveled: u64,
}

impl Boat {
    pub fn hold_button_for(&mut self, millis: u64) {
        self.speed += millis
    }

    pub fn travel_for(&mut self, millis: u64) {
        self.distance_traveled = self.speed * millis
    }
}

pub struct Races {
    races: Vec<Race>,
}

impl Races {
    pub fn get_winning_product(&self) -> u64 {
        self.races.iter().map(Race::find_winning_conditions_amount).product()
    }

    pub fn from_single_race(races: &str) -> Result<Self, ParseRaceError> {
        let numbers = races
            .lines()
            .map(|line| {
                line.split(':')
                    .last()
                    .ok_or_else(|| ParseRaceError::Invalid("Failed to get numbers".to_string()))?
                    .split_ascii_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .map_err(ParseRaceError::ParseInt)
            })
            .collect::<Result<Vec<u64>, ParseRaceError>>()?;

        let races = vec![Race::new(numbers[0], numbers[1])];

        Ok(Self { races })
    }

    pub fn from_multiple_races(races: &str) -> Result<Self, ParseRaceError> {
        let numbers = races
            .lines()
            .map(|line| {
                line.split(':')
                    .last()
                    .ok_or_else(|| ParseRaceError::Invalid("Failed to get numbers".to_string()))?
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u64>().map_err(ParseRaceError::ParseInt))
                    .collect::<Result<Vec<u64>, ParseRaceError>>()
            })
            .collect::<Result<Vec<Vec<u64>>, ParseRaceError>>()?;

        let races = numbers[0]
            .iter()
            .zip(numbers[1].iter())
            .map(|x| Race::new(*x.0, *x.1))
            .collect::<Vec<Race>>();

        Ok(Self { races })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn solution_1() {
        let product = Races::from_multiple_races(EXAMPLE)
            .expect("Failed to parse races")
            .get_winning_product();
        assert_eq!(product, 288)
    }

    #[test]
    fn solution_2() {
        let product = Races::from_single_race(EXAMPLE)
            .expect("Failed to parse race")
            .get_winning_product();
        assert_eq!(product, 71503);
    }
}
