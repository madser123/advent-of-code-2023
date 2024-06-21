use std::ops::{Deref, DerefMut};

use crate::SchematicCoordinate;

#[derive(Debug)]
pub struct Parts(Vec<Part>);

impl Parts {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn sum(&self) -> i32 {
        self.0.iter().map(|part| part.number).sum()
    }
}

impl FromIterator<Part> for Parts {
    fn from_iter<T: IntoIterator<Item = Part>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl<'i> FromIterator<&'i Part> for Parts {
    fn from_iter<T: IntoIterator<Item = &'i Part>>(iter: T) -> Self {
        Self(iter.into_iter().copied().collect())
    }
}

impl Deref for Parts {
    type Target = Vec<Part>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Parts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Part {
    start: SchematicCoordinate,
    end: SchematicCoordinate,
    number: i32,
}

impl Part {
    pub const fn new(number: i32, start: SchematicCoordinate, end: SchematicCoordinate) -> Self {
        Self { start, end, number }
    }

    pub fn adjacent_to(&self, column: &i32) -> bool {
        let range = self.start.column..=self.end.column;
        range.contains(&(column - 1)) || range.contains(column) || range.contains(&(column + 1))
    }

    pub const fn number(&self) -> &i32 {
        &self.number
    }
}
