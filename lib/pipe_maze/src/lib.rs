use std::str::FromStr;

/// Error type for parsing the maze
#[derive(Debug)]
pub enum MazeError {
    InvalidTile(char),
    Empty,
}

impl std::error::Error for MazeError {}

impl std::fmt::Display for MazeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTile(tile) => write!(f, "Invalid tile: {}", tile),
            Self::Empty => write!(f, "Maze is empty"),
        }
    }
}

/// A coordinate in the maze
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    /// Create a new coordinate
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Transform the coordinate in a direction
    #[inline(always)]
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

/// A direction in the maze
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// All directions
    const ALL: [Self; 4] = [Self::North, Self::West, Self::South, Self::East];
}

/// A pipe in the maze
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
    /// Calculate the next direction to go in, based on the current direction and the pipe
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
}

/// A tile in the maze
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

/// A maze
pub struct Maze {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Maze {
    type Err = MazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>, _>>()
            })
            .collect::<Result<Vec<Vec<Tile>>, _>>()?;

        Ok(Self { tiles })
    }
}

impl Maze {
    /// Calculate picks theorem
    #[inline(always)]
    const fn picks(area: usize, boundary: usize) -> usize {
        area - (boundary / 2) + 1
    }

    /// Calculate the area of a polygon using the shoelace formula
    #[inline(always)]
    fn shoelace(coordinates: Vec<Coordinate>) -> usize {
        (coordinates
            .windows(2)
            .fold(0, |acc, coords| {
                acc + (coords[0].y as i32 * coords[1].x as i32) - (coords[1].y as i32 * coords[0].x as i32)
            })
            .abs()
            / 2) as usize
    }

    /// Find the starting position in the maze
    #[inline(always)]
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

    /// Get the tile at a coordinate
    #[inline(always)]
    fn get_tile(&self, coord: &Coordinate) -> &Tile {
        &self.tiles[coord.y][coord.x]
    }

    /// Find the loop in the maze
    pub fn find_loop(&self) -> Vec<Coordinate> {
        let start = &self.find_starting_position();

        for mut direction in Direction::ALL {
            // Copy starting position
            let mut coord = *start;

            // Push start to the beginning
            let mut traversed = vec![*start];

            // Traverse the loop to completion, if the direction is correct
            while let Some(pipe) = self.get_next_pipe(&mut coord, &direction) {
                if let Some(dir) = pipe.redirect(direction) {
                    traversed.push(coord);
                    direction = dir;
                } else {
                    break;
                }
            }

            // Make sure we actually traversed the loop
            if coord == *start && traversed.len() > 1 {
                // Push start to the end, so we can use it in the shoelace formula
                traversed.push(*start);
                return traversed;
            }
        }

        unreachable!("We always have a path")
    }

    /// Find the area of the nest
    #[inline(always)]
    pub fn find_nest_area(&self) -> usize {
        let pipes = self.find_loop();
        let boundary = pipes.len();
        let area = Self::shoelace(pipes);
        Self::picks(area, boundary)
    }

    /// Find the farthest point from the start
    #[inline(always)]
    pub fn find_farthest_point_from_start(&self) -> usize {
        let pipes = self.find_loop();
        let len = pipes.len();
        len / 2
    }

    /// Get the next pipe in a direction
    #[inline(always)]
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
    fn solution_2_example_2() {
        let maze_1 = Maze::from_str(EXAMPLE_2).expect("Failed to parse");

        assert_eq!(maze_1.find_nest_area(), 4);
    }

    #[test]
    fn solution_2_example_3() {
        let maze_2 = Maze::from_str(EXAMPLE_3).expect("Failed to parse");

        assert_eq!(maze_2.find_nest_area(), 8);
    }

    #[test]
    fn solution_2_example_4() {
        let maze_3 = Maze::from_str(EXAMPLE_4).expect("Failed to parse");

        assert_eq!(maze_3.find_nest_area(), 10);
    }
}
