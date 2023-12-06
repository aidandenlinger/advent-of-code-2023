const INPUT: &str = include_str!("../../input/template.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "";

    #[test]
    fn test_web() {
        assert_eq!(0, run(WEB_EXAMPLE));
    }
}
