const INPUT: &str = include_str!("../../input/day09.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> i32 {
    parse(input).iter().map(History::predict).sum()
}

fn parse(input: &str) -> Vec<History> {
    input
        .lines()
        .map(|s| {
            History(
                s.split_ascii_whitespace()
                    .map(|s| s.parse().expect("number"))
                    .collect(),
            )
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct History(Vec<i32>);

impl History {
    fn predict(&self) -> i32 {
        let mut layers = vec![self.0.clone()];

        while layers.last().unwrap().iter().any(|&a| a != 0) {
            layers.push(
                layers
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|arr| {
                        if let [a, b] = arr {
                            b - a
                        } else {
                            unreachable!()
                        }
                    })
                    .collect(),
            );
        }

        // Add 0 to the last layer
        layers.reverse();
        layers.first_mut().unwrap().push(0);

        for index in 1..layers.len() {
            let last_num_upper_layer = *layers[index - 1].last().unwrap();
            let last_num = *layers[index].last().unwrap();
            layers[index].push(last_num + last_num_upper_layer);
        }

        *layers.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_parse() {
        assert_eq!(History(vec![0, 3, 6, 9, 12, 15]), parse(WEB_EXAMPLE)[0]);
    }

    #[test]
    fn test_history() {
        assert_eq!(
            vec![18, 28, 68],
            parse(WEB_EXAMPLE)
                .iter()
                .map(History::predict)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_web() {
        assert_eq!(114, run(WEB_EXAMPLE));
    }
}
