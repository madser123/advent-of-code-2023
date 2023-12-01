pub const NUMBER_STRINGS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[must_use]
pub fn parse_calibration_line(line: &str) -> i32 {
    let mut line = line.to_string();
    let mut assorted = Vec::new();

    // Get all number strings as numbers
    for (i, number_string) in NUMBER_STRINGS.iter().enumerate() {
        line.match_indices(number_string)
            .for_each(|(index, _)| assorted.push((index, i)));
    }

    // Extract all numeric characters
    line.match_indices(char::is_numeric)
        .for_each(|(index, number)| {
            assorted.push((index, number.parse().expect("Couldn't parse number")))
        });

    // Sort by index
    assorted.sort_by_key(|(index, _)| *index);

    // Remove indexes
    let mut numbers = assorted.iter().map(|(_, number)| number);

    // Get first and last number
    let first = numbers.next().expect("No numbers in matches");
    let last = numbers.last().unwrap_or(first);

    // Format numbers
    format!("{first}{last}")
        .parse()
        .expect("Couldn't parse number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_calibration_line() {
        assert_eq!(parse_calibration_line("a1"), 11);
        assert_eq!(parse_calibration_line("a1b2c3d4e5f6g7h8i9"), 19);
        assert_eq!(parse_calibration_line("a1b2c3d4e5f6g7h8i9j0nine"), 19);
    }

    #[test]
    #[should_panic(expected = "No numbers in matches")]
    fn test_parse_invalid_calibration_line() {
        let _ = parse_calibration_line("ihavenonumbers");
    }

    #[test]
    #[should_panic(expected = "No numbers in matches")]
    fn test_parse_empty_calibration_line() {
        let _ = parse_calibration_line("");
    }

    #[test]
    fn test_parse_test_input() {
        assert_eq!(parse_calibration_line("two1nine"), 29);
        assert_eq!(parse_calibration_line("eightwothree"), 83);
        assert_eq!(parse_calibration_line("abcone2threexyz"), 13);
        assert_eq!(parse_calibration_line("xtwone3four"), 24);
        assert_eq!(parse_calibration_line("4nineeightseven2"), 42);
        assert_eq!(parse_calibration_line("zoneight234"), 14);
        assert_eq!(parse_calibration_line("7pqrstsixteen"), 76);
    }
}
