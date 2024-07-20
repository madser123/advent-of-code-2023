use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum ObservatoryError {
    ParsePixel(char),
}

#[derive(Debug)]
pub struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub const fn shortest_path_to(&self, other: &Self) -> usize {
        self.col.abs_diff(other.col) + self.row.abs_diff(other.row)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pixel {
    Empty,
    Galaxy,
}

impl TryFrom<char> for Pixel {
    type Error = ObservatoryError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let pixel = match value {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            invalid => return Err(ObservatoryError::ParsePixel(invalid)),
        };

        Ok(pixel)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pixel = match self {
            Self::Empty => '.',
            Self::Galaxy => '#',
        };
        write!(f, "{pixel}")
    }
}

pub struct Image {
    data: Vec<Vec<Pixel>>,
}

impl FromStr for Image {
    type Err = ObservatoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|row| row.chars().map(Pixel::try_from).collect::<Result<Vec<Pixel>, _>>())
            .collect::<Result<Vec<Vec<Pixel>>, _>>()?;

        Ok(Self { data })
    }
}

impl Image {
    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    pub fn expand(&mut self) {
        let mut row = 0;
        while row < self.height() {
            if !self.row_contains_galaxy(row) {
                self.add_empty_row(row);
                row += 2;
            } else {
                row += 1;
            }
        }

        let mut col = 0;
        while col < self.width() {
            if !self.column_contains_galaxy(col) {
                self.add_empty_column(col);
                col += 2;
            } else {
                col += 1;
            }
        }
    }

    fn add_empty_row(&mut self, at_row: usize) {
        self.data
            .insert(at_row, (0..self.width()).map(|_| Pixel::Empty).collect());
    }

    fn add_empty_column(&mut self, at_col: usize) {
        self.data.iter_mut().for_each(|row| row.insert(at_col, Pixel::Empty));
    }

    fn row_contains_galaxy(&self, row: usize) -> bool {
        self.data[row].contains(&Pixel::Galaxy)
    }

    fn column_contains_galaxy(&self, col: usize) -> bool {
        (0..self.height()).any(|row| {
            if let Some(pixel) = self.data[row].get(col) {
                return pixel == &Pixel::Galaxy;
            }
            false
        })
    }

    fn find_galaxies(&self) -> Vec<Coordinate> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(row, row_pixels)| {
                row_pixels
                    .iter()
                    .enumerate()
                    .filter_map(|(col, pixel)| {
                        if pixel == &Pixel::Galaxy {
                            Some(Coordinate::new(row, col))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Coordinate>>()
            })
            .collect()
    }

    pub fn find_shortest_paths_sum(&self) -> usize {
        let galaxies = self.find_galaxies();

        galaxies
            .iter()
            .enumerate()
            .map(|(index, coord)| {
                let mut sum = 0;
                (index..galaxies.len()).for_each(|other| sum += coord.shortest_path_to(&galaxies[other]));
                sum
            })
            .sum()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let image = self
            .data
            .iter()
            .map(|row| row.iter().map(|col| col.to_string()).collect())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{image}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_expansion() {
        let mut image = Image::from_str(EXAMPLE).expect("Failed to parse image");

        image.expand();

        assert_eq!((image.height(), image.width()), (12, 13));
    }

    #[test]
    fn solution_1() {
        let mut image = Image::from_str(EXAMPLE).expect("Failed to parse image");

        image.expand();

        assert_eq!(374, image.find_shortest_paths_sum());
    }
}
