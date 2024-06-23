use std::{collections::HashMap, num::ParseIntError, ops::Range, str::FromStr, sync::Arc};

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
    pub fn next_variant(&self) -> Self {
        use ValueType::*;
        match *self {
            Seed => Soil,
            Soil => Fertilizer,
            Fertilizer => Water,
            Water => Light,
            Light => Temperature,
            Temperature => Humidity,
            Humidity => Location,
            Location => unreachable!("Tried to get next variant of Location. Fix the code."),
        }
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

#[derive(Debug)]
pub struct TranslationMap(HashMap<TranslationValue, TranslationValue>);

impl TranslationMap {
    fn add_translation(&mut self, from: TranslationValue, to: TranslationValue) {
        self.0.insert(from, to);
    }

    pub fn get_location_of_seed(&self, seed: u64) -> u64 {
        self.translate(ValueType::Seed, seed, ValueType::Location)
    }

    pub fn translate(&self, source_type: ValueType, source_value: u64, destination: ValueType) -> u64 {
        if source_type > destination {
            panic!("Can't translate backwards")
        }

        let mut current_type = source_type;
        let mut current_id = source_value;

        while current_type != destination {
            (current_id, current_type) = self
                .0
                .iter()
                .find(|(key, _)| key.typ == current_type && key.range.contains(&current_id))
                .map(|(key, value)| {
                    let key_diff = key.range.start.abs_diff(current_id);
                    (value.range.start + key_diff, value.typ)
                })
                .unwrap_or((current_id, current_type.next_variant()));
        }

        current_id
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    translation: Arc<TranslationMap>,
}

impl Almanac {
    pub fn get_lowest_location_of_seed_ranges(&self) -> Option<u64> {
        let mut threads = Vec::new();

        for chunk in self.seeds.chunks(2) {
            let range = chunk[0]..(chunk[0] + chunk[1]);
            let translation = self.translation.clone();

            println!("Spawned a thread on seeds: {range:?}");

            threads.push(std::thread::spawn(move || {
                let result = range
                    .clone()
                    .map(|seed| translation.get_location_of_seed(seed))
                    .collect::<Vec<u64>>();
                println!("Thread {range:?} finished!");
                result
            }));
        }

        threads
            .into_iter()
            .flat_map(|thread| thread.join().expect("Thread failed"))
            .min()
    }

    pub fn get_lowest_location(&self) -> Option<u64> {
        self.seeds
            .iter()
            .map(|seed| self.translation.get_location_of_seed(*seed))
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

        let mut map = TranslationMap(HashMap::new());

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

        println!("{almanac:#?}");
        assert_eq!(lowest_location_of_range, 46);
    }
}
