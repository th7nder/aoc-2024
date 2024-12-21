use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use rand::seq::SliceRandom;


#[aoc_generator(day21)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(any_char+));
    let map: Vec<Vec<char>> = pairs.parse(text).unwrap();

    map
}

fn search(start: (i32, i32), target: char, grid: &Vec<Vec<char>>) -> Vec<((i32, i32), Vec<char>)> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((start.0, start.1, vec![]));

    let mut paths: Vec<((i32, i32), Vec<char>)> = Vec::new();

    while let Some((r, c, path)) = queue.pop_front() {
        seen.insert((r, c));

        if grid[r as usize][c as usize] == target {
            if let Some((_, last_path)) = paths.last() {
                if path.len() > last_path.len() {
                    break;
                }
            }
            paths.push(((r, c), path));
            continue;
        }

        for (nr, nc, x) in vec![(r + 1, c, 'v'), (r - 1, c, '^'), (r, c + 1, '>'), (r, c - 1, '<')] {
            if nr < 0 || nc < 0 || nr >= rows || nc >= cols {
                continue;
            }
            if grid[nr as usize][nc as usize] == 'X' {
                continue;
            }

            let mut p = path.clone();
            p.push(x);
            queue.push_back((nr, nc, p));
        }
    }

    paths
}

fn type_on_grid(grid: &Vec<Vec<char>>, mut initial_pos: (i32, i32), code: &Vec<char>) -> Vec<char> {
    // let mut final_paths = Vec::new();

    let mut final_path = Vec::new();
    for symbol in code {
        let paths = search(initial_pos, *symbol, &grid);
        for (next_initial_pos, path) in paths {
            final_path.extend(path);
            final_path.push('A');
            initial_pos = next_initial_pos;
        }
    }

    final_path
}

fn all_paths_for_code(grid: &Vec<Vec<char>>, code: &Vec<char>, index: usize, initial_pos: (i32, i32), current_path: Vec<char>, final_paths: &mut Vec<Vec<char>>) {
    if let Some(last) = final_paths.last() {
        if last.len() < current_path.len() {
            return;
        }
    }

    if index == code.len() {
        final_paths.push(current_path);
        return;
    }

    let symbol = code[index];
    let paths = search(initial_pos, symbol, grid);
    for (next_initial_pos, path) in paths {
        let mut current_path = current_path.clone();
        current_path.extend(path);
        current_path.push('A');
        all_paths_for_code(grid, code, index + 1, next_initial_pos, current_path, final_paths);
    }
}

#[aoc(day21, part1)]
fn part1(codes: &Vec<Vec<char>>) -> i32 {
    let numeric_grid = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['X', '0', 'A'],
    ];
    let numeric_grid_start = (3, 2);
    let directional_grid = vec![
        vec!['X', '^', 'A'],
        vec!['<', 'v', '>'],
    ];
    let directional_grid_start = (0, 2);

    let mut rng = rand::thread_rng();

    let mut ans = 0;
    for code in codes {
        let mut first_paths = Vec::new();
        all_paths_for_code(&numeric_grid, code, 0, numeric_grid_start, Vec::new(), &mut first_paths);

        let mut second_paths = Vec::new();
        for first_path in &first_paths {
            all_paths_for_code(&directional_grid, &first_path, 0, directional_grid_start, Vec::new(), &mut second_paths);
        }

        let second_paths: Vec<Vec<char>> = second_paths.choose_multiple(&mut rng, 20).cloned().collect();

        let mut third_paths = Vec::new();
        for second_path in &second_paths {
            all_paths_for_code(&directional_grid, &second_path, 0, directional_grid_start, Vec::new(), &mut third_paths);
        }

        
        let min_path = third_paths.iter().map(|p| p.len()).min().unwrap() as i32;
        let multiplier = String::from_iter(&code[..code.len() - 1]).parse::<i32>().unwrap();
        ans += min_path * multiplier;
        println!("Min path: {}, mult: {}", min_path, multiplier);
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"029A";

    #[test]
    fn sanity1() {
        let grid = vec![
            vec!['7', '8', '9'],
            vec!['4', '5', '6'],
            vec!['1', '2', '3'],
            vec!['X', '0', 'A'],
        ];
        let numeric_grid_start = (3, 2);
        let code = vec!['0', '2', '9', 'A'];

        let mut paths = Vec::new();
        all_paths_for_code(&grid, &code, 0, numeric_grid_start, Vec::new(), &mut paths);
        for path in paths {
            println!("{}", path.iter().collect::<String>())
        }
        
    }

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 68 * 29);
    }


    // v<A<AA>^>AvA^<Av>A^Av<<A>^>AvA^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAA<Av>A^A, len: 70
    // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    // v<<A>^>A<A>A<AAv>A^Av<AAA^>A
    // <A^A>^^AvvvA
    // <A^A^^>AvvvA
    // 029A

}