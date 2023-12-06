use std::ops::RangeInclusive;

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
    parse(input).win_options().count()
}

impl Race {
    #[allow(clippy::cast_possible_truncation)] // We don't care about the decimals when we're truncating
    fn win_options(&self) -> RangeInclusive<i64> {
        // (0..=self.time)
        //     .filter(|hold_time| hold_time * (self.time - hold_time) > self.distance)
        //     .collect()

        // Part 1 code above to show that we're solving x * (self.time - x) > self.distance
        // where x is hold_time
        // We can rewrite to -x^2 + (self.time)x - distance > 0
        // it's a quadratic!
        // a = -1, b = self.time, c = -distance
        // (-b +/- sqrt(b^2 - 4ac)) / 2a

        // I hate casting, but it seems to be the easiest solution here. The parsed
        // numbers won't fit in an i32 so it needs to be i64
        let a: f64 = -1.0;
        let b = self.time as f64;
        let c = -self.distance as f64;

        let discriminant = b.powf(2.0) - (4.0 * a * c);
        let sol_1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let sol_2 = (-b + discriminant.sqrt()) / (2.0 * a);

        // We first convert to an integer with ceil/floor, then
        // cast. Again, would prefer to not cast, but it's not
        // offered (which makes sense on a type level)

        let (smaller, bigger) = if sol_1 < sol_2 {
            (sol_1, sol_2)
        } else {
            (sol_2, sol_1)
        };

        (smaller.ceil() as i64)..=(bigger.floor() as i64)
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
