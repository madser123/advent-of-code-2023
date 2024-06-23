use std::{collections::BTreeMap, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum ScratchCardError {
    Invalid,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ScratchCardError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

#[derive(Debug, Clone)]
pub struct ScratchCard {
    id: usize,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
    amount: usize,
}

impl FromStr for ScratchCard {
    type Err = ScratchCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');

        let id = split
            .next()
            .and_then(|s| s.split_ascii_whitespace().last())
            .ok_or(ScratchCardError::Invalid)?
            .parse()?;

        let mut numbers = split
            .last()
            .ok_or(ScratchCardError::Invalid)?
            .split('|')
            .map(|numbers| {
                numbers
                    .split_ascii_whitespace()
                    .filter_map(|number| number.parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        Ok(Self {
            id,
            numbers: numbers.pop().ok_or(ScratchCardError::Invalid)?,
            winning_numbers: numbers.pop().ok_or(ScratchCardError::Invalid)?,
            amount: 1,
        })
    }
}

impl ScratchCard {
    pub const fn id(&self) -> &usize {
        &self.id
    }

    pub const fn amount(&self) -> &usize {
        &self.amount
    }

    pub fn worth(&self) -> i32 {
        let mut points = 0;
        let total_winning = self.total_winning_numbers();

        for _ in 0..total_winning {
            if points == 0 {
                points += 1;
            } else {
                points *= 2;
            }
        }

        points
    }

    fn total_winning_numbers(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|wn| self.numbers.contains(wn))
            .count()
    }

    pub fn add_copy(&mut self) {
        self.amount += 1;
    }
}

#[derive(Debug)]
pub struct ScratchCards(BTreeMap<usize, ScratchCard>);

impl ScratchCards {
    pub fn get_points_worth(&self) -> i32 {
        self.0
            .values()
            .map(ScratchCard::worth)
            .collect::<Vec<i32>>()
            .iter()
            .sum()
    }

    pub fn calculate_copies_and_get_total(&mut self) -> Result<usize, ScratchCardError> {
        for key in 1..self.0.len() {
            let Some(card) = self.0.get(&key) else {
                panic!("Key exceeded iterator length!")
            };

            let winnings = card.total_winning_numbers();

            if winnings > 0 {
                let range = (key + 1)..=(key + winnings);
                let amount = *card.amount();

                for _ in 0..amount {
                    range
                        .clone()
                        .for_each(|i| self.0.get_mut(&i).map(|card| card.add_copy()).unwrap_or(()))
                }
            }
        }

        Ok(self.0.values().map(|card| card.amount).sum())
    }
}

impl FromIterator<(usize, ScratchCard)> for ScratchCards {
    fn from_iter<T: IntoIterator<Item = (usize, ScratchCard)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for ScratchCards {
    type Err = ScratchCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_iter(
            s.lines()
                .map(ScratchCard::from_str)
                .collect::<Result<Vec<ScratchCard>, _>>()?
                .into_iter()
                .map(|card| (*card.id(), card)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn points() {
        let cards = ScratchCards::from_str(EXAMPLE).expect("Failed to create scratchcards");
        assert_eq!(cards.get_points_worth(), 13);
    }

    #[test]
    fn copies() {
        let mut cards = ScratchCards::from_str(EXAMPLE).expect("Failed to create scratchcards");
        let total = cards
            .calculate_copies_and_get_total()
            .expect("Failed to calculate copies");
        assert_eq!(total, 30)
    }
}
