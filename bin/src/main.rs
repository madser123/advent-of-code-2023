#![allow(clippy::missing_panics_doc, missing_docs, clippy::missing_errors_doc)]

use calibration_parser::parse_calibration_line;

fn main() {
    // read and parse input.txt
    let result: i32 = std::fs::read_to_string("input.txt")
        .expect("Couldn't read input-file")
        .lines()
        .map(parse_calibration_line)
        .sum();
    println!("Result: {result}");
}
