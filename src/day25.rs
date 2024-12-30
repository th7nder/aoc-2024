
use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day25)]
fn parse_input(text: &str) -> Vec<Vec<Vec<char>>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        sections(lines(any_char+))
    );
    let map: Vec<Vec<Vec<char>>> = pairs.parse(text).unwrap();

    map
}


#[aoc(day25, part1)]
fn part1(schematics: &Vec<Vec<Vec<char>>>) -> usize {

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in schematics {
        let is_lock = schematic[0].iter().filter(|c| **c == '#').count() == schematic[0].len();

        if is_lock {
            let mut combination = Vec::new();
            for col in 0..schematic[0].len() {
                let mut pin_height = 0;
                for row in 1..schematic.len() {
                    if schematic[row][col] == '#' {
                        pin_height += 1;
                    }
                }

                combination.push(pin_height);
            }

            locks.push(combination);
        } else {
            let mut combination = Vec::new();
            for col in 0..schematic[0].len() {
                let mut pin_height = 0;
                for row in 0..schematic.len() - 1 {
                    if schematic[row][col] == '#' {
                        pin_height += 1;
                    }
                }

                combination.push(pin_height);
            }

            keys.push(combination);
        }

    }

    // println!("{:?}", locks);
    // println!("{:?}", keys);

    let mut fits = 0;
    for key in keys {

        for lock in &locks {
            let mut good = true;
            for i in 0..lock.len() {
                if key[i] + lock[i] > 5 {
                    good = false;
                }
            }
            if good {
                fits += 1;
            }

        }
    }

    fits
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn p1() {
        assert_eq!(3, part1(&parse_input(TEST_INPUT)));
    }
}