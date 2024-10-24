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

    pub const fn add_row(&self, row: usize) -> Self {
        Self {
            row: self.row + row,
            col: self.col,
        }
    }

    pub const fn add_col(&self, col: usize) -> Self {
        Self {
            row: self.row,
            col: self.col + col,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pixel {
    Empty(usize),
    Galaxy,
}

impl Pixel {
    pub const fn get_length(&self) -> usize {
        match self {
            Self::Empty(n) => *n,
            Self::Galaxy => 1,
        }
    }
}

impl TryFrom<char> for Pixel {
    type Error = ObservatoryError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let pixel = match value {
            '.' => Self::Empty(1),
            '#' => Self::Galaxy,
            invalid => return Err(ObservatoryError::ParsePixel(invalid)),
        };

        Ok(pixel)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pixel = match self {
            Self::Empty(n) => format!("_.x{}_", n),
            Self::Galaxy => "#".to_string(),
        };
        write!(f, "{pixel}")
    }
}

#[derive(Clone)]
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

    pub fn expand(&mut self, age: usize) {
        let mut row = 0;
        while row < self.height() {
            if !self.row_contains_galaxy(row) {
                self.data
                    .get_mut(row)
                    .expect("Failed to get row")
                    .iter_mut()
                    .for_each(|pixel| {
                        *pixel = Pixel::Empty(age);
                    });
            }

            row += 1;
        }

        let mut col = 0;
        while col < self.width() {
            if !self.column_contains_galaxy(col) {
                self.data.iter_mut().for_each(|row| {
                    row[col] = Pixel::Empty(age);
                });
            }
            col += 1;
        }
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
            .flat_map(|(r, row_pixels)| {
                row_pixels
                    .iter()
                    .enumerate()
                    .filter_map(|(c, pixel)| {
                        if pixel == &Pixel::Galaxy {
                            let coord = self.data[0..=r].iter().enumerate().fold(
                                Coordinate::new(0, 0),
                                |mut acc, (row, col_data)| {
                                    if row == r {
                                        acc = acc.add_col(col_data[0..c].iter().map(|p| p.get_length()).sum());
                                    } else {
                                        acc = acc.add_row(col_data[c].get_length());
                                    };

                                    acc
                                },
                            );

                            Some(coord)
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
    fn test_10x_expansion() {
        let mut image = Image::from_str(EXAMPLE).expect("Failed to parse image");

        image.expand(10);

        assert_eq!(1030, image.find_shortest_paths_sum());
    }

    #[test]
    fn test_100x_expansion() {
        let mut image = Image::from_str(EXAMPLE).expect("Failed to parse image");

        image.expand(100);

        assert_eq!(8410, image.find_shortest_paths_sum());
    }

    #[test]
    fn solution_1() {
        let mut image = Image::from_str(EXAMPLE).expect("Failed to parse image");

        image.expand(2);

        assert_eq!(374, image.find_shortest_paths_sum());
    }
}
