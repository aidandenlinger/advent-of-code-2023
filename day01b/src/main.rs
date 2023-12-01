use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> u32 {
    input.lines().map(line_val).sum()
}

#[derive(Clone, Debug)]
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
                Some(first_word) => {
                    if curr_ind_val.index < first_word.index {
                        Some(curr_ind_val.clone())
                    } else {
                        Some(first_word)
                    }
                }
                None => Some(curr_ind_val.clone()),
            };
        };

        // and last occurance
        if let Some(ind) = line.rfind(word_digit) {
            let curr_ind_val = IndVal {
                index: ind,
                val: word_digits[word_digit],
            };

            last_word = match last_word {
                Some(last_word) => {
                    if curr_ind_val.index > last_word.index {
                        Some(curr_ind_val.clone())
                    } else {
                        Some(last_word)
                    }
                }
                None => Some(curr_ind_val.clone()),
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
    let first = match (first_word, first_digit) {
        (None, None) => unreachable!(),
        (None, Some(first_digit)) => first_digit.val,
        (Some(first_word), None) => first_word.val,
        (Some(first_word), Some(first_digit)) => {
            if first_word.index < first_digit.index {
                first_word.val
            } else {
                first_digit.val
            }
        }
    };

    let last = match (last_word, last_digit) {
        (None, None) => unreachable!(),
        (None, Some(last_digit)) => last_digit.val,
        (Some(last_word), None) => last_word.val,
        (Some(last_word), Some(last_digit)) => {
            if last_word.index > last_digit.index {
                last_word.val
            } else {
                last_digit.val
            }
        }
    };

    first * 10 + last
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
