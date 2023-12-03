use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day03.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    // Go through the grid, character by character. When we see a digit,
    // start building a number and detect if there are any symbols around
    // each character. When full number and part digit, add it to this sum
    let mut sum = 0;

    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, char) in row.iter().enumerate() {
            if *char == '*' {
                let n = gear_neighbors(&grid, row_num, col_num);
                if n.len() == 2 {
                    sum += n[0] * n[1];
                }
            }
        }
    }

    sum
}

fn gear_neighbors(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<u32> {
    let mut neighbors: Vec<(usize, usize)> = vec![];

    for r in row.saturating_sub(1)..=row + 1 {
        if r >= grid.len() {
            continue;
        }
        for c in col.saturating_sub(1)..=col + 1 {
            if c >= grid[0].len() || (r == row && c == col) {
                continue;
            }
            neighbors.push((r, c));
        }
    }

    let mut numbers = vec![];
    let mut added_points = HashSet::new();

    for (r, c) in neighbors {
        if !added_points.contains(&(r, c)) && grid[r][c].is_ascii_digit() {
            let mut c_left = c;
            while c_left > 0 && grid[r][c_left - 1].is_ascii_digit() {
                c_left -= 1;
            }

            let mut c_right = c;
            while c_right + 1 < grid[r].len() && grid[r][c_right + 1].is_ascii_digit() {
                c_right += 1;
            }

            for y in c_left..=c_right {
                added_points.insert((r, y));
            }

            let num_str: String = grid[r][c_left..=c_right].iter().collect();

            numbers.push(num_str.parse().unwrap());
        }
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_web() {
        assert_eq!(467_835, run(WEB_EXAMPLE));
    }

    #[test]
    fn test_gear_neighbors() {
        assert_eq!(
            vec![467, 35],
            gear_neighbors(
                &"467..\n...*.\n..35."
                    .lines()
                    .map(|s| s.chars().collect())
                    .collect(),
                1,
                3
            )
        );
    }
}
