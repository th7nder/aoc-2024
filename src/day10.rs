use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(text: &str) -> Vec<Vec<usize>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(digit+));
    let lists: Vec<Vec<usize>> = pairs.parse(text).unwrap();
    lists
}

fn explore(map: &Vec<Vec<usize>>, start: (i32, i32)) -> usize {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut score = HashSet::new();
    while let Some((row, col)) = queue.pop_front() {
        if map[row as usize][col as usize] == 9 {
            score.insert((row, col));
            continue;
        }

        let nexts = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];
        for (nr, nc) in nexts {
            if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                continue;
            }
            if map[nr as usize][nc as usize] as i32 - (map[row as usize][col as usize] as i32) != 1
            {
                continue;
            }

            queue.push_back((nr, nc));
        }
    }

    score.len()
}

fn explore2(map: &Vec<Vec<usize>>, start: (i32, i32)) -> usize {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut score = 0;
    while let Some((row, col)) = queue.pop_front() {
        if map[row as usize][col as usize] == 9 {
            score += 1;
            continue;
        }

        let nexts = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];
        for (nr, nc) in nexts {
            if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                continue;
            }
            if map[nr as usize][nc as usize] as i32 - (map[row as usize][col as usize] as i32) != 1
            {
                continue;
            }

            queue.push_back((nr, nc));
        }
    }

    score
}

#[aoc(day10, part1)]
fn part1(map: &Vec<Vec<usize>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 0 {
                ans += explore(map, (row as i32, col as i32));
            }
        }
    }

    ans
}

#[aoc(day10, part2)]
fn part2(map: &Vec<Vec<usize>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 0 {
                ans += explore2(map, (row as i32, col as i32));
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 81);
    }
}
