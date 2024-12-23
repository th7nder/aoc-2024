use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day22)]
fn parse_input(text: &str) -> Vec<i64> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(i64));
    let map: Vec<i64> = pairs.parse(text).unwrap();

    map
}

fn round(secret_number: i64) -> i64 {
    let secret_number = (secret_number ^ (secret_number * 64)) % 16777216;
    let secret_number = (secret_number ^ (secret_number / 32)) % 16777216;
    let secret_number = (secret_number ^ (secret_number * 2048)) % 16777216;

    secret_number
}

#[aoc(day22, part1)]
fn part1(secret_numbers: &Vec<i64>) -> i64 {
    let mut secret_numbers = secret_numbers.clone();

    for _ in 0..2000 {
        for secret_number in secret_numbers.iter_mut() {
            *secret_number = round(*secret_number);
        }
    }

    secret_numbers.iter().sum()
}

#[aoc(day22, part2)]
fn part2(secret_numbers: &Vec<i64>) -> i64 {
    let mut secret_numbers = secret_numbers.clone();
    // let mut secret_numbers = vec![1, 2, 3, 2024];

    let mut sequences: Vec<Vec<i64>> = vec![vec![]; secret_numbers.len()];
    let mut previous_prices: Vec<i64> = secret_numbers.iter().map(|n| n % 10).collect();

    let mut sequence_to_price: Vec<HashMap<(i64, i64, i64, i64), i64>> =
        vec![HashMap::new(); secret_numbers.len()];

    let mut seen = HashSet::new();

    for _ in 0..2000 {
        for (seq_id, secret_number) in secret_numbers.iter_mut().enumerate() {
            *secret_number = round(*secret_number);
            let price = *secret_number % 10;

            sequences[seq_id].push(price - previous_prices[seq_id]);

            let l = sequences[seq_id].len();
            if l >= 4 {
                let [a, b, c, d] = &sequences[seq_id][l - 4..] else {
                    unreachable!("asd");
                };

                let tuple = (*a, *b, *c, *d);
                if !sequence_to_price[seq_id].contains_key(&tuple) {
                    sequence_to_price[seq_id].insert(tuple.clone(), price);
                    seen.insert(tuple);
                }
            }

            previous_prices[seq_id] = price;
        }
    }

    let mut max_score = 0;
    for tuple in seen {
        let mut score = 0;
        for seq_id in 0..secret_numbers.len() {
            if sequence_to_price[seq_id].contains_key(&tuple) {
                score += *sequence_to_price[seq_id].get(&tuple).unwrap();
            }
        }

        max_score = std::cmp::max(max_score, score);
    }

    max_score
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
        let input: Vec<i64> = vec![1, 10, 100, 2024];
        assert_eq!(37327623, part1(&input));
    }

    #[test]
    fn p2() {
        let input: Vec<i64> = vec![1];
        assert_eq!(37327623, part2(&input));
    }
}
