use std::{cmp::min, ops::Range};

const INPUT: &str = include_str!("../../input/day05.txt");

fn main() {
    println!("{}", run(INPUT));
}

#[derive(Debug, PartialEq)]
struct Seeds(Vec<Range<i64>>);

struct Almanac(Vec<Mapping>);

#[derive(Debug)]
struct Mapping(Vec<SingleRange>);

#[derive(Debug, PartialEq)]
struct SingleRange {
    source_range: Range<i64>,
    translation: i64,
}

fn run(input: &str) -> i64 {
    let (seeds, almanac) = parse(input);

    let mut min_score: Option<i64> = None;
    for seed_range in seeds.0 {
        for seed in seed_range {
            let curr_score = almanac.convert(seed);
            min_score = match min_score {
                Some(score) => Some(min(score, curr_score)),
                None => Some(curr_score),
            }
        }
    }

    min_score.unwrap()
}

impl Almanac {
    fn new(mappings: &str) -> Self {
        Almanac(mappings.split("\n\n").map(Mapping::new).collect())
    }

    fn convert(&self, seed: i64) -> i64 {
        self.0
            .iter()
            .fold(seed, |val, mapping| mapping.convert(val))
    }
}

impl Mapping {
    fn new(mapping: &str) -> Self {
        Mapping(mapping.lines().skip(1).map(SingleRange::new).collect())
    }

    fn convert(&self, seed: i64) -> i64 {
        for mapping in &self.0 {
            if mapping.source_range.contains(&seed) {
                return seed + mapping.translation;
            }
        }

        seed
    }
}

impl SingleRange {
    fn new(input: &str) -> Self {
        let nums: Vec<&str> = input.split_ascii_whitespace().collect();

        let destination: i64 = nums[0].parse().expect("number");
        let source: i64 = nums[1].parse().expect("number");
        let range_len: i64 = nums[2].parse().expect("number");

        SingleRange {
            source_range: source..(source + range_len),
            translation: destination - source,
        }
    }
}

impl Seeds {
    fn new(seeds: &str) -> Self {
        Seeds(
            seeds
                .split_once(": ")
                .expect("Seed: header")
                .1
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("numbers"))
                .collect::<Vec<i64>>()
                .chunks_exact(2)
                .map(|chunk| {
                    if let &[range_start, range_len] = chunk {
                        range_start..(range_start + range_len)
                    } else {
                        unreachable!("Not chunks of 2");
                    }
                })
                .collect(),
        )
    }
}

fn parse(input: &str) -> (Seeds, Almanac) {
    let (seeds, mappings) = input.split_once("\n\n").expect("Seeds at start");

    (Seeds::new(seeds), Almanac::new(mappings))
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "seeds: 79 14 55 13

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
56 93 4
";

    #[test]
    fn test_first_mapping() {
        let (_, almanac) = parse(WEB_EXAMPLE);

        assert_eq!(81, almanac.0[0].convert(79));
        assert_eq!(14, almanac.0[0].convert(14));
        assert_eq!(57, almanac.0[0].convert(55));
        assert_eq!(13, almanac.0[0].convert(13));
    }

    #[test]
    fn test_parse() {
        let (seeds, almanac) = parse(WEB_EXAMPLE);

        assert_eq!(Seeds(vec![79..93, 55..68]), seeds);
        assert_eq!(
            SingleRange {
                source_range: 98..(98 + 2),
                translation: (50 - 98)
            },
            almanac.0[0].0[0]
        );
    }

    #[test]
    fn test_web() {
        assert_eq!(46, run(WEB_EXAMPLE));
    }
}
