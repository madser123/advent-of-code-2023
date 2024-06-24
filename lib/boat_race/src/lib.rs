use std::{
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

pub struct Race {
    time: i32,
    distance: i32,
}

impl Race {
    pub const fn new(time: i32, distance: i32) -> Self {
        Self { time, distance }
    }

    pub const fn has_won(&self, boat: &Boat) -> bool {
        boat.distance_traveled > self.distance
    }

    pub fn find_winning_conditions(&self) -> Vec<i32> {
        // We are not interested in holding the button for 0 ms,
        // as this would result in 0 distance
        let range = 1..self.time;

        range
            .filter(|hold_ms| {
                let mut boat = Boat::default();

                boat.hold_button_for(*hold_ms);
                boat.travel_for(self.time - hold_ms);

                self.has_won(&boat)
            })
            .collect()
    }
}

#[derive(Default)]
pub struct Boat {
    speed: i32,
    distance_traveled: i32,
}

impl Boat {
    pub fn hold_button_for(&mut self, millis: i32) {
        self.speed += millis
    }

    pub fn travel_for(&mut self, millis: i32) {
        self.distance_traveled = self.speed * millis
    }
}

pub struct Races {
    races: Vec<Race>,
}

impl Races {
    pub fn get_winning_product(&self) -> usize {
        self.races
            .iter()
            .map(|race| race.find_winning_conditions().len())
            .product()
    }
}

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .map(|line| {
                line.split(':')
                    .last()
                    .expect("Failed to get numbers")
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i32>())
                    .collect::<Result<Vec<i32>, ParseIntError>>()
                    .expect("Failed to parse numbers")
            })
            .collect::<Vec<Vec<i32>>>();

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
    fn it_works() {
        let product = Races::from_str(EXAMPLE)
            .expect("Failed to parse race")
            .get_winning_product();
        assert_eq!(product, 288)
    }
}
