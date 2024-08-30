//! I chose to use a const-generic for solution 2 in this libary.
//!
//! The `JOKERS` const-generic on each of the types implementing it, represents
//! wether or not we use jokers (true = Use jokers, false = Don't use jokers)
//!

use std::{
    collections::{BTreeMap, BTreeSet},
    num::{ParseIntError, TryFromIntError},
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Error type for parsing cards
#[derive(Debug)]
pub enum CardsError {
    ParseInt(ParseIntError),
    ConvertUsize(TryFromIntError),
    InvalidCard(char),
    GetLowCount,
    GetHighCount,
    GetHighCard,
}

impl From<ParseIntError> for CardsError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<TryFromIntError> for CardsError {
    fn from(value: TryFromIntError) -> Self {
        Self::ConvertUsize(value)
    }
}

/// Card type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card<const JOKERS: bool> {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl<const JOKERS: bool> TryFrom<char> for Card<JOKERS> {
    type Error = CardsError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Card::*;

        let card = match value {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' if JOKERS => Joker,
            'J' if !JOKERS => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,

            invalid => return Err(CardsError::InvalidCard(invalid)),
        };

        Ok(card)
    }
}

/// Card count
#[derive(Debug, Default)]
pub struct CardCount<const JOKERS: bool>(BTreeMap<Card<JOKERS>, u16>);

impl<const JOKERS: bool> CardCount<JOKERS> {
    /// Creates a new card count from a slice of cards
    pub fn new(cards: &[Card<JOKERS>]) -> Result<Self, CardsError> {
        // Create empty map
        let mut counts = Self::default();

        // Count all cards
        cards.iter().for_each(|card| counts.count(card));

        // If jokers are enabled, we need to check some things:
        if JOKERS {
            // If the highest non-joker card doesn't exist, it must be all jokers, so we just return
            let Some(highest_card) = counts.highest_non_joker_card() else {
                return Ok(counts);
            };

            // Get the count of jokers in the cards
            let joker_count = counts.get(&Card::<JOKERS>::Joker).copied().unwrap_or(0);

            // Remove jokers from the map, as they will still be represented in the hand,
            // but shouldn't be represented in the counts.
            counts.remove(&Card::<JOKERS>::Joker);

            // Add the jokers to the highest non-joker card count
            counts.entry(highest_card).and_modify(|val| *val += joker_count);
        };

        Ok(counts)
    }

    /// Adds a card to the counter if no entry is found.
    /// Increments the counter if the card is found.
    #[inline(always)]
    pub fn count(&mut self, card: &Card<JOKERS>) {
        self.entry(*card).and_modify(|count| *count += 1).or_insert(1);
    }

    /// Returns the lowest count in the counter
    #[inline(always)]
    pub fn lowest(&self) -> Result<&u16, CardsError> {
        self.values().min().ok_or(CardsError::GetLowCount)
    }

    /// Returns the highest count in the counter
    #[inline(always)]
    pub fn highest(&self) -> Result<&u16, CardsError> {
        self.values().max().ok_or(CardsError::GetHighCount)
    }

    /// Returns the highest counted non-joker card
    #[inline(always)]
    fn highest_non_joker_card(&self) -> Option<Card<JOKERS>> {
        self.iter()
            .filter(|(card, _)| **card != Card::<JOKERS>::Joker)
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(card, _)| *card)
    }
}

impl<const JOKERS: bool> Deref for CardCount<JOKERS> {
    type Target = BTreeMap<Card<JOKERS>, u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<const JOKERS: bool> DerefMut for CardCount<JOKERS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Hand type
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/// A hand of cards
#[derive(Debug, PartialEq, Eq)]
pub struct Hand<const JOKERS: bool> {
    cards: [Card<JOKERS>; 5],
    bid: u64,
    typ: HandType,
}

impl<const JOKERS: bool> Hand<JOKERS> {
    /// Creates a new hand of cards
    #[inline(always)]
    pub fn new(cards: [Card<JOKERS>; 5], bid: u64) -> Result<Self, CardsError> {
        let typ = Self::get_hand_type(&cards)?;
        Ok(Self { cards, bid, typ })
    }

    /// Gets the hand type of the cards
    #[inline(always)]
    fn get_hand_type(cards: &[Card<JOKERS>; 5]) -> Result<HandType, CardsError> {
        use HandType::*;

        // Get the card type counts
        let counts = CardCount::new(cards)?;

        // Match the card counts, to find out which hand-type we have
        let typ = match counts.len() {
            1 => FiveOfAKind,
            2 => {
                let lowest = *counts.lowest()?;
                match lowest {
                    1 => FourOfAKind,
                    2 => FullHouse,
                    _ => unreachable!("Hands with 2 types should always have a low value of 1 or 2"),
                }
            }
            3 => {
                let highest = *counts.highest()?;
                match highest {
                    3 => ThreeOfAKind,
                    2 => TwoPair,
                    _ => unreachable!("Hands with 3 types should always have a high value of 3 or 2"),
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!("No more than 5 types are possible in a hand with 5 cards."),
        };

        Ok(typ)
    }
}

impl<const JOKERS: bool> PartialOrd for Hand<JOKERS> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const JOKERS: bool> Ord for Hand<JOKERS> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        let typ_cmp = self.typ.cmp(&other.typ);

        // If hand-types aren't equal, return ordering
        if typ_cmp != Equal {
            return typ_cmp;
        }

        // When hand types are equal, we compare cards
        for i in 0..5 {
            let card_cmp = self.cards[i].cmp(&other.cards[i]);

            // Early return card-ordering if cards aren't equal
            if card_cmp != Equal {
                return card_cmp;
            }
        }

        Equal
    }
}

impl<const JOKERS: bool> FromStr for Hand<JOKERS> {
    type Err = CardsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand_str = s.split_ascii_whitespace().collect::<Vec<&str>>();

        let cards = hand_str[0]
            .chars()
            .map(Card::<JOKERS>::try_from)
            .collect::<Result<Vec<Card<JOKERS>>, CardsError>>()?;

        let bid = hand_str[1].parse::<u64>()?;

        Self::new([cards[0], cards[1], cards[2], cards[3], cards[4]], bid)
    }
}

#[derive(Debug)]
pub struct Hands<const JOKERS: bool>(BTreeSet<Hand<JOKERS>>);

impl<const JOKERS: bool> Hands<JOKERS> {
    /// Gets the total winnings of the hands
    #[inline(always)]
    pub fn get_total_winnings(&self) -> u64 {
        // Zip ranks with the values, and multiply
        (1u64..).zip(self.0.iter()).map(|(rank, hand)| rank * hand.bid).sum()
    }
}

impl<const JOKERS: bool> FromStr for Hands<JOKERS> {
    type Err = CardsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(Hand::<JOKERS>::from_str)
                .collect::<Result<BTreeSet<Hand<JOKERS>>, CardsError>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn hand_parsing() {
        let five_kind = Hand::<false>::from_str("AAAAA 0").expect("Failed parsing five of a kind");
        let four_kind = Hand::<false>::from_str("AA8AA 0").expect("Failed parsing four of a kind");
        let full_house = Hand::<false>::from_str("23332 0").expect("Failed parsing full house");
        let three_kind = Hand::<false>::from_str("TTT98 0").expect("Failed to parse three of a kind");
        let two_pair = Hand::<false>::from_str("23432 0").expect("Failed parsing two pair");
        let one_pair = Hand::<false>::from_str("A23A4 0").expect("Failed parsing one pair");
        let high_card = Hand::<false>::from_str("23456 0").expect("Failed pasing high card");

        assert_eq!(five_kind.typ, HandType::FiveOfAKind);
        assert_eq!(four_kind.typ, HandType::FourOfAKind);
        assert_eq!(full_house.typ, HandType::FullHouse);
        assert_eq!(three_kind.typ, HandType::ThreeOfAKind);
        assert_eq!(two_pair.typ, HandType::TwoPair);
        assert_eq!(one_pair.typ, HandType::OnePair);
        assert_eq!(high_card.typ, HandType::HighCard);
    }

    #[test]
    fn hand_ordering() {
        let hand_1 = Hand::<false>::from_str("33332 0").expect("Failed parsing hand 1");
        let hand_2 = Hand::<false>::from_str("2AAAA 0").expect("Failed parsing hand 2");

        assert!(hand_1 > hand_2);

        let hand_3 = Hand::<false>::from_str("77888 0").expect("Failed parsing hand 3");
        let hand_4 = Hand::<false>::from_str("77788 0").expect("Failed parsing hand 4");

        assert!(hand_3 > hand_4);

        assert!(hand_1 > hand_3 && hand_1 > hand_4);
        assert!(hand_2 > hand_3 && hand_2 > hand_4);
    }

    #[test]
    fn hand_parsing_jokers() {
        let five_kind = Hand::<true>::from_str("AAJAA 0").expect("Failed parsing five of a kind");
        let four_kind = Hand::<true>::from_str("QJJQ2 0").expect("Failed parsing four of a kind");

        assert_eq!(five_kind.typ, HandType::FiveOfAKind);
        assert_eq!(four_kind.typ, HandType::FourOfAKind);

        let five_kind_joker = Hand::<true>::from_str("JJJJJ 0").expect("Failed parsing five of a kind jokers");

        assert_eq!(five_kind_joker.typ, HandType::FiveOfAKind);
        assert_eq!(five_kind_joker.cards, [Card::<true>::Joker; 5])
    }

    #[test]
    fn hand_ordering_jokers() {
        let hand_1 = Hand::<true>::from_str("QQQQ2 0").expect("Failed parsing hand 2");
        let hand_2 = Hand::<true>::from_str("JKKK2 0").expect("Failed parsing hand 1");

        assert!(hand_1 > hand_2);
    }

    #[test]
    fn solution_1() {
        let hands = Hands::<false>::from_str(EXAMPLE).expect("Failed parsing hands");
        assert_eq!(hands.get_total_winnings(), 6440);
    }

    #[test]
    fn solution_2() {
        let hands = Hands::<true>::from_str(EXAMPLE).expect("Failed parsing hands with jokers");
        assert_eq!(hands.get_total_winnings(), 5905);
    }
}
