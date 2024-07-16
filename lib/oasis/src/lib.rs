use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum OasisError {
    ParseHistory(ParseIntError),
}

impl From<ParseIntError> for OasisError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseHistory(value)
    }
}

pub type Value = i64;

#[derive(Debug, Clone)]
pub struct Layer(Vec<Value>);

impl Layer {
    #[inline(always)]
    fn is_all_zero(&self) -> bool {
        self.0.iter().all(|n| *n == 0)
    }

    #[inline(always)]
    pub fn get_next_layer(&self) -> Option<Self> {
        if self.is_all_zero() {
            return None;
        }

        let next = self.0.windows(2).map(|a| a[1] - a[0]).collect();

        Some(Self(next))
    }

    #[inline(always)]
    pub fn last_value(&self) -> Option<&Value> {
        self.0.last()
    }

    #[inline(always)]
    pub fn first_value(&self) -> Option<&Value> {
        self.0.first()
    }
}

#[derive(Debug)]
pub struct History(Layer);

impl FromStr for History {
    type Err = OasisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Layer(
            s.split_whitespace()
                .map(|x| x.parse::<Value>())
                .collect::<Result<_, _>>()?,
        )))
    }
}

impl History {
    fn get_layers(&self) -> Vec<Layer> {
        let mut layers = vec![self.0.clone()];

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

        layers
    }

    pub fn calculate_next_value(&self) -> Value {
        self.get_layers()
            .iter()
            .rev()
            .fold(0, |acc, layer| layer.last_value().map_or(acc, |value| acc + value))
    }

    pub fn calculate_prev_value(&self) -> Value {
        self.get_layers()
            .iter()
            .rev()
            .fold(0, |acc, layer| layer.first_value().map_or(acc, |value| acc + value))
    }
}

#[derive(Debug)]
pub struct Report(Vec<History>);

impl Report {
    pub fn get_next_values_sum(&self) -> Value {
        self.0.iter().map(|history| history.calculate_next_value()).sum()
    }

    pub fn get_prev_values_sum(&self) -> Value {
        self.0.iter().map(|history| history.calculate_prev_value()).sum()
    }
}

impl FromStr for Report {
    type Err = OasisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(History::from_str).collect::<Result<_, _>>()?))
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
