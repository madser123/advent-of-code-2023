use std::{collections::BTreeMap, num::ParseIntError, ops::Range, str::FromStr, sync::Arc};

#[derive(Debug)]
pub enum ParseAlmanacError {
    ParseInt(ParseIntError),
    GetSeeds,
    InvalidMapKey(String),
    NoKeyFound,
}

impl From<ParseIntError> for ParseAlmanacError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ValueType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl ValueType {
    pub const fn next_variant(&self) -> Option<Self> {
        use ValueType::*;
        let next = match *self {
            Seed => Soil,
            Soil => Fertilizer,
            Fertilizer => Water,
            Water => Light,
            Light => Temperature,
            Temperature => Humidity,
            Humidity => Location,
            Location => return None,
        };

        Some(next)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslationValue {
    typ: ValueType,
    range: Range<u64>,
}

impl TranslationValue {
    pub const fn new(typ: ValueType, from: u64, to: u64) -> Self {
        Self { typ, range: from..to }
    }
}

impl PartialOrd for TranslationValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TranslationValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        let type_compare = self.typ.cmp(&other.typ);

        if type_compare != Equal {
            return type_compare;
        }

        // self range is before other range, so it is less
        if self.range.end <= other.range.start {
            Less
        // self range is after other range, so it is greater
        } else if self.range.start >= other.range.end {
            Greater
        } else {
            Equal
        }
    }
}

#[derive(Debug)]
pub struct TranslationMap(BTreeMap<TranslationValue, TranslationValue>);

impl TranslationMap {
    fn add_translation(&mut self, from: TranslationValue, to: TranslationValue) {
        self.0.insert(from, to);
    }

    pub fn get_location_of_seed(&self, seed: u64) -> Option<u64> {
        self.walk(ValueType::Seed, seed..seed)
    }

    fn translate_range(&self, source_type: ValueType, source_range: Range<u64>) -> Option<u64> {
        self.walk(source_type, source_range)
    }

    fn walk(&self, from: ValueType, range: Range<u64>) -> Option<u64> {
        // Iterate over keys for current valuetype (Seed, Soil, etc..) where the range is hitting
        let keys = self
            .0
            .iter()
            .filter(|(key, _)| key.typ == from && range.start <= key.range.end && key.range.start <= range.end)
            .collect::<Vec<_>>();

        // Check if next variant exists
        let Some(next) = from.next_variant() else {
            // We arrived at Location-variant, so return the beginning of the range
            return Some(range.start);
        };

        // No keys found, so we just walk the next variant with the same range
        if keys.is_empty() {
            return self.walk(next, range);
        }

        // For each key, test the lowest value possible first, and keep doing so, until the whole range has been checked.
        keys.iter()
            .flat_map(|(key, value)| {
                // Diff between value and key
                let diff = value.range.start.abs_diff(key.range.start);

                // Get the range translation
                let new_range = if key.range.start < value.range.start {
                    (key.range.start.max(range.start) + diff)..(key.range.end.min(range.end) + diff)
                } else {
                    (key.range.start.max(range.start) - diff)..(key.range.end.min(range.end) - diff)
                };

                // Walk the next variant with the new range
                self.walk(next, new_range)
            })
            // Return the lowest value found
            .min()
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    translation: Arc<TranslationMap>,
}

impl Almanac {
    pub fn get_lowest_location_of_seed_ranges(&self) -> Option<u64> {
        let mut min = u64::MAX;

        for chunk in self.seeds.chunks(2) {
            let range = chunk[0]..(chunk[0] + chunk[1]);
            let translation = self.translation.clone();

            let result = translation.translate_range(ValueType::Seed, range.clone())?;

            if result < min {
                min = result;
            };
        }

        Some(min)
    }

    pub fn get_lowest_location(&self) -> Option<u64> {
        self.seeds
            .iter()
            .flat_map(|seed| self.translation.get_location_of_seed(*seed))
            .min()
    }
}

impl FromStr for Almanac {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Seeds are on the first line
        let seeds = lines
            .next()
            .ok_or(ParseAlmanacError::GetSeeds)?
            .split(':')
            .last()
            .ok_or(ParseAlmanacError::GetSeeds)?
            .split_ascii_whitespace()
            .map(str::parse::<u64>)
            .collect::<Result<Vec<u64>, _>>()?;

        // Remove empty line after seeds
        lines.next();

        let mut map = TranslationMap(BTreeMap::new());

        let maps = lines
            // Replace newline seperators with pipes, for easy splitting.
            .map(|s| {
                if s.is_empty() {
                    "|\n".to_string()
                } else {
                    format!("{s}\n")
                }
            })
            .collect::<String>();

        let maps = maps
            .split("|\n")
            .map(|s| s.lines().map(|l| l.to_string()).collect())
            .collect::<Vec<Vec<String>>>();

        for map_vec in maps {
            let mut map_lines = map_vec.iter();

            let key = map_lines.next().ok_or(ParseAlmanacError::NoKeyFound)?;

            let (source, destination) = match key {
                _ if key.contains("seed-to-soil") => (ValueType::Seed, ValueType::Soil),
                _ if key.contains("soil-to-fertilizer") => (ValueType::Soil, ValueType::Fertilizer),
                _ if key.contains("fertilizer-to-water") => (ValueType::Fertilizer, ValueType::Water),
                _ if key.contains("water-to-light") => (ValueType::Water, ValueType::Light),
                _ if key.contains("light-to-temperature") => (ValueType::Light, ValueType::Temperature),
                _ if key.contains("temperature-to-humidity") => (ValueType::Temperature, ValueType::Humidity),
                _ if key.contains("humidity-to-location") => (ValueType::Humidity, ValueType::Location),

                invalid => return Err(ParseAlmanacError::InvalidMapKey(invalid.to_string())),
            };

            let numbers = &mut map_lines.take_while(|s| s.chars().any(|c| c.is_numeric()));

            numbers.try_for_each(|s| {
                let numbers = s
                    .split_ascii_whitespace()
                    .map(|number| number.parse::<u64>())
                    .collect::<Result<Vec<_>, _>>()?;

                let dest_start = numbers[0];
                let source_start = numbers[1];
                let length = numbers[2];

                map.add_translation(
                    TranslationValue::new(source, source_start, length + source_start),
                    TranslationValue::new(destination, dest_start, length + dest_start),
                );

                Ok::<(), ParseAlmanacError>(())
            })?;

            // Fill in the blank spaces
            let keys = map
                .0
                .keys()
                .filter(|key| key.typ == source)
                .cloned()
                .collect::<Vec<_>>();

            let mut from = 0;

            for key in keys {
                let value = key.range.start;

                if value == 0 {
                    from = value;
                    continue;
                }

                let new_key = TranslationValue::new(source, from, value);

                if !map.0.contains_key(&new_key) {
                    map.add_translation(new_key, TranslationValue::new(destination, from, value));
                }

                from = value;
            }
        }

        Ok(Self {
            seeds,
            translation: Arc::new(map),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn solution_1() {
        let almanac = Almanac::from_str(EXAMPLE).expect("Failed to create map");
        let lowest_location = almanac.get_lowest_location().expect("Failed to get lowest location");
        assert_eq!(lowest_location, 35);
    }

    #[test]
    fn solution_2() {
        let almanac = Almanac::from_str(EXAMPLE).expect("Failed to create map");
        let lowest_location_of_range = almanac
            .get_lowest_location_of_seed_ranges()
            .expect("Failed to get lowest location");

        assert_eq!(lowest_location_of_range, 46);
    }
}
