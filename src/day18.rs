use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet, VecDeque}};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day18)]
fn parse_input(text: &str) -> Vec<(usize, usize)> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        lines(usize "," usize)
    );
    let map: Vec<(usize, usize)> = pairs.parse(text).unwrap();

    map
}

fn grid(bytes: &Vec<(usize, usize)>, num_bytes: usize, size: usize) -> Vec<Vec<char>> {
    let rows = size;
    let cols = size;
    let mut grid = vec![vec!['.'; cols]; rows];
    for (col, row) in &bytes[..num_bytes] {
        let (row, col) = (*row, *col);
        grid[row][col] = '#';
    }

    grid
}

fn print(grid: &Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            print!("{}", grid[row][col]);
        }
        println!();
    }
}


fn path(grid: &Vec<Vec<char>>) -> i32 {
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0 as i32), 0 as i32, 0 as i32));

    let mut distances = HashMap::new();
    let mut seen = HashSet::new();

    while let Some((distance, r, c)) = queue.pop() {
        seen.insert((r, c));
        if r == rows - 1 && c == cols - 1 {
            return distance.0;
        }

        for (nr, nc) in vec![(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                continue;
            }
            if grid[nr as usize][nc as usize] == '#' {
                continue;
            }
            if seen.contains(&(nr, nc)) {
                continue;
            }

            if distance.0 + 1 < *distances.get(&(nr, nc)).unwrap_or(&i32::MAX) {
                distances.insert((nr, nc), distance.0 + 1);

                queue.push((Reverse(distance.0 + 1), nr, nc));
            }
        }
    }

    unreachable!("didn't find path");
}

#[aoc(day18, part1)]
fn part1(bytes: &Vec<(usize, usize)>) -> i32 {
    let grid = grid(bytes, 1024, 71);

    path(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_example_small() {
        let bytes = parse_input(TEST_INPUT);
        let grid = grid(&bytes, 12, 7);
        print(&grid);

        assert_eq!(path(&grid), 22);
    }
}