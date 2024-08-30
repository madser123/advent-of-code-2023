use std::{
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

#[derive(Debug)]
pub enum ScratchCardError {
    Invalid,
    ParseInt(ParseIntError),
    ConvertUsize(TryFromIntError),
}

impl std::error::Error for ScratchCardError {}

impl From<ParseIntError> for ScratchCardError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<TryFromIntError> for ScratchCardError {
    fn from(value: TryFromIntError) -> Self {
        Self::ConvertUsize(value)
    }
}

impl std::fmt::Display for ScratchCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid scratchcard"),
            Self::ParseInt(int_err) => write!(f, "Failed to parse integer: {int_err:?}"),
            Self::ConvertUsize(int_err) => write!(f, "Failed to convert integer: {int_err:?}"),
        }
    }
}

/// A scratchcard with winning and regular numbers
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
    /// Get the ID of the scratchcard
    #[inline(always)]
    pub const fn id(&self) -> &usize {
        &self.id
    }

    /// Get the amount of scratchcards (copies)
    #[inline(always)]
    pub const fn amount(&self) -> &usize {
        &self.amount
    }

    /// Get the total worth of the scratchcard
    #[inline]
    pub fn worth(&self) -> Result<i32, ScratchCardError> {
        let total_winning = self.total_winning_numbers();

        if total_winning == 0 {
            return Ok(0);
        }

        let exponent = (total_winning - 1).try_into()?;

        Ok(2i32.pow(exponent))
    }

    /// Get the total amount of winning numbers
    #[inline(always)]
    fn total_winning_numbers(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|wn| self.numbers.contains(wn))
            .count()
    }

    /// Add copies of the scratchcard
    #[inline(always)]
    pub fn add_copies(&mut self, copies: usize) {
        self.amount += copies;
    }
}

#[derive(Debug)]
pub struct ScratchCards(Vec<ScratchCard>);

impl ScratchCards {
    /// Get the total worth of all scratchcards
    pub fn get_points_worth(&self) -> Result<i32, ScratchCardError> {
        Ok(self
            .0
            .iter()
            .map(ScratchCard::worth)
            .collect::<Result<Vec<i32>, ScratchCardError>>()?
            .iter()
            .sum())
    }

    /// Calculate the total amount of scratchcards, with respect to copies
    pub fn calculate_copies_and_get_total(&mut self) -> Result<usize, ScratchCardError> {
        (0..self.0.len()).for_each(|key| {
            let Some(card) = self.0.get(key) else {
                panic!("Key exceeded iterator length!")
            };

            let winnings = card.total_winning_numbers();

            if winnings > 0 {
                let range = (key + 1)..=(key + winnings);
                let amount = *card.amount();
                range.for_each(|i| self.0.get_mut(i).map(|card| card.add_copies(amount)).unwrap_or(()))
            }
        });

        Ok(self.0.iter().map(|card| card.amount).sum())
    }
}

impl FromIterator<ScratchCard> for ScratchCards {
    fn from_iter<T: IntoIterator<Item = ScratchCard>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for ScratchCards {
    type Err = ScratchCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_iter(
            s.lines()
                .map(ScratchCard::from_str)
                .collect::<Result<Vec<ScratchCard>, _>>()?,
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
        assert_eq!(cards.get_points_worth().expect("Failed to calculate worth"), 13);
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
