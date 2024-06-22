#![allow(clippy::missing_panics_doc, missing_docs, clippy::missing_errors_doc)]

use std::str::FromStr;

use cube_game::{cube::Color, Cubes, Game};
use gondola_lift::EngineSchematic;
use scratchcard::ScratchCards;
use trebuchet::Trebuchet;

macro_rules! get_input {
    ($day:tt) => {
        std::fs::read_to_string(&format!("input_{}.txt", $day)).expect("Couldn't read input-file!")
    };
}

fn day_1() {
    // read and parse input.txt
    let input = get_input!("day1");
    let result = Trebuchet::new(&input)
        .expect("Failed to create trebuchet")
        .get_calibration_sum();

    println!("Result: {result}");
}

fn day_2() {
    let input = get_input!("day2");

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
    let input = get_input!("day3");
    let schem = EngineSchematic::new(&input).expect("Failed to create schematic");

    let parts_sum = schem.get_parts().sum();
    println!("Part-numbers sum: {parts_sum}");

    let gear_ratio = schem.get_gear_ratio();
    println!("Gear ration: {gear_ratio}");
}

fn day_4() {
    let input = get_input!("day4");
    let mut cards = ScratchCards::from_str(&input).expect("Failed to create scratchcards!");
    let total = cards.get_points_worth();

    println!("Total points: {total}");

    let copies = cards
        .calculate_copies_and_get_total()
        .expect("Failed to calculate copies");
    println!("Total cards won: {copies}");
}

fn main() {
    day_4();
}
