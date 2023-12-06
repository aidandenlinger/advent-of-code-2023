const INPUT: &str = include_str!("../../input/day06.txt");

fn main() {
    println!("{}", run(INPUT));
}

#[derive(PartialEq, Debug)]
struct Race {
    time: i32,
    distance: i32,
}

fn run(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|r| r.win_options().len()) // number of winning options
        .reduce(|acc, e| acc * e) // multiply them all together
        .unwrap()
}

impl Race {
    fn win_options(&self) -> Vec<i32> {
        (0..=self.time)
            // hold_time is equal to speed, time racing is the total time minus hold time, multiply for total distance
            .filter(|hold_time| hold_time * (self.time - hold_time) > self.distance)
            .collect()
    }
}

fn parse(input: &str) -> Vec<Race> {
    let (time, distance) = input.split_once('\n').expect("Two lines");

    let to_num_vec = |input: &str| -> Vec<i32> {
        input
            .split_ascii_whitespace()
            .skip(1)
            .map(|s| s.parse().expect("number"))
            .collect()
    };

    to_num_vec(time)
        .into_iter()
        .zip(to_num_vec(distance))
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_parse() {
        assert_eq!(
            Race {
                time: 7,
                distance: 9
            },
            parse(WEB_EXAMPLE)[0]
        );
    }

    #[test]
    fn test_web() {
        assert_eq!(288, run(WEB_EXAMPLE));
    }
}
