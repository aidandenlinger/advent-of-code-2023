use std::{
    cmp::max,
    collections::{HashSet, VecDeque},
};

const INPUT: &str = include_str!("../../input/day10.txt");

fn main() {
    println!("{}", run(INPUT));
}

fn run(input: &str) -> usize {
    let grid: Vec<Vec<Option<Pipe>>> = input
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| PipeDir::new(c).map(|dir| Pipe { dir, row, col }))
                .collect()
        })
        .collect();

    let start = grid
        .iter()
        .flatten()
        .find(|p| p.as_ref().map_or(false, |p| p.dir == PipeDir::Start))
        .expect("A starting point")
        .as_ref()
        .unwrap();

    // Now just BFS!
    let mut max_dist = 1;
    let mut queue: VecDeque<(usize, &Pipe)> =
        start.neighbor(&grid).into_iter().map(|p| (1, p)).collect();
    let mut visited: HashSet<&Pipe> = HashSet::from_iter(start.neighbor(&grid));
    visited.insert(start);

    while let Some((dist, pipe)) = queue.pop_front() {
        max_dist = max(max_dist, dist);

        for n in pipe.neighbor(&grid) {
            if !visited.contains(n) {
                queue.push_back((dist + 1, n));
                visited.insert(n);
            }
        }
    }

    max_dist
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct Pipe {
    dir: PipeDir,
    row: usize,
    col: usize,
}

#[derive(Hash, Debug, PartialEq, Eq)]
enum PipeDir {
    Vertical,
    Horizonal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start,
}

impl Pipe {
    fn neighbor<'a>(&'a self, grid: &'a Vec<Vec<Option<Pipe>>>) -> Vec<&Pipe> {
        let max_row = grid.len();
        let max_col = grid[0].len();

        let mut coords = vec![];
        // HORRIBLE. I'm just lazy right now
        // this should be inverted to calculate the four neighbors, then add them
        // depending on the pipe dir to avoid all this copy paste

        match self.dir {
            PipeDir::Vertical => {
                if let Some(row) = self.row.checked_sub(1) {
                    if (0..max_row).contains(&row) {
                        coords.push((row, self.col));
                    }
                }
                if (0..max_row).contains(&(self.row + 1)) {
                    coords.push((self.row + 1, self.col));
                }
            }
            PipeDir::Horizonal => {
                if let Some(col) = self.col.checked_sub(1) {
                    if (0..max_col).contains(&col) {
                        coords.push((self.row, col));
                    }
                }
                if (0..max_col).contains(&(self.col + 1)) {
                    coords.push((self.row, self.col + 1));
                }
            }
            PipeDir::NorthEast => {
                if let Some(row) = self.row.checked_sub(1) {
                    if (0..max_row).contains(&row) {
                        coords.push((row, self.col));
                    }
                }
                if (0..max_col).contains(&(self.col + 1)) {
                    coords.push((self.row, self.col + 1));
                }
            }
            PipeDir::NorthWest => {
                if let Some(row) = self.row.checked_sub(1) {
                    if (0..max_row).contains(&row) {
                        coords.push((row, self.col));
                    }
                }

                if let Some(col) = self.col.checked_sub(1) {
                    if (0..max_col).contains(&col) {
                        coords.push((self.row, col));
                    }
                }
            }
            PipeDir::SouthWest => {
                if (0..max_row).contains(&(self.row + 1)) {
                    coords.push((self.row + 1, self.col));
                }

                if let Some(col) = self.col.checked_sub(1) {
                    if (0..max_col).contains(&col) {
                        coords.push((self.row, col));
                    }
                }
            }
            PipeDir::SouthEast => {
                if (0..max_row).contains(&(self.row + 1)) {
                    coords.push((self.row + 1, self.col));
                }

                if (0..max_col).contains(&(self.col + 1)) {
                    coords.push((self.row, self.col + 1));
                }
            }
            PipeDir::Start => {
                if let Some(row) = self.row.checked_sub(1) {
                    if (0..max_row).contains(&row) {
                        coords.push((row, self.col));
                    }
                }
                if (0..max_row).contains(&(self.row + 1)) {
                    coords.push((self.row + 1, self.col));
                }
                if let Some(col) = self.col.checked_sub(1) {
                    if (0..max_col).contains(&col) {
                        coords.push((self.row, col));
                    }
                }
                if (0..max_col).contains(&(self.col + 1)) {
                    coords.push((self.row, self.col + 1));
                }

                return coords
                    .into_iter()
                    .flat_map(|(row, col)| &grid[row][col])
                    // Special case: Get the neighbors actually connected to start
                    .filter(|p| p.neighbor(grid).contains(&self))
                    .collect();
            }
        }

        coords
            .into_iter()
            .flat_map(|(row, col)| &grid[row][col])
            .collect()
    }
}

impl PipeDir {
    fn new(input: char) -> Option<Self> {
        match input {
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizonal),
            'L' => Some(Self::NorthEast),
            'J' => Some(Self::NorthWest),
            '7' => Some(Self::SouthWest),
            'F' => Some(Self::SouthEast),
            'S' => Some(Self::Start),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const WEB_EXAMPLE_2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn test_web() {
        assert_eq!(4, run(WEB_EXAMPLE));
    }

    #[test]
    fn test_web_2() {
        assert_eq!(8, run(WEB_EXAMPLE_2));
    }
}
