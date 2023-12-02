use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day01.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> u32 {
    input.lines().map(line_val).sum()
}

// Will order by the first element, our index. Indexes are unique among IndVals
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IndVal {
    index: usize,
    val: u32,
}

fn line_val(line: &str) -> u32 {
    let word_digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut first_word: Option<IndVal> = None;
    let mut last_word: Option<IndVal> = None;

    // Search for each word in the string to find the highest and lowest string occurances
    for word_digit in word_digits.keys() {
        // first occurance...
        if let Some(ind) = line.find(word_digit) {
            let curr_ind_val = IndVal {
                index: ind,
                val: word_digits[word_digit],
            };

            first_word = match first_word {
                Some(first_word) => Some(std::cmp::min(curr_ind_val, first_word)),
                None => Some(curr_ind_val),
            };
        };

        // and last occurance
        if let Some(ind) = line.rfind(word_digit) {
            let curr_ind_val = IndVal {
                index: ind,
                val: word_digits[word_digit],
            };

            last_word = match last_word {
                Some(last_word) => Some(std::cmp::max(curr_ind_val, last_word)),
                None => Some(curr_ind_val),
            };
        }
    }

    // Now, get the highest/lowest digits and their positions
    let digits: Vec<(usize, u32)> = line
        .chars()
        .enumerate()
        .filter(|a| a.1.is_ascii_digit())
        .map(|a| (a.0, a.1.to_digit(10).unwrap()))
        .collect();

    let first_digit = digits.first().map(|digit| IndVal {
        index: digit.0,
        val: digit.1,
    });

    let last_digit = digits.last().map(|digit| IndVal {
        index: digit.0,
        val: digit.1,
    });

    // Finally, get the highest/lowest numbers at all by comparing positions
    // Each string must have at least one low number and one high number, so one of these will be Some
    // Using min directly could result in "None", so we need to explicitly check our options
    let first = match (first_word, first_digit) {
        (None, None) => unreachable!(),
        (None, Some(first_digit)) => first_digit.val,
        (Some(first_word), None) => first_word.val,
        (Some(first_word), Some(first_digit)) => std::cmp::min(first_word, first_digit).val,
    };

    // Since "Some" has a higher ordering than "None", we can just call max directly and get a Some out
    let last = std::cmp::max(last_word, last_digit).unwrap().val;

    first * 10 + last
}

// *Much* simpler code inspired by some online solutions. I was entraced
// by the str .find() method and only having to call it 18 times, while
// this solution searches on *every* character 9 times. However, that's
// pretty much what .find() has to be doing behind the hood, and this
// has a much simpler solution by not having to combine the results from
// find/rfind with the digit results. It basically does both at once.
// The real ultimate solution is probably using regex and replacing smartly.
// But overall I'm still okay with my first answer, it's not too outclassed,
// it's just way overcomplicated
fn _line_val_alt(line: &str) -> u32 {
    let word_digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut digits = Vec::new();

    for (ind, char) in line.chars().enumerate() {
        if char.is_ascii_digit() {
            digits.push(char.to_digit(10).unwrap());
        } else {
            let sub = &line[ind..];
            for word in word_digits.keys() {
                if sub.starts_with(word) {
                    digits.push(word_digits[word]);
                    break;
                }
            }
        }
    }

    digits.first().unwrap() * 10 + digits.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_web() {
        let answers = [29, 83, 13, 24, 42, 14, 76];
        for (line, answer) in std::iter::zip(WEB_EXAMPLE.lines(), answers) {
            assert_eq!(answer, line_val(line));
        }
        assert_eq!(281, run(WEB_EXAMPLE));
    }

    #[test]
    fn same_number_twice() {
        // "five" is in this string twice, so we need to check for the first *and* last occurance
        assert_eq!(35, line_val("sdpgz3five4seven6fiveh"));
    }
}
