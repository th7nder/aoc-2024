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
}