#![allow(clippy::missing_panics_doc, missing_docs, clippy::missing_errors_doc)]

use std::str::FromStr;

use almanac::Almanac;
use boat_race::Races;
use camel_cards::Hands;
use cube_game::{cube::Color, Cubes, Game};
use gondola_lift::EngineSchematic;
use network_nodes::{Network, Node};
use scratchcard::ScratchCards;
use trebuchet::Trebuchet;

macro_rules! time {
    ($name:expr, $block:block) => {{
        let __start = std::time::Instant::now();
        {
            $block
        };
        let __duration = __start.elapsed();
        println!("[TIMING] '{}' took: {:?}", $name, __duration);
    }};

    ($name:expr, $fn:ident) => {
        time!($name, { $fn() })
    };
}

macro_rules! day {
    ($day:tt, $fn:ident) => {
        time!(format!("Day {}", $day), {
            println!("# Day {}", $day);
            $fn(get_input!($day));
        })
    };
}

macro_rules! get_input {
    ($day:tt) => {
        std::fs::read_to_string(&format!("inputs/day{}.txt", $day)).expect("Couldn't read input-file!")
    };
}

fn day1(input: String) {
    let result = Trebuchet::new(&input)
        .expect("Failed to create trebuchet")
        .get_calibration_sum();

    println!("Result: {result}");
}

fn day2(input: String) {
    let mut limits = Cubes::new();
    limits.insert(Color::Red, 12);
    limits.insert(Color::Green, 13);
    limits.insert(Color::Blue, 14);

    let result = Game::new(&input, limits)
        .expect("Failed to create game")
        .get_minimum_powers_sum();

    println!("Result: {result}")
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
    let races = Races::from_multiple_races(&input).expect("Failed to parse races");
    let winning_product = races.get_winning_product();

    println!("Winning product for multiple races: {winning_product}");

    let race = Races::from_single_race(&input).expect("Failed to parse race");
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
    })
}
