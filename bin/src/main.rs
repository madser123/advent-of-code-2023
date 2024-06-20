#![allow(clippy::missing_panics_doc, missing_docs, clippy::missing_errors_doc)]

use calibration_parser::parse_calibration_line;
use cube_game::{cube::Color, Game, Limits};

fn day_1() {
    // read and parse input.txt
    let result: i32 = std::fs::read_to_string("input_day1.txt")
        .expect("Couldn't read input-file")
        .lines()
        .map(parse_calibration_line)
        .sum();
    println!("Result: {result}");
}

fn day_2() {
    let input = std::fs::read_to_string("input_day2.txt").expect("Couldn't read input file");
    let mut limits = Limits::new();
    limits.insert(Color::Red, 12);
    limits.insert(Color::Green, 13);
    limits.insert(Color::Blue, 14);

    let result = Game::new(&input, limits)
        .expect("Failed to create game")
        .get_id_sums();
    println!("Result: {result}")
}

fn main() {
    day_2();
}
