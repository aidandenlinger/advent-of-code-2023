use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day04.txt");

fn main() {
    println!("{}", run(INPUT));
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: i32,
    winning: HashSet<i32>,
    have: HashSet<i32>,
}

impl Card {
    fn new(card: &str) -> Self {
        let (game_id, numbers) = card.split_once(": ").expect(": between games");

        let id = game_id
            .split_ascii_whitespace()
            .nth(1)
            .expect("numbers separated by space")
            .parse()
            .expect("number");

        let to_num_arr = |input: &str| {
            input
                .split_ascii_whitespace()
                .map(|s| s.parse().expect("number"))
                .collect()
        };

        let (winning, have) = numbers.split_once(" | ").expect("split with |");
        let winning = to_num_arr(winning);
        let have = to_num_arr(have);

        Card { id, winning, have }
    }

    fn win(self) -> i32 {
        let shared_nums = self.have.intersection(&self.winning).count();

        if shared_nums == 0 {
            0
        } else {
            2_i32.pow((shared_nums - 1).try_into().unwrap())
        }
    }
}

fn run(input: &str) -> i32 {
    input.lines().map(|s| Card::new(s).win()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_web() {
        assert_eq!(13, run(WEB_EXAMPLE));
    }

    #[test]
    fn test_win() {
        assert_eq!(8, Card::new(WEB_EXAMPLE.lines().next().unwrap()).win());
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Card {
                id: 1,
                winning: HashSet::from([41, 48, 83, 86, 17]),
                have: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
            },
            Card::new(WEB_EXAMPLE.lines().next().unwrap())
        );
    }
}
