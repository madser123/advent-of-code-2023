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

pub type Value = u32;

#[derive(Debug, Clone)]
pub struct Layer(Vec<Value>);

impl Layer {
    #[inline(always)]
    fn is_all_zero(&self) -> bool {
        println!("Is zero? {self:?}");
        let sum = self.0.iter().sum::<Value>();
        println!("Sum: {sum}");
        sum == 0
    }

    #[inline(always)]
    pub fn get_next_layer(&self) -> Option<Self> {
        let next = Self(
            self.0
                .windows(2)
                .map(|a| {
                    let (a, b) = (a[0], a[1]);
                    let diff = a.abs_diff(b);
                    println!("Diff {a} | {b} = {diff}");
                    diff
                })
                .collect(),
        );

        if next.is_all_zero() {
            return None;
        }

        Some(next)
    }

    #[inline(always)]
    pub fn last_value(&self) -> Option<&Value> {
        self.0.last()
    }
}

#[derive(Debug)]
pub struct History(Layer);

impl FromStr for History {
    type Err = OasisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Layer(
            s.split(' ').map(|x| x.parse::<Value>()).collect::<Result<_, _>>()?,
        )))
    }
}

impl History {
    pub fn calculate_next_value(&self) -> Value {
        let mut layers = vec![self.0.clone()];
        println!("Calculating layer: {layers:?}");

        loop {
            let Some(current) = layers.last() else {
                unreachable!("Always at least one layer!")
            };

            if let Some(next) = current.get_next_layer() {
                println!("Pushing next layer: {next:?}");
                layers.push(next);
            } else {
                break;
            }
        }

        let mut current = 0u32;

        (layers.len()..0).for_each(|index| {
            let above_value = index.checked_sub(1).map_or_else(
                || layers.first().expect("No layers").last_value().unwrap_or(&0),
                |under| layers[under].last_value().unwrap_or(&0),
            );

            current = dbg!(current.abs_diff(*above_value));

            println!("New current: {current}");
        });

        current
    }
}

#[derive(Debug)]
pub struct Report(Vec<History>);

impl Report {
    pub fn get_next_values_sum(&self) -> Value {
        self.0.iter().map(|history| history.calculate_next_value()).sum()
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

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn solution_1() {
        let report = Report::from_str(EXAMPLE).expect("Failed to parse");

        println!("Parsed! {report:?}");

        assert_eq!(report.get_next_values_sum(), 114);
    }
}
