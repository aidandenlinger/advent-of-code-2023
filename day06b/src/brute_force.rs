const INPUT: &str = include_str!("../../input/day06.txt");

fn main() {
    println!("{}", run(INPUT));
}

#[derive(PartialEq, Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn run(input: &str) -> usize {
    parse(input).win_options().len()
}

impl Race {
    fn win_options(&self) -> Vec<i64> {
        (0..=self.time)
            // hold_time is equal to speed, time racing is the total time minus hold time, multiply for total distance
            .filter(|hold_time| hold_time * (self.time - hold_time) > self.distance)
            .collect()
    }
}

fn parse(input: &str) -> Race {
    let (time, distance) = input.split_once('\n').expect("Two lines");

    let to_num = |input: &str| -> i64 {
        input
            .split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("number")
    };

    let time = to_num(time);
    let distance = to_num(distance);

    Race { time, distance }
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
                time: 71530,
                distance: 940_200
            },
            parse(WEB_EXAMPLE)
        );
    }

    #[test]
    fn test_web() {
        assert_eq!(71503, run(WEB_EXAMPLE));
    }
}
