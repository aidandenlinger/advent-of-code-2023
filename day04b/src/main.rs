use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day04.txt");

fn main() {
    println!("{}", run(INPUT));
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: i32,
    copies: usize,
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

        Card {
            id,
            winning,
            have,
            copies: 1,
        }
    }

    fn win_amt(&self) -> usize {
        self.have.intersection(&self.winning).count()
    }
}

fn run(input: &str) -> usize {
    let mut cards: Vec<Card> = input.lines().map(Card::new).collect();
    let mut num_of_cards = cards.len();

    for index in 0..cards.len() {
        let card = &cards[index];
        let copies = card.copies;
        let winning_numbers = card.win_amt();

        // For each winning number, we get a new card
        // For each copy of this card, we get 1 of that new card
        for num in 1..=winning_numbers {
            num_of_cards += copies;
            cards[index + num].copies += copies;
        }
    }

    num_of_cards
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
        assert_eq!(30, run(WEB_EXAMPLE));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Card {
                id: 1,
                copies: 1,
                winning: HashSet::from([41, 48, 83, 86, 17]),
                have: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
            },
            Card::new(WEB_EXAMPLE.lines().next().unwrap())
        );
    }
}
