

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        lines(any_char+)
    );
    let map: Vec<Vec<char>> = pairs.parse(text).unwrap();

    map
}

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 'S' {
                return (row as i32, col as i32);
            }
        }
    }

    unreachable!("map should always have a start");
}

fn find_end(map: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 'E' {
                return (row as i32, col as i32);
            }
        }
    }

    unreachable!("map should always have a start");
}



fn print(map: &Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            print!("{}", map[row][col]);
        }
        println!();
    }

    println!();
}



fn bfs(map: &Vec<Vec<char>>) -> i32 {
    let (r, c) = find_start(&map);
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((r, c, 0));

    while let Some((r, c, distance)) = queue.pop_front() {
        visited.insert((r, c));

        if map[r as usize][c as usize] == 'E' {
            return distance;
        }

        for (nr, nc) in vec![(r + 1, c), (r - 1, c), (r, c - 1), (r, c + 1)] {
            if map[r as usize][c as usize] == '#' {
                continue;
            }
            if visited.contains(&(nr, nc)) {
                continue;
            }

            queue.push_back((nr, nc, distance + 1));
        }
    }

    unreachable!("cannot find any path")
}

#[aoc(day20, part1)]
fn part1(map: &Vec<Vec<char>>) -> i32 {   
    let mut map = map.clone();
    let (rows, cols) = (map.len(), map[0].len());

    let benchmark = bfs(&map);

    let mut ans = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if map[r][c] == '#' {
                map[r][c] = '.';
                let score = bfs(&map);
                if score < benchmark && benchmark - score >= 100 {
                    ans += 1;
                }
                map[r][c] = '#';
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 84);
    }
}