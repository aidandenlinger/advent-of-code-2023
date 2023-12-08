use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day08.txt");

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> i32 {
    let map = parse(input);

    let mut curr = map
        .nodes
        .get_key_value(START_NODE)
        .unwrap_or_else(|| panic!("{START_NODE} to be in map"));
    let mut count = 0;

    for dir in map.dirs.iter().cycle() {
        curr = map
            .nodes
            .get_key_value(match dir {
                Dir::Left => curr.1 .0,
                Dir::Right => curr.1 .1,
            })
            .unwrap();
        count += 1;

        if *curr.0 == END_NODE {
            break;
        }
    }

    count
}

fn parse(input: &str) -> Map {
    let (dirs, elements) = input.split_once("\n\n").expect("Empty line");

    let dirs = dirs.chars().map(Dir::new).collect();
    let nodes = elements
        .lines()
        .map(|l| {
            let (key, elems) = l.split_once(" = ").expect("split with =");

            let elem_tuple = elems
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();

            (key, elem_tuple)
        })
        .collect();

    Map { dirs, nodes }
}

#[derive(Debug, PartialEq)]
struct Map<'a> {
    dirs: Vec<Dir>,
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug, PartialEq)]
enum Dir {
    Left,
    Right,
}

impl Dir {
    fn new(input: char) -> Self {
        match input {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!("Not dir"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const WEB_EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_parse() {
        assert_eq!(
            Map {
                dirs: vec![Dir::Left, Dir::Left, Dir::Right],
                nodes: HashMap::from([
                    ("AAA", ("BBB", "BBB")),
                    ("BBB", ("AAA", "ZZZ")),
                    ("ZZZ", ("ZZZ", "ZZZ"))
                ])
            },
            parse(WEB_EXAMPLE_2)
        );
    }

    #[test]
    fn test_web() {
        assert_eq!(2, run(WEB_EXAMPLE));
    }

    #[test]
    fn test_web_2() {
        assert_eq!(6, run(WEB_EXAMPLE_2));
    }
}
