const INPUT: &str = include_str!("../../input/day01.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> u32 {
    input.lines().map(line_val).sum()
}

fn line_val(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn test_web() {
        let answers = [12, 38, 15, 77];
        for (line, answer) in std::iter::zip(WEB_EXAMPLE.lines(), answers) {
            assert_eq!(answer, line_val(line));
        }
        assert_eq!(142, run(WEB_EXAMPLE));
    }
}
