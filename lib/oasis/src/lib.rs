use std::{num::ParseIntError, str::FromStr};

/// Errors than can occur during an Oasis report
#[derive(Debug)]
pub enum OasisError {
    /// Failed to parse History
    ParseHistory(ParseIntError),

    /// No histories found when parsing
    NoHistories,
}

impl From<ParseIntError> for OasisError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseHistory(value)
    }
}

/// A value from an Oasis report
pub type Value = i64;

/// A Layer in a History
#[derive(Debug, Clone)]
pub struct Layer(Vec<Value>);

impl Layer {
    /// Checks if all values in the layer is `0`
    fn is_all_zero(&self) -> bool {
        self.0.iter().all(|n| *n == 0)
    }

    /// Calculates the differences between values in the layer, to create the layer below it.
    pub fn get_next_layer(&self) -> Option<Self> {
        if self.is_all_zero() {
            return None;
        }

        // Get differences
        let next = self.0.windows(2).map(|a| a[1] - a[0]).collect();

        Some(Self(next))
    }

    /// Returns the last value of the layer, if any
    pub fn last_value(&self) -> Option<&Value> {
        self.0.last()
    }

    /// Returns the first value of the layer, if any
    pub fn first_value(&self) -> Option<&Value> {
        self.0.first()
    }
}

/// A History
#[derive(Debug)]
pub struct History(Vec<Layer>);

impl FromStr for History {
    type Err = OasisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Create a vector, containing the first layer
        let mut layers = vec![Layer(
            s.split_whitespace()
                .map(|x| x.parse::<Value>())
                .collect::<Result<_, _>>()?,
        )];

        // Calculate the rest of the layers
        loop {
            let Some(current) = layers.last() else {
                unreachable!("Always at least one layer!")
            };

            if let Some(next) = current.get_next_layer() {
                layers.push(next);
            } else {
                break;
            }
        }

        Ok(Self(layers))
    }
}

impl History {
    /// Calculates the next value in the history
    pub fn calculate_next_value(&self) -> Value {
        self.0
            .iter()
            .rev()
            .fold(0, |acc, layer| layer.last_value().map_or(acc, |value| acc + value))
    }

    /// Calculates the previous value in the history
    pub fn calculate_prev_value(&self) -> Value {
        self.0.iter().rev().fold(0, |below, layer| {
            layer.first_value().map_or(below, |value| value - below)
        })
    }
}

/// An Oasis-Report
#[derive(Debug)]
pub struct Report(Vec<History>);

impl Report {
    /// Returns the sum of all the next values in the Reports Histories
    pub fn get_next_values_sum(&self) -> Value {
        self.0.iter().map(|history| history.calculate_next_value()).sum()
    }

    /// Returns the sum of all the previous values in the Reports Histories
    pub fn get_prev_values_sum(&self) -> Value {
        self.0.iter().map(|history| history.calculate_prev_value()).sum()
    }

    /// Checks if the report is empty (No histories)
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl FromStr for Report {
    type Err = OasisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let report = Self(s.lines().map(History::from_str).collect::<Result<_, _>>()?);

        if report.is_empty() {
            return Err(OasisError::NoHistories);
        }

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    const EXAMPLE_2: &str = "0 -1 -1 0";

    const EXAMPLE_3: &str = "0 1 1 0";

    #[test]
    fn bin_test() {
        let report = Report::from_str(EXAMPLE_2).expect("Failed to parse");
        let sum = report.get_next_values_sum();

        assert_eq!(sum, 2);

        let report = Report::from_str(EXAMPLE_3).expect("Failed to parse");
        let sum = report.get_next_values_sum();

        assert_eq!(sum, -2);
    }

    #[test]
    #[should_panic]
    fn empty_test() {
        Report::from_str("").expect("Failed to parse");
    }

    #[test]
    fn solution_1() {
        let report = Report::from_str(EXAMPLE_1).expect("Failed to parse");
        let sum = report.get_next_values_sum();

        assert_eq!(sum, 114);
    }

    #[test]
    fn solution_2() {
        let report = Report::from_str(EXAMPLE_1).expect("Failed to parse");
        let sum = report.get_prev_values_sum();

        assert_eq!(sum, 2);
    }
}
