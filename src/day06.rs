use std::collections::{HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down, 
    Left
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1 , 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[aoc_generator(day6)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};
    
    let pairs = parser!(lines(any_char+));
    let lists: Vec<Vec<char>> = pairs.parse(text).unwrap();
    lists
}


fn walk(matrix: &Vec<Vec<char>>, pos: (i32, i32), mut dir: Direction, visited: &mut HashSet<(i32 ,i32)>) {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    visited.insert(pos);

    let mut next_pos = (pos.0 + dir.delta().0, pos.1 + dir.delta().1);
    if next_pos.0 < 0 || next_pos.0 >= rows || next_pos.1 < 0 || next_pos.1 >= cols {
        return;
    }

    if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
        dir = dir.turn();
        // this time we stay in place, can't get into the wall
        next_pos = pos;
    }

    walk(matrix, next_pos, dir, visited);
}


fn is_loop(matrix: &Vec<Vec<char>>, pos: (i32, i32), dir: Direction, visited_with_dirs: &mut HashSet<(i32, i32, Direction)>) -> bool {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    visited_with_dirs.insert((pos.0, pos.1, dir.clone()));

    let mut next_pos = (pos.0 + dir.delta().0, pos.1 + dir.delta().1);
    if next_pos.0 < 0 || next_pos.0 >= rows || next_pos.1 < 0 || next_pos.1 >= cols {
        visited_with_dirs.remove(&(pos.0, pos.1, dir.clone()));
        return false;
    }

    let mut next_dir = dir.clone();
    if matrix[next_pos.0 as usize][next_pos.1 as usize] == '#' {
        next_dir = next_dir.turn();
        // this time we stay in place, can't get into the wall
        next_pos = pos;
    }

    if visited_with_dirs.contains(&(next_pos.0, next_pos.1, next_dir.clone())) {
        visited_with_dirs.remove(&(pos.0, pos.1, dir.clone()));
        return true;
    }

    let is_l = is_loop(matrix, next_pos, next_dir.clone(), visited_with_dirs);
    visited_with_dirs.remove(&(pos.0, pos.1, dir.clone()));
    return is_l;
}


#[aoc(day6, part1)]
pub fn part1(matrix: &Vec<Vec<char>>) -> usize {
    let mut matrix = matrix.clone();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut start = (0, 0);
    let start_dir = Direction::Up;

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == '^' {
                start = (row as i32, col as i32);
                matrix[row][col] = '.';
                break;
            }
        }
    }

    let mut visited = HashSet::new();
    walk(&matrix, start, start_dir, &mut visited);


    visited.len()
}

#[aoc(day6, part2)]
pub fn part2(matrix: &Vec<Vec<char>>) -> usize {
    let mut matrix = matrix.clone();
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut start = (0, 0);
    let start_dir = Direction::Up;

    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == '^' {
                start = (row as i32, col as i32);
                matrix[row][col] = '.';
                break;
            }
        }
    }

    let mut path = HashSet::new();
    walk(&matrix, start, start_dir.clone(), &mut path);


    let mut ans = 0;
    let mut visited = HashSet::new();
    for entry in path {
        matrix[entry.0 as usize][entry.1 as usize] = '#';

        if is_loop(&matrix, start.clone(), start_dir.clone(), &mut visited) {
            ans += 1;
            // println!("Aaa");
        }

        matrix[entry.0 as usize][entry.1 as usize] = '.';
    }

    ans
}




#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 6);
    }
}