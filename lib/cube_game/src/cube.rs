use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// A set of Cubes.
#[derive(Default)]
pub struct Cubes(HashMap<Color, i32>);

impl Cubes {
    /// Creates a new empty set of cubes.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the "power" of the cubes.
    ///
    /// This means all the cube-counts, multiplied with each other.
    #[inline(always)]
    pub fn power(&self) -> i32 {
        // Multiply all values in the limits
        self.values().product()
    }
}

// Deref traits for Cubes so we can use hashmap methods.
impl Deref for Cubes {
    type Target = HashMap<Color, i32>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Cubes {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Color parsing error
#[derive(Debug)]
pub struct ColorError(String);

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown color: {}", self.0)
    }
}

impl std::error::Error for ColorError {}

/// A color of a cube
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            // Invalid color, so we return an error
            s => Err(ColorError(s.to_string())),
        }
    }
}
