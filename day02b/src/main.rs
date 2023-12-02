const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", run(INPUT));
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Set {
    blue: u32,
    green: u32,
    red: u32,
}

fn run(input: &str) -> u32 {
    input.lines().map(|g| power(&parse_game(g))).sum()
}

fn parse_game(game: &str) -> Game {
    // First, get the id
    let (id, set_strs) = game.split_once(": ").expect("Game separated by :");
    // id = "Game 1", split out to the number and parse
    let id = id
        .split_ascii_whitespace()
        .nth(1)
        .expect("Game number")
        .parse()
        .expect("A number");

    // Now, parse sets
    let mut sets = Vec::new();

    for set in set_strs.split("; ") {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for color_str in set.split(", ") {
            let (amt, color) = color_str.split_once(' ').expect("Amount and color");
            let amt = amt.parse().unwrap();
            match color {
                "red" => red = amt,
                "green" => green = amt,
                "blue" => blue = amt,
                _ => unreachable!(),
            }
        }

        sets.push(Set { blue, green, red });
    }

    Game { id, sets }
}

fn power(game: &Game) -> u32 {
    let total_red = game.sets.iter().map(|s| s.red).max().unwrap();
    let total_blue = game.sets.iter().map(|s| s.blue).max().unwrap();
    let total_green = game.sets.iter().map(|s| s.green).max().unwrap();

    total_red * total_blue * total_green
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_parse() {
        assert_eq!(
            Game {
                id: 1,
                sets: Vec::from([
                    Set {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                    Set {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Set {
                        green: 2,
                        red: 0,
                        blue: 0
                    }
                ])
            },
            parse_game(WEB_EXAMPLE.lines().next().unwrap())
        );
    }

    #[test]
    fn test_parse_large_num() {
        assert_eq!(
            Game {
                id: 3,
                sets: Vec::from([
                    Set {
                        green: 8,
                        blue: 6,
                        red: 20
                    },
                    Set {
                        blue: 5,
                        red: 4,
                        green: 13
                    },
                    Set {
                        green: 5,
                        red: 1,
                        blue: 0
                    }
                ])
            },
            parse_game(WEB_EXAMPLE.lines().nth(2).unwrap())
        );
    }

    #[test]
    fn test_web() {
        assert_eq!(2286, run(WEB_EXAMPLE));
    }
}
