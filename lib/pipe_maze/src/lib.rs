use std::str::FromStr;

#[derive(Debug)]
pub enum MazeError {
    InvalidTile(char),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn transform(&mut self, direction: &Direction) -> Option<()> {
        match direction {
            Direction::North => self.y = self.y.checked_sub(1)?,
            Direction::East => self.x = self.x.checked_add(1)?,
            Direction::South => self.y = self.y.checked_add(1)?,
            Direction::West => self.x = self.x.checked_sub(1)?,
        };
        Some(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn is_horizontal(&self) -> bool {
        *self == Self::East || *self == Self::West
    }

    pub fn is_vertical(&self) -> bool {
        *self == Self::North || *self == Self::South
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl TryFrom<char> for Pipe {
    type Error = MazeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let pipe = match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            other => return Err(MazeError::InvalidTile(other)),
        };

        Ok(pipe)
    }
}

impl Pipe {
    pub const fn redirect(&self, from: Direction) -> Option<Direction> {
        match from {
            Direction::South => match self {
                Self::Vertical => Some(from),
                Self::NorthEast => Some(Direction::East),
                Self::NorthWest => Some(Direction::West),
                _ => None,
            },
            Direction::West => match self {
                Self::Horizontal => Some(from),
                Self::NorthEast => Some(Direction::North),
                Self::SouthEast => Some(Direction::South),
                _ => None,
            },
            Direction::North => match self {
                Self::Vertical => Some(from),
                Self::SouthEast => Some(Direction::East),
                Self::SouthWest => Some(Direction::West),
                _ => None,
            },
            Direction::East => match self {
                Self::Horizontal => Some(from),
                Self::NorthWest => Some(Direction::North),
                Self::SouthWest => Some(Direction::South),
                _ => None,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = MazeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile = match value {
            '.' => Self::Ground,
            'S' => Self::Start,
            other => Self::Pipe(Pipe::try_from(other)?),
        };

        Ok(tile)
    }
}

pub struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Maze {
    type Err = MazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();

        let tiles = lines
            .map(|l| l.chars().map(Tile::try_from).collect::<Result<Vec<Tile>, _>>())
            .collect::<Result<Vec<Vec<Tile>>, _>>()?;

        Ok(Self { tiles })
    }
}

impl Maze {
    fn find_starting_position(&self) -> Coordinate {
        let Some((x, y)) = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, row)| row.contains(&Tile::Start))
            .and_then(|(x, start_row)| start_row.iter().position(|tile| *tile == Tile::Start).zip(Some(x)))
        else {
            unreachable!("Maze always contains a starting position")
        };

        Coordinate::new(x, y)
    }

    fn get_tile(&self, coord: &Coordinate) -> &Tile {
        &self.tiles[coord.y][coord.x]
    }

    pub fn find_longest_distance_from_start(&self) -> i32 {
        let start = &self.find_starting_position();

        for mut direction in Direction::ALL {
            // Copy starting position
            let mut coord = *start;

            let mut traversed: i32 = 0;

            while let Some(pipe) = self.get_next_pipe(&mut coord, &direction) {
                if let Some(dir) = pipe.redirect(direction) {
                    traversed += 1;
                    direction = dir;
                } else {
                    break;
                }
            }

            if coord == *start && traversed > 0 {
                let div = traversed / 2;

                if traversed % 2 > 0 {
                    return div + 1;
                }

                return div;
            }
        }

        unreachable!("We always have a path")
    }

    fn get_next_pipe(&self, coord: &mut Coordinate, direction: &Direction) -> Option<&Pipe> {
        coord.transform(direction);

        if let Tile::Pipe(pipe) = self.get_tile(coord) {
            return Some(pipe);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARSING_EXAMPLE: &str = ".....
.F-7.
.|.|.
.L-J.
.....";

    const SIMPLE_EXAMPLE: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE_1: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_parsing() {
        let maze = Maze::from_str(PARSING_EXAMPLE).expect("Failed to parse");

        assert_eq!(
            maze.tiles,
            vec![
                vec![Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
                vec![
                    Tile::Ground,
                    Tile::Pipe(Pipe::SouthEast),
                    Tile::Pipe(Pipe::Horizontal),
                    Tile::Pipe(Pipe::SouthWest),
                    Tile::Ground
                ],
                vec![
                    Tile::Ground,
                    Tile::Pipe(Pipe::Vertical),
                    Tile::Ground,
                    Tile::Pipe(Pipe::Vertical),
                    Tile::Ground
                ],
                vec![
                    Tile::Ground,
                    Tile::Pipe(Pipe::NorthEast),
                    Tile::Pipe(Pipe::Horizontal),
                    Tile::Pipe(Pipe::NorthWest),
                    Tile::Ground
                ],
                vec![Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground, Tile::Ground],
            ]
        )
    }

    #[test]
    fn test_simple_example() {
        let maze = Maze::from_str(SIMPLE_EXAMPLE).expect("Failed to parse");

        assert_eq!(maze.find_longest_distance_from_start(), 4)
    }

    #[test]
    fn solution_1() {
        let maze = Maze::from_str(EXAMPLE_1).expect("Failed to parse");

        assert_eq!(maze.find_longest_distance_from_start(), 8)
    }
}
