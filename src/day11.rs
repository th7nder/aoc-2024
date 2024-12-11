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

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"125 17";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 55312);
    }
}