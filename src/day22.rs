use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
fn parse_input(text: &str) -> Vec<u64> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(u64));
    let map: Vec<u64> = pairs.parse(text).unwrap();

    map
}

fn round(secret_number: u64) -> u64 {
    let secret_number = (secret_number ^ (secret_number * 64)) % 16777216;
    let secret_number = (secret_number ^ (secret_number / 32)) % 16777216;
    let secret_number = (secret_number ^ (secret_number * 2048)) % 16777216;

    secret_number
}

#[aoc(day22, part1)]
fn part1(secret_numbers: &Vec<u64>) -> u64 {
    let mut secret_numbers = secret_numbers.clone();

    for _ in 0..2000 {
        for secret_number in secret_numbers.iter_mut() {
            *secret_number = round(*secret_number);
        }
    }


    secret_numbers.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sanity() {
        assert_eq!(15887950, round(123));
    }

    #[test]
    fn p1() {
        let input: Vec<u64> = vec![1, 10, 100, 2024];
        assert_eq!(37327623, part1(&input));
    }
}
