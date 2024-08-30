use std::{
    collections::BTreeMap,
    num::{ParseIntError, TryFromIntError},
};

/// Part module
pub mod part;

/// Symbol module
pub mod symbol;

use part::{Part, Parts};
use symbol::Symbol;

/// Error for schematic parsing
#[derive(Debug)]
pub enum SchematicError {
    ParseUsize(TryFromIntError),
    ParseInt(ParseIntError),
    Empty,
}

impl std::error::Error for SchematicError {}

impl From<TryFromIntError> for SchematicError {
    fn from(value: TryFromIntError) -> Self {
        Self::ParseUsize(value)
    }
}

impl From<ParseIntError> for SchematicError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl std::fmt::Display for SchematicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Schematic is empty"),
            Self::ParseUsize(int_err) => write!(f, "Failed to parse usize: {int_err:?}"),
            Self::ParseInt(int_err) => write!(f, "Failed to parse integer: {int_err:?}"),
        }
    }
}

/// A coordinate in a schematic
#[derive(Clone, Copy, Debug)]
pub struct SchematicCoordinate {
    line: i32,
    column: i32,
}

impl SchematicCoordinate {
    /// Create a new schematic coordinate
    #[inline(always)]
    pub const fn new(line: i32, column: i32) -> Self {
        Self { line, column }
    }

    /// Get the line (row) of the coordinate
    #[inline(always)]
    pub const fn line(&self) -> &i32 {
        &self.line
    }

    /// Get the column of the coordinate
    #[inline(always)]
    pub const fn column(&self) -> &i32 {
        &self.column
    }
}

/// A schematic for an engine
#[derive(Debug)]
pub struct EngineSchematic {
    symbols: Vec<Symbol>,
    possible_parts: BTreeMap<i32, Parts>,
}

impl EngineSchematic {
    /// Create a new schematic
    pub fn new(schematic: &str) -> Result<Self, SchematicError> {
        let schematic: BTreeMap<i32, String> = (0i32..)
            // Enumerate all lines
            .zip(schematic.lines().map(str::to_string))
            // Collect as btreemap
            .collect();

        let symbols = schematic
            .iter()
            // Use map to find all symbols
            .flat_map(Self::find_symbols)
            .collect();

        let possible_parts = schematic
            .iter()
            // Use map to find all parts
            .map(Self::find_possible_parts)
            .collect::<Result<BTreeMap<i32, Parts>, SchematicError>>()?;

        Ok(Self {
            symbols,
            possible_parts,
        })
    }

    /// Get all parts in the schematic
    #[inline(always)]
    pub fn get_parts(&self) -> Parts {
        self.symbols
            .iter()
            .flat_map(|coord| self.get_parts_around_symbol(coord))
            .collect()
    }

    /// Get the gear ratio of the schematic
    #[inline(always)]
    pub fn get_gear_ratio(&self) -> i32 {
        self.symbols
            .iter()
            .filter(|s| s.is_gear())
            .filter_map(|s| {
                let parts = self.get_parts_around_symbol(s);

                if parts.len() == 2 {
                    return Some(parts.iter().map(|p| p.number()).product::<i32>());
                }

                None
            })
            .sum()
    }

    /// Get the parts around a symbol
    #[inline(always)]
    fn get_parts_around_symbol(&self, symbol: &Symbol) -> Vec<Part> {
        let mut adjacent_parts = Vec::new();

        // Loop from the line above to the line below
        for l in (symbol.coord().line().saturating_sub(1))..=(symbol.coord().line() + 1) {
            if let Some(parts) = self.possible_parts.get(&l) {
                let mut vec = parts
                    .iter()
                    .filter(|p| p.adjacent_to(symbol.coord().column()))
                    .copied()
                    .collect();
                adjacent_parts.append(&mut vec);
            }
        }

        adjacent_parts
    }

    /// Find symbols in a line
    #[inline(always)]
    fn find_symbols(line: (&i32, &String)) -> Vec<Symbol> {
        // Extract tuple values
        let (line, contents) = line;

        contents
            .chars()
            // Enumerate to get column
            .zip(0i32..)
            // Map and match
            .filter_map(|(c, column)| match c {
                '.' => None,
                _ if c.is_ascii_punctuation() => Some(Symbol::new(SchematicCoordinate::new(*line, column), '*' == c)),
                _ => None,
            })
            .collect::<Vec<_>>()
    }

    /// Find possible parts in a line
    #[inline(always)]
    fn find_possible_parts(line: (&i32, &String)) -> Result<(i32, Parts), SchematicError> {
        // Extract tuple values
        let (line, contents) = line;

        // Filter out all numeric characters and zip with column
        let number_chars = contents
            .chars()
            .zip(0i32..)
            .filter(|(c, _)| c.is_numeric())
            .collect::<Vec<_>>();

        let mut part_start = 0;

        let parts = (1..=number_chars.len())
            .flat_map(move |i| {
                // Get part groups by checking if next char is not directly next to the current column
                if i == number_chars.len() || number_chars[i - 1].1 + 1 != number_chars[i].1 {
                    let begin = part_start;
                    part_start = i;
                    // Take part group
                    Some(number_chars[begin..i].to_vec())
                } else {
                    None
                }
            })
            .map(|p| {
                // Construct part out of number vecs
                let number = p.iter().map(|(c, _)| c).collect::<String>().parse::<i32>()?;
                let columns = p.iter().map(|(_, column)| *column).collect::<Vec<i32>>();
                let start = SchematicCoordinate::new(*line, columns[0]);
                let end = SchematicCoordinate::new(*line, *columns.last().unwrap_or(&columns[0]));

                Ok(Part::new(number, start, end))
            })
            .collect::<Result<Parts, SchematicError>>()?;

        Ok((*line, parts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn it_works() {
        let schem = EngineSchematic::new(EXAMPLE).expect("Failed to create schematic");
        assert_eq!(schem.get_parts().sum(), 4361)
    }
}
