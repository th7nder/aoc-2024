
use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(any_char+));
    let lists: Vec<Vec<char>> = pairs.parse(text).unwrap();
    lists
}


fn fence(map: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>, region_id: char, (sr, sc): (i32, i32)) -> (usize, usize) {
    let (rows, cols) = (map.len() as i32, map[0].len() as i32); 

    let mut queue = VecDeque::new();
    queue.push_back((sr, sc));

    let mut area = 0;
    let mut perimeter = 0;
    while let Some((r, c)) = queue.pop_front() {
        area += 1;

        for (nr, nc) in vec![(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                perimeter += 1;
                // TOOD: handle perimeter
                continue;
            }
            if map[nr as usize][nc as usize] != region_id {
                perimeter += 1;
                // TODO: handle perimeter
                continue;
            }

            if visited.contains(&(nr, nc)) {
                continue;
            }

            queue.push_back((nr, nc));
            visited.insert((nr, nc));
        }
    }

    (area, perimeter)
}

#[aoc(day12, part1)]
fn part1(map: &Vec<Vec<char>>) -> usize {

    let mut visited = HashSet::new();
    let (rows, cols) = (map.len(), map[0].len()); 

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            let region = map[row][col];
            if visited.contains(&(row as i32, col as i32)) {
                continue;
            }
            visited.insert((row as i32, col as i32));

            let (area, perimeter) = fence(map, &mut visited, region, (row as i32, col as i32 ));
            // println!("Region: {}, area: {}, perimeter: {}", region, area, perimeter);
            ans += area * perimeter;
        }
    }

    ans 
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 772);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE")), 1930);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse_input(TEST_INPUT)), 6);
    // }
}
