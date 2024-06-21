use crate::cube::{Color, ColorError, Limits};

use super::cube::Cubes;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum RoundError {
    NoId,
    ParseInt(ParseIntError),
    ParseColor(ColorError),
}

impl From<ParseIntError> for RoundError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<ColorError> for RoundError {
    fn from(value: ColorError) -> Self {
        Self::ParseColor(value)
    }
}

pub struct Draw {
    cubes: Vec<Cubes>,
}

impl FromStr for Draw {
    type Err = RoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cubes = s
            // Remove all commas
            .replace(',', "")
            // Split the str
            .split(' ')
            // Remove empty strings
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            // Create iterator of chunks with (<count>, <color>)
            .chunks(2)
            // Create cubes from chunks
            .map(|chunk| {
                let count = chunk[0].parse::<i32>()?;
                let color = Color::from_str(chunk[1])?;

                Ok(Cubes::new(count, color))
            })
            .collect::<Result<Vec<Cubes>, RoundError>>()?;

        Ok(Self { cubes })
    }
}

pub struct Round {
    id: i32,
    draws: Vec<Draw>,
}

impl Round {
    pub fn is_valid(&self, limits: &Limits) -> bool {
        // Iterate over all draws
        self.draws.iter().all(|draw| {
            // Iterate over the cubes in that draw
            draw.cubes.iter().all(|cubes| {
                // Grab the limit for the cube's color
                limits
                    .get(cubes.color())
                    // Check if the amount of cubes is within the limit
                    .map(|limit| cubes.count() <= limit)
                    // If the color is non-existant in the limits, return true, as it then has no limit.
                    .unwrap_or(true)
            })
        })
    }

    pub fn get_minimum_set(&self) -> Limits {
        // Create set
        let mut minimum_set = Limits::new();

        // Iterate over draws
        self.draws.iter().for_each(|draw| {
            // Iterate over cubes
            draw.cubes.iter().for_each(|cubes| {
                // Get the count of current cubes
                let cube_count = *cubes.count();
                // Get the color from set, if its already there
                minimum_set
                    .entry(*cubes.color())
                    // Check if the current count is lower than the cubes-count
                    .and_modify(|count| {
                        if *count < cube_count {
                            *count = cube_count
                        }
                    })
                    // Or insert the cubes-count if no entry was found
                    .or_insert(*cubes.count());
            })
        });

        minimum_set
    }

    pub const fn id(&self) -> &i32 {
        &self.id
    }
}

impl FromStr for Round {
    type Err = RoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string to "Game <id>" | "<draws>"
        let split = s.split(':').collect::<Vec<_>>();

        // Try to get the id from the last part of the first part of the split
        let id = if let Some(id_str) = split[0].split(' ').last() {
            // The id exists where we expect, so parse it to i32
            id_str.parse::<i32>()?
        } else {
            // Return an error,
            return Err(RoundError::NoId);
        };

        let draws = split[1]
            // Split the draw into seperate draws
            .split(';')
            // Parse each draw
            .map(Draw::from_str)
            // Handle result
            .collect::<Result<Vec<_>, RoundError>>()?;

        Ok(Self { id, draws })
    }
}
