use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};
    
    let pairs = parser!(lines(any_char+));
    let lists: Vec<Vec<char>> = pairs.parse(text).unwrap();
    lists
}



fn check(matrix: &Vec<Vec<char>>, row: i32, col: i32, dir: (i32, i32), expected: &Vec<char>, current: usize) -> u32 {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    if row < 0 || row >= rows || col < 0 || col >= cols {
        return 0;
    }
    if matrix[row as usize][col as usize] != expected[current] {
        return 0;
    }
    if current == expected.len() - 1 {
        return 1;
    }

    let mut ans = 0;
    ans += check(matrix, row + dir.0, col + dir.1, dir, &expected, current + 1);
    ans
}


fn check_path(matrix: &Vec<Vec<char>>, row: i32, col: i32, dir: (i32, i32), expected: &Vec<char>, current: usize, path: &mut Vec<(i32, i32)>) -> Option<(i32, i32)> {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    if row < 0 || row >= rows || col < 0 || col >= cols {
        return None;
    }
    if matrix[row as usize][col as usize] != expected[current] {
        return None;
    }
    if current == expected.len() - 1 {
        return Some(path[1]);
    }

    let mut ans = None;
    path.push((row + dir.0, col + dir.1));
    ans = check_path(matrix, row + dir.0, col + dir.1, dir, &expected, current + 1, path);
    path.pop();
    ans
}




#[aoc(day4, part1)]
pub fn part1(matrix: &Vec<Vec<char>>) -> u32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let expected = vec!['X', 'M', 'A', 'S'];
    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            let dirs = vec![-1, 0, 1];
            for dir1 in &dirs {
                for dir2 in &dirs {
                    if *dir1 == 0 && *dir2 == 0 {
                        continue;
                    }

                    ans += check(matrix, row as i32, col as i32, (*dir1, *dir2), &expected, 0);
                }
            }
        }
    }

    ans
}

#[aoc(day4, part2)]
pub fn part2(matrix: &Vec<Vec<char>>) -> usize {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let expected = vec!['M', 'A', 'S'];
    let mut a = HashMap::new();
    let mut path = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let dirs = vec![(-1, -1), (1, 1), (1, -1), (-1, 1)];
            for dir in dirs {
                path.push((row as i32, col as i32));
                if let Some(point) = check_path(matrix, row as i32, col as i32, dir, &expected, 0, &mut path) {
                    *a.entry(point).or_insert(0) += 1;
                }
                path.pop();
            }
        }
    }

    a.values().filter(|v| **v == 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 18);
    }

    #[test]
    fn part2_example() {
        let input = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(part2(&parse_input(input)), 9);
    }
}