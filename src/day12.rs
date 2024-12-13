use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(any_char+));
    let lists: Vec<Vec<char>> = pairs.parse(text).unwrap();
    lists
}

fn fence(
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    region_id: char,
    (sr, sc): (i32, i32),
) -> (usize, usize) {
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
                continue;
            }
            if map[nr as usize][nc as usize] != region_id {
                perimeter += 1;
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

fn sides(
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(i32, i32)>,
    region_id: char,
    (sr, sc): (i32, i32),
) -> (usize, usize) {
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);

    let mut queue = VecDeque::new();
    queue.push_back((sr, sc));

    let mut area = 0;
    let mut sides = 0;
    while let Some((r, c)) = queue.pop_front() {
        area += 1;

        let different = |row: i32, col: i32| -> bool {
            if row < 0 || row >= rows || col < 0 || col >= cols {
                return true;
            }
            if map[row as usize][col as usize] != region_id {
                return true;
            }
            return false;
        };

        if different(r - 1, c) && different(r, c + 1) {
            sides += 1;
        }
        if different(r, c + 1) && !different(r + 1, c) && !different(r + 1, c + 1) {
            sides += 1;
        }
        if different(r - 1, c) && different(r, c - 1) {
            sides += 1;
        }
        if different(r, c - 1) && !different(r + 1, c) && !different(r + 1, c - 1) {
            sides += 1;
        }
        // TOP DONE!

        if different(r, c - 1) && different(r + 1, c) {
            sides += 1;
        }
        if different(r, c - 1) && !different(r - 1, c) && !different(r - 1, c - 1) {
            sides += 1;
        }
        if different(r, c + 1) && different(r + 1, c) {
            sides += 1;
        }
        if different(r, c + 1) && !different(r - 1, c) && !different(r - 1, c + 1) {
            sides += 1;
        }

        for (nr, nc) in vec![(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                continue;
            }
            if map[nr as usize][nc as usize] != region_id {
                continue;
            }

            if visited.contains(&(nr, nc)) {
                continue;
            }

            queue.push_back((nr, nc));
            visited.insert((nr, nc));
        }
    }

    (area, sides)
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

            let (area, perimeter) = fence(map, &mut visited, region, (row as i32, col as i32));
            // println!("Region: {}, area: {}, perimeter: {}", region, area, perimeter);
            ans += area * perimeter;
        }
    }

    ans
}

#[aoc(day12, part2)]
fn part2(map: &Vec<Vec<char>>) -> usize {
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

            let (area, sides) = sides(map, &mut visited, region, (row as i32, col as i32));
            println!(
                "Region: {}, area: {}, sides: {}, total, {}",
                region,
                area,
                sides,
                area * sides
            );
            ans += area * sides;
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
        assert_eq!(
            part1(&parse_input(
                r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            )),
            1930
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(
                r"AAAA
BBCD
BBCC
EEEC"
            )),
            80
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            part2(&parse_input(
                r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            )),
            436
        );
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            part2(&parse_input(
                r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            )),
            236
        );
    }

    #[test]
    fn part2_example4() {
        assert_eq!(
            part2(&parse_input(
                r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            )),
            368
        );
    }

    #[test]
    fn part2_example5() {
        assert_eq!(
            part2(&parse_input(
                r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            )),
            1206
        );
    }
}
