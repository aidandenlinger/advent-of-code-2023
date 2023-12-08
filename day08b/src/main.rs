use std::collections::HashMap;

use num::integer::lcm;

const INPUT: &str = include_str!("../../input/day08.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> i64 {
    let map = parse(input);

    // This kinda sucks, I figured out LCM was useful in the example
    // but checked online to see if it applied to the actual data.
    // The puzzle description doesn't clarify that every path will
    // cycle, etc. Bleh. Next time I'll hold onto my assumptions on
    // the input and see what happens, but the problem statement imo
    // doesn't give us the assumptions we need to fairly solve the problem.
    // I probably should have investigated these assumptions myself, but it feels
    // annoying for the problem itself to not state all the constraints of
    // the data. In that case, these constraints may have been only on my
    // data, making it a non-general solution. Eh.
    map.nodes
        .iter()
        // Get our starting nodes
        .filter(|(val, _)| val.ends_with('A'))
        // Find out how long their path to a node ending with Z is
        .map(|start| {
            let mut curr = start;
            let mut path_len = 0;

            for dir in map.dirs.iter().cycle() {
                match dir {
                    Dir::Left => curr = map.nodes.get_key_value(curr.1 .0).unwrap(),
                    Dir::Right => curr = map.nodes.get_key_value(curr.1 .1).unwrap(),
                }
                path_len += 1;

                if curr.0.ends_with('Z') {
                    break;
                }
            }

            path_len
        })
        // Get LCM of all these numbers, done
        .reduce(lcm)
        .unwrap()
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

    const WEB_EXAMPLE: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_web() {
        assert_eq!(6, run(WEB_EXAMPLE));
    }
}
