use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
fn parse_input(text: &str) -> (Vec<String>, Vec<String>) {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        section(string(any_char+))
        section(lines(string(any_char+)))
    );
    let (patterns, designs): (String, Vec<String>) = pairs.parse(text).unwrap();

    let mut p = Vec::new();
    for pattern in patterns.trim().split(", ") {
        p.push(pattern.to_string());
    }

    (p, designs)
}

fn possible(design: &String, patterns: &Vec<String>, design_index: usize) -> bool {
    if design_index == design.len() {
        return true;
    }

    for pattern in patterns {
        if design[design_index..].starts_with(pattern) {
            if possible(design, patterns, design_index + pattern.len()) {
                return true;
            }
        }
    }

    false
}



fn ways(design: &String, patterns: &Vec<String>, design_index: usize, cache: &mut HashMap<usize, usize> ) -> usize {
    if design_index == design.len() {
        return 1;
    }

    // cache stores whether how many ways we can construct a rest of the work at the design index 
    if cache.contains_key(&design_index) {
        return *cache.get(&design_index).unwrap();
    } 

    let mut w = 0;
    for pattern in patterns {
        if design_index + pattern.len() > design.len() {
            continue;
        }
        let part = &design[design_index..design_index + pattern.len()];
        if part == pattern {
            w += ways(design, patterns, design_index + pattern.len(), cache);
        } 
    }

    cache.insert(design_index, w);

    w
}


#[aoc(day19, part1)]
fn part1((patterns, designs): &(Vec<String>, Vec<String>)) -> usize {
    let mut ans = 0;
    for design in designs {
        if possible(design, patterns, 0) {
            ans += 1;
        }
    }

    ans
}

#[aoc(day19, part2)]
fn part2((patterns, designs): &(Vec<String>, Vec<String>)) -> usize {
    let mut ans = 0;
    for design in designs {
        println!("Checking: {design}");
        let mut cache = HashMap::new();
        ans += ways(design, patterns, 0, &mut cache);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";


    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 16);
    }
}