use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum MazeError {
    InvalidTile(char),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub const fn new(x: i32, y: i32) -> Self {
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
    const ALL: [Self; 4] = [Self::North, Self::West, Self::South, Self::East];

    pub fn is_horizontal(&self) -> bool {
        *self == Self::East || *self == Self::West
    }

    pub fn is_vertical(&self) -> bool {
        *self == Self::North || *self == Self::South
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        use Direction::*;

        match from {
            Direction::South => match self {
                Self::Vertical => Some(from),
                Self::NorthEast => Some(East),
                Self::NorthWest => Some(West),
                _ => None,
            },
            Direction::West => match self {
                Self::Horizontal => Some(from),
                Self::NorthEast => Some(North),
                Self::SouthEast => Some(South),
                _ => None,
            },
            Direction::North => match self {
                Self::Vertical => Some(from),
                Self::SouthEast => Some(East),
                Self::SouthWest => Some(West),
                _ => None,
            },
            Direction::East => match self {
                Self::Horizontal => Some(from),
                Self::NorthWest => Some(North),
                Self::SouthWest => Some(South),
                _ => None,
            },
        }
    }

    pub const fn find_connector_between(direction: &Direction, end_at: &Direction) -> Option<Self> {
        use Direction::*;

        match (direction, end_at) {
            (North, North) | (South, South) => Some(Self::Vertical),
            (East, East) | (West, West) => Some(Self::Horizontal),
            (North, West) | (East, South) => Some(Self::SouthWest),
            (North, East) | (West, South) => Some(Self::SouthEast),
            (South, West) | (East, North) => Some(Self::NorthWest),
            (South, East) | (West, North) => Some(Self::NorthEast),
            _ => None,
        }
    }

    pub const fn is_horizontal_neighbour(&self, other: &Self) -> bool {
        match self {
            Self::Vertical => false,
            _ => !matches!(other, Self::Vertical),
        }
    }

    pub const fn is_corner(&self) -> bool {
        matches!(
            self,
            Self::NorthEast | Self::NorthWest | Self::SouthEast | Self::SouthWest
        )
    }

    pub const fn is_connected_corner(&self, other: &Self) -> bool {
        match self {
            Self::NorthEast => matches!(other, Self::SouthWest | Self::NorthWest),
            Self::NorthWest => matches!(other, Self::SouthEast | Self::NorthEast),
            Self::SouthEast => matches!(other, Self::NorthWest | Self::SouthWest),
            Self::SouthWest => matches!(other, Self::NorthEast | Self::SouthEast),
            _ => false,
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
    tiles: HashMap<Coordinate, Tile>,
}

impl FromStr for Maze {
    type Err = MazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s.lines().enumerate().try_fold(HashMap::new(), |mut map, (row, line)| {
            line.chars().enumerate().try_for_each(|(col, tile)| {
                let tile = Tile::try_from(tile)?;
                map.insert(Coordinate::new(col as i32, row as i32), tile);
                Ok(())
            })?;
            Ok(map)
        })?;

        Ok(Self { tiles })
    }
}

impl Maze {
    fn picks(area: i32, boundary: i32) -> i32 {
        println!("{area} + 1 - {boundary} / 2");
        area + 1 - boundary / 2
    }

    fn shoestring(pipes: Vec<Coordinate>) -> i32 {
        pipes.windows(2).fold(0, |acc, coords| {
            acc + (coords[0].y * coords[1].x) - (coords[1].y * coords[0].x)
        }) / 2
    }

    fn find_starting_position(&self) -> Coordinate {
        let Some((pos, _)) = self.tiles.iter().find(|(_, tile)| **tile == Tile::Start) else {
            unreachable!("Maze always contains a starting position")
        };

        *pos
    }

    fn get_tile(&self, coord: &Coordinate) -> Option<&Tile> {
        self.tiles.get(coord)
    }

    pub fn find_loop(&self) -> Vec<Coordinate> {
        let start = &self.find_starting_position();

        for mut direction in Direction::ALL {
            // Copy starting position
            let mut coord = *start;

            let mut traversed = vec![*start];

            while let Some(pipe) = self.get_next_pipe(&mut coord, &direction) {
                if let Some(dir) = pipe.redirect(direction) {
                    traversed.push(coord);
                    direction = dir;
                } else {
                    break;
                }
            }

            if coord == *start && traversed.len() != 1 {
                return traversed;
            }
        }

        unreachable!("We always have a path")
    }

    pub fn find_nest_area(&self) -> i32 {
        let pipes = self.find_loop();
        let boundary = (pipes.len()) as i32;
        let area = Self::shoestring(pipes).abs();
        Self::picks(area, boundary)
    }

    pub fn find_farthest_point_from_start(&self) -> usize {
        let pipes = self.find_loop();

        let len = pipes.len();
        len / 2
    }

    fn get_next_pipe(&self, coord: &mut Coordinate, direction: &Direction) -> Option<&Pipe> {
        coord.transform(direction);

        if let Tile::Pipe(pipe) = self.get_tile(coord)? {
            return Some(pipe);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    const EXAMPLE_2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE_3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EXAMPLE_4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_loop_simple_example() {
        let maze = Maze::from_str(SIMPLE_EXAMPLE).expect("Failed to parse");

        assert_eq!(maze.find_farthest_point_from_start(), 4)
    }

    #[test]
    fn solution_1() {
        let maze = Maze::from_str(EXAMPLE_1).expect("Failed to parse");

        assert_eq!(maze.find_farthest_point_from_start(), 8)
    }

    #[test]
    fn test_nest_simple_example() {
        let maze = Maze::from_str(SIMPLE_EXAMPLE).expect("Failed to parse");

        assert_eq!(maze.find_nest_area(), 1)
    }

    #[test]
    fn solution_2_example_1() {
        let maze_1 = Maze::from_str(EXAMPLE_2).expect("Failed to parse");

        assert_eq!(maze_1.find_nest_area(), 4);
    }

    #[test]
    fn solution_2_example_2() {
        let maze_2 = Maze::from_str(EXAMPLE_3).expect("Failed to parse");

        assert_eq!(maze_2.find_nest_area(), 8);
    }

    #[test]
    fn solution_2_example_3() {
        let maze_3 = Maze::from_str(EXAMPLE_4).expect("Failed to parse");

        assert_eq!(maze_3.find_nest_area(), 10);
    }
}
