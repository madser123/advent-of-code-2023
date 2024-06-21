#![allow(clippy::missing_panics_doc, missing_docs, clippy::missing_errors_doc)]

use cube_game::{cube::Color, Cubes, Game};
use gondola_lift::EngineSchematic;
use trebuchet::Trebuchet;

fn day_1() {
    // read and parse input.txt
    let input: String =
        std::fs::read_to_string("input_day1.txt").expect("Couldn't read input-file");

    let result = Trebuchet::new(&input)
        .expect("Failed to create trebuchet")
        .get_calibration_sum();

    println!("Result: {result}");
}

fn day_2() {
    let input = std::fs::read_to_string("input_day2.txt").expect("Couldn't read input file");
    let mut limits = Cubes::new();
    limits.insert(Color::Red, 12);
    limits.insert(Color::Green, 13);
    limits.insert(Color::Blue, 14);

    let result = Game::new(&input, limits)
        .expect("Failed to create game")
        .get_minimum_powers_sum();
    println!("Result: {result}")
}

fn day_3() {
    let input = std::fs::read_to_string("input_day3.txt").expect("Couldn't read input file");
    let schem = EngineSchematic::new(&input).expect("Failed to create schematic");
    let parts_sum = schem.get_parts().sum();
    println!("Part-numbers sum: {parts_sum}");
    let gear_ratio = schem.get_gear_ratio();
    println!("Gear ration: {gear_ratio}");
}

fn main() {
    day_3();
}
