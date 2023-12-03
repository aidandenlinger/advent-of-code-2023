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
        let mut scanned_num: Option<String> = None; // current number we're scanning
        let mut part_num = false; // is this a part number

        for (col_num, char) in row.iter().enumerate() {
            if char.is_ascii_digit() {
                scanned_num = match scanned_num {
                    Some(mut num_so_far) => {
                        num_so_far.push(*char);
                        Some(num_so_far)
                    }
                    None => Some(char.to_string()),
                };

                if !part_num {
                    part_num = touching_symbol(&grid, row_num, col_num);
                };
            } else {
                // char *isn't* ascii digit, see if we finished scanning a num
                if let Some(ref num_so_far) = scanned_num {
                    if part_num {
                        sum += num_so_far.parse::<u32>().unwrap();
                    }
                    // reset
                    scanned_num = None;
                    part_num = false;
                }
            }
        }

        // end of row, if we have anything stored add it
        if let Some(ref num_so_far) = scanned_num {
            if part_num {
                sum += num_so_far.parse::<u32>().unwrap();
            }
        }
    }

    sum
}

fn touching_symbol(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let mut neighbors: Vec<char> = vec![];

    for r in row.saturating_sub(1)..=row + 1 {
        if r >= grid.len() {
            continue;
        }
        for c in col.saturating_sub(1)..=col + 1 {
            if c >= grid[0].len() || (r == row && c == col) {
                continue;
            }
            neighbors.push(grid[r][c]);
        }
    }

    neighbors
        .into_iter()
        .any(|c| !c.is_ascii_digit() && c != '.')
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
        assert_eq!(4361, run(WEB_EXAMPLE));
    }

    #[test]
    fn test_left_boundary() {
        assert_eq!(10, run("10#."));
    }

    #[test]
    fn test_right_boundary() {
        // forgot to check the buffers at the end of a row
        assert_eq!(10, run(".#10"));
    }

    #[test]
    fn test_up_boundary() {
        assert_eq!(10, run("...#\n..10"));
    }

    #[test]
    fn test_down_boundary() {
        assert_eq!(10, run("..10\n...#"));
    }
}
