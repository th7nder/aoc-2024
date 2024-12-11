use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn parse_input(text: &str) -> Vec<u64> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(line(repeat_sep(u64, " ")));
    let lists: Vec<u64> = pairs.parse(text).unwrap();
    lists
}

fn digits(mut pebble: u64) -> Vec<u8> {
    let mut digits = Vec::with_capacity(20);

    while pebble > 0 {
        digits.push((pebble % 10) as u8);

        pebble /= 10;
    }

    digits
}

fn from_digits(digits: &[u8], len: usize) -> u64 {
    let mut n: u64 = 0;

    let mut i = (len as i64) - 1;
    while i >= 0 {
        n *= 10;
        n += digits[i as usize] as u64;
        i -= 1;
    }

    n
}

fn blink(pebbles: Vec<u64>) -> Vec<u64> {
    let mut new_pebbles = Vec::new();

    for pebble in pebbles {
        let digits = digits(pebble);
        if pebble == 0 {
            new_pebbles.push(1);
        } else if digits.len() % 2 == 0 {
            new_pebbles.push(from_digits(&digits[digits.len() / 2..], digits.len() / 2));
            new_pebbles.push(from_digits(&digits[0..digits.len() / 2], digits.len() / 2));
        } else {
            new_pebbles.push(pebble * 2024);
        }
    }
    new_pebbles
}

#[aoc(day11, part1)]
fn part1(pebbles: &Vec<u64>) -> usize {
    let mut pebbles = pebbles.clone();

    for _ in 0..25 {
        pebbles = blink(pebbles);
        // println!("Pebbles: {:?}", pebbles);
    }

    pebbles.len()
}

fn count(stone: u64, steps: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }
    if cache.contains_key(&(stone, steps)) {
        return cache[&(stone, steps)];
    }

    let res = if stone == 0 {
        count(1, steps - 1, cache)
    } else {
        let digits = digits(stone);
        if digits.len() % 2 == 0 {
            count(
                from_digits(&digits[digits.len() / 2..], digits.len() / 2),
                steps - 1,
                cache,
            ) + count(
                from_digits(&digits[0..digits.len() / 2], digits.len() / 2),
                steps - 1,
                cache,
            )
        } else {
            count(stone * 2024, steps - 1, cache)
        }
    };

    cache.insert((stone, steps), res);
    res
}

#[aoc(day11, part2)]
fn part2(pebbles: &Vec<u64>) -> u64 {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for stone in pebbles {
        sum += count(*stone, 75, &mut cache);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 65601038650482);
    }
}

// every 1, after 4 iterations, gives back 2024 + 6 numbers
