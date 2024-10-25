#![allow(clippy::missing_panics_doc, missing_docs, clippy::missing_errors_doc)]

// Macros
mod macros;

// Imports
use std::str::FromStr;

// Solution imports
use almanac::Almanac;
use boat_race::Races;
use camel_cards::Hands;
use cube_game::{cube::Color, Cubes, Game};
use gondola_lift::EngineSchematic;
use network_nodes::{Network, Node};
use oasis::Report;
use observatory::Image;
use pipe_maze::Maze;
use scratchcard::ScratchCards;
use trebuchet::Trebuchet;

fn day1(input: String) {
    let trebuchet = Trebuchet::new(&input).expect("Failed to create trebuchet");
    let value_numeric = trebuchet
        .get_numeric_calibration_sum()
        .expect("Failed to get numeric calibration value");
    println!("Numeric calibration value: {value_numeric}");

    let value = trebuchet
        .get_calibration_sum()
        .expect("Failed to get calibration value");
    println!("Calibration value: {value}");
}

fn day2(input: String) {
    let mut limits = Cubes::new();
    limits.insert(Color::Red, 12);
    limits.insert(Color::Green, 13);
    limits.insert(Color::Blue, 14);

    let game = Game::new(&input, limits).expect("Failed to create game");
    let ids_sum = game.get_ids_sum();
    println!("Ids sum: {ids_sum}");

    let minimum_powers = game.get_minimum_powers_sum();
    println!("Powers sum: {minimum_powers}");
}

fn day3(input: String) {
    let schem = EngineSchematic::new(&input).expect("Failed to create schematic");
    let parts_sum = schem.get_parts().sum();
    println!("Part-numbers sum: {parts_sum}");

    let gear_ratio = schem.get_gear_ratio();
    println!("Gear ration: {gear_ratio}");
}

fn day4(input: String) {
    let mut cards = ScratchCards::from_str(&input).expect("Failed to create scratchcards!");
    let total = cards.get_points_worth().expect("Failed to calculate total");
    println!("Total points: {total}");

    let copies = cards
        .calculate_copies_and_get_total()
        .expect("Failed to calculate copies");
    println!("Total cards won: {copies}");
}

fn day5(input: String) {
    let almanac = Almanac::from_str(&input).expect("Failed to create almanac");

    let lowest_location = almanac.get_lowest_location().expect("Lowest location not found");
    println!("Lowest location: {lowest_location}");

    let lowest_location_seed_range = almanac
        .get_lowest_location_of_seed_ranges()
        .expect("Lowest location of ranges not found");
    println!("Lowest location of ranges: {lowest_location_seed_range}");
}

fn day6(input: String) {
    let races = Races::from_str(&input).expect("Failed to parse races");
    let winning_product = races.get_winning_product();
    println!("Winning product for multiple races: {winning_product}");

    let race = races.as_single_race().expect("Failed to parse race");
    let winning_product = race.get_winning_product();
    println!("Winning product for single race: {winning_product}");
}

fn day7(input: String) {
    let hands = Hands::<false>::from_str(&input).expect("Failed parsing hands");
    let total_winnings = hands.get_total_winnings();
    println!("Total winnings: {total_winnings}");

    let hands = Hands::<true>::from_str(&input).expect("Failed parsing hands with jokers");
    let total_joker_winnings = hands.get_total_winnings();
    println!("Total winnings with jokers: {total_joker_winnings}");
}

fn day8(input: String) {
    let network = Network::from_str(&input).expect("Failed parsing network");
    let find = Node::from_str("ZZZ").expect("Failed parsing finde node");
    let steps = network.find_steps_required_for(&find).expect("Failed getting steps");
    println!("Steps from 'AAA' to 'ZZZ': {steps}");

    let ghost_steps = network
        .find_ghost_steps_required_for('Z')
        .expect("Failed getting ghost steps");
    println!("Ghost steps from '__A' to '__Z': {ghost_steps}");
}

fn day9(input: String) {
    let report = Report::from_str(&input).expect("Failed parsing report");
    let next_sum = report.get_next_values_sum();
    println!("Sum of next values for histories: {next_sum}");

    let prev_sum = report.get_prev_values_sum();
    println!("Sum of prev values for histories: {prev_sum}")
}

fn day10(input: String) {
    let maze = Maze::from_str(&input).expect("Failed parsing maze");
    let farthest_point = maze.find_farthest_point_from_start();
    println!("Farthest point from start: {farthest_point}");

    let area = maze.find_nest_area();
    println!("Area of nest: {area}");
}

fn day11(input: String) {
    let mut image = Image::from_str(&input).expect("Failed to parse image");

    image.resize(2);

    let paths_sum = image.find_shortest_paths_sum();
    println!("Shortest paths sum: {paths_sum}");

    image.resize(1_000_000);

    let paths_sum = image.find_shortest_paths_sum();
    println!("Shortest paths sum 1mil: {paths_sum}");
}

fn main() {
    println!("## Advent of Code 2023 solutions ##");
    time!("All", {
        day!(1, day1);
        day!(2, day2);
        day!(3, day3);
        day!(4, day4);
        day!(5, day5);
        day!(6, day6);
        day!(7, day7);
        day!(8, day8);
        day!(9, day9);
        day!(10, day10);
        day!(11, day11);
    })
}
