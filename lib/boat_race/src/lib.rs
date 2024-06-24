use std::num::ParseIntError;

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
        let mut end = self.time;

        loop {
            let mut boat = Boat::default();

            boat.hold_button_for(start);
            boat.travel_for(self.time - start);

            if self.has_won(&boat) {
                break;
            }

            start += 1;
        }

        loop {
            let mut boat = Boat::default();

            boat.hold_button_for(end);
            boat.travel_for(self.time - end);

            if self.has_won(&boat) {
                break;
            }

            end -= 1;
        }

        start.abs_diff(end) + 1
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

    pub fn from_single_race(races: &str) -> Self {
        let numbers = races
            .lines()
            .map(|line| {
                line.split(':')
                    .last()
                    .expect("Failed to get numbers")
                    .split_ascii_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .expect("Failed to parse number")
            })
            .collect::<Vec<u64>>();

        let races = vec![Race::new(numbers[0], numbers[1])];

        Self { races }
    }

    pub fn from_multiple_races(races: &str) -> Self {
        let numbers = races
            .lines()
            .map(|line| {
                line.split(':')
                    .last()
                    .expect("Failed to get numbers")
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<u64>())
                    .collect::<Result<Vec<u64>, ParseIntError>>()
                    .expect("Failed to parse numbers")
            })
            .collect::<Vec<Vec<u64>>>();

        let races = numbers[0]
            .iter()
            .zip(numbers[1].iter())
            .map(|x| Race::new(*x.0, *x.1))
            .collect::<Vec<Race>>();

        Self { races }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn solution_1() {
        let product = Races::from_multiple_races(EXAMPLE).get_winning_product();
        assert_eq!(product, 288)
    }

    #[test]
    fn solution_2() {
        let product = Races::from_single_race(EXAMPLE).get_winning_product();
        assert_eq!(product, 71503);
    }
}
