use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};
    
    let pairs = parser!(lines(any_char+));
    let lists: Vec<Vec<char>> = pairs.parse(text).unwrap();
    lists
}


#[aoc(day8, part1)]
pub fn part1(map: &Vec<Vec<char>>) -> usize {
    let mut antennas = HashMap::new();

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    for row in 0..rows {
        for col in 0..cols {
            let frequency = map[row as usize][col as usize];

            if frequency != '.'  && frequency != '#' {
                let frequency = map[row as usize][col as usize];
                antennas.entry(frequency).or_insert(Vec::new()).push((row, col));
            }
        }
    }

    let mut antinodes = HashSet::new();

    for row in 0..rows {
        for col in 0..cols {
            let frequency = map[row as usize][col as usize];

            if frequency == '.' || frequency == '#' {
                continue;
            }


            let positions = antennas.get(&frequency).unwrap();
            for (other_row, other_col) in positions {
                if *other_row == row && *other_col == col {
                    continue;
                }

                let dr = row - other_row;
                let dc = col - other_col;

                let antinode_row = row + 2 * (-dr);
                let antinode_col = col + 2 * (-dc);

                if antinode_row < 0 || antinode_row >= rows || antinode_col < 0 || antinode_col >= cols {
                    continue;
                }

                // println!("Antenna {}: {} {}, Antinode: {} {}", frequency, row, col, antinode_row, antinode_col);
                antinodes.insert((antinode_row, antinode_col));
            }
        }
    }


    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(map: &Vec<Vec<char>>) -> usize {
    let mut antennas = HashMap::new();

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    for row in 0..rows {
        for col in 0..cols {
            let frequency = map[row as usize][col as usize];

            if frequency != '.'  && frequency != '#' {
                let frequency = map[row as usize][col as usize];
                antennas.entry(frequency).or_insert(Vec::new()).push((row, col));
            }
        }
    }

    let mut antinodes = HashSet::new();

    for row in 0..rows {
        for col in 0..cols {
            let frequency = map[row as usize][col as usize];

            if frequency == '.' || frequency == '#' {
                continue;
            }


            let positions = antennas.get(&frequency).unwrap();
            for (other_row, other_col) in positions {
                if *other_row == row && *other_col == col {
                    continue;
                }

                let dr = row - other_row;
                let dc = col - other_col;

                for times in 1..1000 {
                    let antinode_row = row + times * (-dr);
                    let antinode_col = col + times * (-dc);
    
                    if antinode_row < 0 || antinode_row >= rows || antinode_col < 0 || antinode_col >= cols {
                        break;
                    }
    
                    antinodes.insert((antinode_row, antinode_col));
                }
            }
        }
    }


    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#..";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 34);
    }

}