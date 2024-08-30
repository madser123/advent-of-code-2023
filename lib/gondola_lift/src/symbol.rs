use crate::SchematicCoordinate;

/// A symbol in a schematic
#[derive(Debug)]
pub struct Symbol {
    coord: SchematicCoordinate,
    is_gear: bool,
}

impl Symbol {
    /// Create a new symbol
    #[inline(always)]
    pub const fn new(coord: SchematicCoordinate, is_gear: bool) -> Self {
        Self { coord, is_gear }
    }

    /// Get the coordinate of the symbol
    #[inline(always)]
    pub const fn coord(&self) -> &SchematicCoordinate {
        &self.coord
    }

    /// Check if the symbol is a gear
    #[inline(always)]
    pub const fn is_gear(&self) -> bool {
        self.is_gear
    }
}
