use std::{num::ParseIntError, str::FromStr};

const NUMBER_STRINGS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Error type for parsing calibration values
#[derive(Debug)]
pub enum CalibrationError {
    ParseNumber(ParseIntError),
    NoNumbers,
}

impl From<ParseIntError> for CalibrationError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseNumber(value)
    }
}

impl std::fmt::Display for CalibrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoNumbers => write!(f, "No numbers found"),
            Self::ParseNumber(int_err) => write!(f, "Failed to parse integer: {int_err:?}"),
        }
    }
}

/// A calibration value representing the
/// first and last numbers for a calibration line
pub struct CalibrationValue {
    number: i32,
}

impl CalibrationValue {
    pub const fn number(&self) -> &i32 {
        &self.number
    }
}

impl FromStr for CalibrationValue {
    type Err = CalibrationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut assorted = Vec::new();

        // Get all number strings as numbers
        for (i, number_string) in NUMBER_STRINGS.iter().enumerate() {
            s.match_indices(number_string)
                .for_each(|(index, _)| assorted.push((index, i.to_string())));
        }

        // Extract all numeric characters
        s.match_indices(char::is_numeric).for_each(|(index, number)| {
            assorted.push((index, number.to_string()));
        });

        // Sort by index
        assorted.sort_by_key(|(index, _)| *index);

        let mut numbers = assorted.iter();

        // Get first and last number
        let first = numbers.next().ok_or(CalibrationError::NoNumbers)?;
        let last = numbers.last().unwrap_or(first);

        // Format numbers
        let number = format!("{}{}", first.1, last.1).parse()?;

        Ok(Self { number })
    }
}

pub struct Trebuchet {
    calibration: Vec<CalibrationValue>,
}

impl Trebuchet {
    pub fn new(calibration: &str) -> Result<Self, CalibrationError> {
        Ok(Self {
            calibration: calibration
                .lines()
                .map(CalibrationValue::from_str)
                .collect::<Result<Vec<_>, CalibrationError>>()?,
        })
    }

    pub fn get_calibration_sum(&self) -> i32 {
        self.calibration.iter().map(CalibrationValue::number).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_calibration_line() {
        assert_eq!(
            CalibrationValue::from_str("a1")
                .expect("Failed to parse calibration value")
                .number(),
            &11
        );
        assert_eq!(
            CalibrationValue::from_str("a1b2c3d4e5f6g7h8i9")
                .expect("Failed to parse calibration value")
                .number(),
            &19
        );
        assert_eq!(
            CalibrationValue::from_str("a1b2c3d4e5f6g7h8i9j0nine")
                .expect("Failed to parse calibration value")
                .number(),
            &19
        );
    }

    #[test]
    #[should_panic(expected = "No numbers in matches")]
    fn test_parse_invalid_calibration_line() {
        let _ = CalibrationValue::from_str("ihavenonumbers").expect("No numbers in matches");
    }

    #[test]
    #[should_panic(expected = "No numbers in matches")]
    fn test_parse_empty_calibration_line() {
        let _ = CalibrationValue::from_str("").expect("No numbers in matches");
    }

    #[test]
    fn test_parse_test_input() {
        assert_eq!(
            CalibrationValue::from_str("two1nine")
                .expect("Failed to parse calibration value")
                .number(),
            &29
        );
        assert_eq!(
            CalibrationValue::from_str("eightwothree")
                .expect("Failed to parse calibration value")
                .number(),
            &83
        );
        assert_eq!(
            CalibrationValue::from_str("abcone2threexyz")
                .expect("Failed to parse calibration value")
                .number(),
            &13
        );
        assert_eq!(
            CalibrationValue::from_str("xtwone3four")
                .expect("Failed to parse calibration value")
                .number(),
            &24
        );
        assert_eq!(
            CalibrationValue::from_str("4nineeightseven2")
                .expect("Failed to parse calibration value")
                .number(),
            &42
        );
        assert_eq!(
            CalibrationValue::from_str("zoneight234")
                .expect("Failed to parse calibration value")
                .number(),
            &14
        );
        assert_eq!(
            CalibrationValue::from_str("7pqrstsixteen")
                .expect("Failed to parse calibration value")
                .number(),
            &76
        );
    }
}
