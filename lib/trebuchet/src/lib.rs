use std::{
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

const NUMBER_STRINGS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub type CalibrationResult<T> = Result<T, CalibrationError>;

/// Error type for parsing calibration values
#[derive(Debug)]
pub enum CalibrationError {
    ParseInt(ParseIntError),
    ConvertInt(TryFromIntError),
    NoNumbers,
}

impl From<ParseIntError> for CalibrationError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<TryFromIntError> for CalibrationError {
    fn from(value: TryFromIntError) -> Self {
        Self::ConvertInt(value)
    }
}

impl std::fmt::Display for CalibrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoNumbers => write!(f, "No numbers found"),
            Self::ParseInt(int_err) => write!(f, "Failed to parse integer: {int_err:?}"),
            Self::ConvertInt(int_err) => write!(f, "Failed to convert integer: {int_err:?}"),
        }
    }
}

pub enum CalibrationNumber {
    Alphabetic(i32),
    Numeric(i32),
}

impl CalibrationNumber {
    pub const fn as_int(&self) -> &i32 {
        match self {
            Self::Alphabetic(n) | Self::Numeric(n) => n,
        }
    }

    pub const fn is_numeric(&self) -> bool {
        match self {
            Self::Numeric(_) => true,
            Self::Alphabetic(_) => false,
        }
    }
}

/// A calibration value representing the
/// first and last numbers for a calibration line
pub struct CalibrationValue {
    numbers: Vec<CalibrationNumber>,
}

impl CalibrationValue {
    pub fn get_numeric_value(&self) -> CalibrationResult<i32> {
        let mut numerics = self.numbers.iter().filter(|n| CalibrationNumber::is_numeric(n));

        let first = numerics.next().ok_or(CalibrationError::NoNumbers)?;
        let last = numerics.last().unwrap_or(first);

        Self::parse_number(first, last)
    }

    pub fn get_value(&self) -> CalibrationResult<i32> {
        // Get first and last number
        let first = self.numbers.first().ok_or(CalibrationError::NoNumbers)?;
        let last = self.numbers.last().unwrap_or(first);

        Self::parse_number(first, last)
    }

    fn parse_number(first: &CalibrationNumber, last: &CalibrationNumber) -> CalibrationResult<i32> {
        // Format numbers
        format!("{}{}", first.as_int(), last.as_int())
            .parse()
            .map_err(|err: ParseIntError| err.into())
    }
}

impl FromStr for CalibrationValue {
    type Err = CalibrationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut assorted = Vec::new();

        // Get all number strings as numbers
        for (i, number_string) in NUMBER_STRINGS.iter().enumerate() {
            s.match_indices(number_string).try_for_each(|(index, _)| {
                let number = i.try_into()?;
                assorted.push((index, CalibrationNumber::Alphabetic(number)));
                Ok::<(), TryFromIntError>(())
            })?;
        }

        // Extract all numeric characters
        s.match_indices(char::is_numeric).try_for_each(|(index, number)| {
            let number = number.parse()?;
            assorted.push((index, CalibrationNumber::Numeric(number)));
            Ok::<(), ParseIntError>(())
        })?;

        // Sort by index
        assorted.sort_by_key(|(index, _)| *index);

        let numbers = assorted.into_iter().map(|(_, n)| n).collect::<Vec<_>>();

        if numbers.is_empty() {
            return Err(CalibrationError::NoNumbers);
        }

        Ok(Self { numbers })
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

    pub fn get_calibration_sum(&self) -> CalibrationResult<i32> {
        Ok(self
            .calibration
            .iter()
            .map(CalibrationValue::get_value)
            .collect::<CalibrationResult<Vec<i32>>>()?
            .iter()
            .sum())
    }

    pub fn get_numeric_calibration_sum(&self) -> CalibrationResult<i32> {
        Ok(self
            .calibration
            .iter()
            .map(CalibrationValue::get_numeric_value)
            .collect::<CalibrationResult<Vec<i32>>>()?
            .iter()
            .sum())
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
                .get_value()
                .expect("Failed to get value"),
            11
        );
        assert_eq!(
            CalibrationValue::from_str("a1b2c3d4e5f6g7h8i9")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            19
        );
        assert_eq!(
            CalibrationValue::from_str("a1b2c3d4e5f6g7h8i9j0nine")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            19
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
                .get_value()
                .expect("Failed to get value"),
            29
        );
        assert_eq!(
            CalibrationValue::from_str("eightwothree")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            83
        );
        assert_eq!(
            CalibrationValue::from_str("abcone2threexyz")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            13
        );
        assert_eq!(
            CalibrationValue::from_str("xtwone3four")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            24
        );
        assert_eq!(
            CalibrationValue::from_str("4nineeightseven2")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            42
        );
        assert_eq!(
            CalibrationValue::from_str("zoneight234")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            14
        );
        assert_eq!(
            CalibrationValue::from_str("7pqrstsixteen")
                .expect("Failed to parse calibration value")
                .get_value()
                .expect("Failed to get value"),
            76
        );
    }
}
