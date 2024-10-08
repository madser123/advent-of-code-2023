use crate::SchematicCoordinate;
use std::ops::{Deref, DerefMut};

/// A collection of parts
#[derive(Debug, Default)]
pub struct Parts(Vec<Part>);

impl Parts {
    /// Get the sum of all part numbers
    #[inline(always)]
    pub fn sum(&self) -> i32 {
        self.0.iter().map(|part| part.number).sum()
    }
}

impl FromIterator<Part> for Parts {
    #[inline(always)]
    fn from_iter<T: IntoIterator<Item = Part>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl<'i> FromIterator<&'i Part> for Parts {
    #[inline(always)]
    fn from_iter<T: IntoIterator<Item = &'i Part>>(iter: T) -> Self {
        Self(iter.into_iter().copied().collect())
    }
}

impl Deref for Parts {
    type Target = Vec<Part>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Parts {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A part in a schematic
#[derive(Clone, Copy, Debug)]
pub struct Part {
    start: SchematicCoordinate,
    end: SchematicCoordinate,
    number: i32,
}

impl Part {
    /// Create a new part
    #[inline(always)]
    pub const fn new(number: i32, start: SchematicCoordinate, end: SchematicCoordinate) -> Self {
        Self { start, end, number }
    }

    /// Check if the part is adjacent to a column
    #[inline(always)]
    pub fn adjacent_to(&self, column: &i32) -> bool {
        let range = self.start.column..=self.end.column;
        range.contains(&(column - 1)) || range.contains(column) || range.contains(&(column + 1))
    }

    /// Get the number of the part
    #[inline(always)]
    pub const fn number(&self) -> &i32 {
        &self.number
    }
}
