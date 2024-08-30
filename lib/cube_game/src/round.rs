use crate::cube::{Color, ColorError, Cubes};
use std::{num::ParseIntError, str::FromStr};

/// Error type for parsing rounds
#[derive(Debug)]
pub enum RoundError {
    NoId,
    ParseInt(ParseIntError),
    ParseColor(ColorError),
    DuplicateCubes(Color),
}

impl std::error::Error for RoundError {}

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

impl std::fmt::Display for RoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateCubes(color) => write!(f, "Got duplicate cubes with {color:?}"),
            Self::NoId => write!(f, "Round id couldn't be determined"),
            Self::ParseColor(wrong_color) => write!(f, "Failed to parse color '{wrong_color}'"),
            Self::ParseInt(int_err) => write!(f, "Failed to parse integer {int_err:?}"),
        }
    }
}

/// A draw of cubes
pub struct Draw {
    cubes: Cubes,
}

impl FromStr for Draw {
    type Err = RoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Create cubes
        let mut cubes = Cubes::new();

        s
            // Remove all commas
            .replace(',', "")
            // Split the str
            .split(' ')
            // Remove empty strings
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            // Create iterator of chunks with (<count>, <color>)
            .chunks(2)
            .try_for_each(|chunk| {
                let count = chunk[0].parse::<i32>()?;
                let color = Color::from_str(chunk[1])?;

                if cubes.insert(color, count).is_some() {
                    return Err(RoundError::DuplicateCubes(color));
                }

                Ok(())
            })?;

        Ok(Self { cubes })
    }
}

/// A round of the game
pub struct Round {
    id: i32,
    draws: Vec<Draw>,
}

impl Round {
    /// Checks if the round is valid
    #[inline(always)]
    pub fn is_valid(&self, limits: &Cubes) -> bool {
        // Iterate over all draws
        self.draws.iter().all(|draw| {
            // Iterate over the cubes in that draw
            draw.cubes.iter().all(|(color, count)| {
                // Grab the limit for the cube's color
                limits
                    .get(color)
                    // Check if the amount of cubes is within the limit
                    .map(|limit| count <= limit)
                    // If the color is non-existant in the limits, return true, as it then has no limit.
                    .unwrap_or(true)
            })
        })
    }

    /// Get the minimum set of cubes
    #[inline(always)]
    pub fn get_minimum_set(&self) -> Cubes {
        // Create set
        let mut minimum_set = Cubes::new();

        // Iterate over draws
        self.draws.iter().for_each(|draw| {
            // Iterate over cubes
            draw.cubes.iter().for_each(|(color, count)| {
                // Get the count of current cubes
                let cube_count = *count;
                // Get the color from set, if its already there
                minimum_set
                    .entry(*color)
                    // Check if the current count is lower than the cubes-count
                    .and_modify(|count| {
                        if *count < cube_count {
                            *count = cube_count
                        }
                    })
                    // Or insert the cubes-count if no entry was found
                    .or_insert(*count);
            })
        });

        minimum_set
    }

    /// Get the ID of the round
    #[inline(always)]
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
