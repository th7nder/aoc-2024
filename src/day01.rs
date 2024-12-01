use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(text: &str) -> (Vec<i32>, Vec<i32>) {
    use aoc_parse::{parser, prelude::*};
    
    let pairs = parser!(lines(i32 "   " i32));
    let lists: Vec<(i32, i32)> = pairs.parse(text).unwrap();
    lists.into_iter().unzip()
}


#[aoc(day1, part1)]
pub fn part1(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut left, mut right) = (lists.0.clone(), lists.1.clone());

    left.sort();
    right.sort();

    let total_min_distance: i32 = left.iter().zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    total_min_distance
}

#[aoc(day1, part2)]
pub fn part2(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (left, right) = (lists.0.clone(), lists.1.clone());

    let count_right = right.into_iter()
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    let total: i32 = left.into_iter()
        .map(|left_num| left_num * count_right.get(&left_num).unwrap_or(&0))
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 31);
    }
}