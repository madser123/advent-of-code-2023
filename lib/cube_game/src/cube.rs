use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Default)]
pub struct Limits(HashMap<Color, i32>);

impl Limits {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for Limits {
    type Target = HashMap<Color, i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Limits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct ColorError(String);

#[derive(PartialEq, Eq, Hash)]
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

pub struct Cubes {
    color: Color,
    count: i32,
}

impl Cubes {
    pub const fn new(count: i32, color: Color) -> Self {
        Self { color, count }
    }

    pub const fn color(&self) -> &Color {
        &self.color
    }

    pub const fn count(&self) -> &i32 {
        &self.count
    }
}
