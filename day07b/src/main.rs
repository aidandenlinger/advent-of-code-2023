use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day07.txt");

fn main() {
    println!("{}", run(INPUT));
}

// Deriving Order is perfect for us: it'll first compare handtype, and if equal
// then it'll compare the cards vec
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: i32,
}

fn run(input: &str) -> i32 {
    let mut sorted_hands = parse(input);
    sorted_hands.sort_unstable();

    sorted_hands
        .iter()
        .fold((1, 0), |(rank, winnings), h| {
            (rank + 1, winnings + rank * h.bid)
        })
        .1 // get the final winnings
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new).collect()
}

impl Hand {
    fn new(input: &str) -> Self {
        let (cards, bid) = input.split_once(' ').expect("Space");

        let cards: Vec<Card> = cards.chars().map(Card::new).collect();
        let bid = bid.parse().expect("number");

        let hand_type = {
            // ugly annoying way to get all the cards and their counts, sorted
            let mut card_counts: HashMap<Card, i32> = HashMap::new();

            for card in &cards {
                *card_counts.entry(*card).or_default() += 1;
            }

            let joker_count = *card_counts.get(&Card::Joker).unwrap_or(&0);
            card_counts.remove(&Card::Joker);

            // card_counts holds the counts for every card *except* joker
            let mut card_counts = card_counts.iter().collect::<Vec<(&Card, &i32)>>();
            card_counts.sort_by(|a, b| b.1.cmp(a.1));

            if card_counts.len() <= 1 {
                HandType::FiveOfAKind
            } else if card_counts.len() == 2 && *card_counts[0].1 + joker_count == 4 {
                HandType::FourOfAKind
            } else if card_counts.len() == 2 {
                // if there's only two types of cards (plus any types of potential jokers)
                // and we don't have a five of a kind or a four of a kind (checked earlier),
                // we *must* have a full house. To have two types of cards and 5 cards total,
                // one group must have 3 and one group must have 2. We don't need to check *how*
                // to distribute the jokers, they can be distributed into a full house
                HandType::FullHouse
            } else if card_counts.len() == 3 && *card_counts[0].1 + joker_count == 3 {
                HandType::ThreeOfAKind
            } else if card_counts.len() == 3 {
                // Same logic as full house - we have 3 card types and we don't have
                // 3 of a kind. Therefore, two groups must have 2 cards each. we can
                // distribute jokers as needed
                HandType::TwoPair
            } else if card_counts.len() == 4 {
                // By definition, if we have 4 different groups of cards and 5 total
                // cards we must have a one pair
                HandType::OnePair
            } else if card_counts.len() == 5 {
                HandType::HighCard
            } else {
                unreachable!("Undetermined hand type");
            }
        };

        Self {
            hand_type,
            cards,
            bid,
        }
    }
}

impl Card {
    fn new(input: char) -> Self {
        match input {
            'J' => Self::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!("Not a card"),
        }
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
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
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    #[test]
    fn test_parse() {
        assert_eq!(
            Hand {
                cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                bid: 765,
                hand_type: HandType::OnePair
            },
            parse(WEB_EXAMPLE)[0]
        );
    }

    #[test]
    fn test_web() {
        assert_eq!(5905, run(WEB_EXAMPLE));
    }
}
