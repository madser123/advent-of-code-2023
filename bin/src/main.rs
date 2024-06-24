#![allow(clippy::missing_panics_doc, missing_docs, clippy::missing_errors_doc)]

use std::str::FromStr;

use almanac::Almanac;
use boat_race::Races;
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
    println!("# Day 1");

    let input = get_input!("day1");
    let result = Trebuchet::new(&input)
        .expect("Failed to create trebuchet")
        .get_calibration_sum();

    println!("Result: {result}");
}

fn day_2() {
    println!("# Day 2");

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
    println!("# Day 3");

    let input = get_input!("day3");
    let schem = EngineSchematic::new(&input).expect("Failed to create schematic");

    let parts_sum = schem.get_parts().sum();
    println!("Part-numbers sum: {parts_sum}");

    let gear_ratio = schem.get_gear_ratio();
    println!("Gear ration: {gear_ratio}");
}

fn day_4() {
    println!("# Day 4");

    let input = get_input!("day4");
    let mut cards = ScratchCards::from_str(&input).expect("Failed to create scratchcards!");
    let total = cards.get_points_worth();

    println!("Total points: {total}");

    let copies = cards
        .calculate_copies_and_get_total()
        .expect("Failed to calculate copies");
    println!("Total cards won: {copies}");
}

fn day_5() {
    println!("# Day 5");

    let input = get_input!("day5");
    let almanac = Almanac::from_str(&input).expect("Failed to create almanac");
    let lowest_location = almanac.get_lowest_location().expect("Lowest location not found");

    println!("Lowest location: {lowest_location}");

    let lowest_location_seed_range = almanac
        .get_lowest_location_of_seed_ranges()
        .expect("Lowest location of ranges not found");

    println!("Lowest location of ranges: {lowest_location_seed_range}");
}

fn day_6() {
    println!("# Day 6");

    let input = get_input!("day6");
    let races = Races::from_multiple_races(&input);
    let winning_product = races.get_winning_product();

    println!("Winning product for multiple races: {winning_product}");

    let race = Races::from_single_race(&input);
    let winning_product = race.get_winning_product();

    println!("Winning product for single race: {winning_product}");
}

fn main() {
    println!("## Advent of Code 2023 solutions ##");
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
    day_6();
}
