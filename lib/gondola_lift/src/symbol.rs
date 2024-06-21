use crate::SchematicCoordinate;

#[derive(Debug)]
pub struct Symbol {
    coord: SchematicCoordinate,
    is_gear: bool,
}

impl Symbol {
    pub const fn new(coord: SchematicCoordinate, is_gear: bool) -> Self {
        Self { coord, is_gear }
    }

    pub const fn coord(&self) -> &SchematicCoordinate {
        &self.coord
    }

    pub const fn is_gear(&self) -> bool {
        self.is_gear
    }
}
