use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
pub struct Mul {
    a: u32,
    b: u32,
    enabled: bool,
}

fn extract_from(re: &Regex, text: &str, enabled: bool) -> Vec<Mul> {
    let mut results = vec![];
    for (_, [_, a, b]) in re.captures_iter(text).map(|c| c.extract()) {
        results.push(Mul {
            enabled,
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
        });
    }

    return results;
}

#[aoc_generator(day3)]
fn parse_input(text: &str) -> Vec<Mul> {
    let re = Regex::new(r"(mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();

    let mut results = vec![];

    let mut enabled = true;

    let mut text = text;

    loop {
        let dont = text.find(if enabled { "don't()" } else { "do()" });
        match dont {
            Some(position) => {
                let found = extract_from(&re, &text[..position], enabled);
                results.extend(found);
                if enabled {
                    text = &text[position + 7..];
                } else {
                    text = &text[position + 4..];
                }
                enabled = !enabled;
            }
            None => {
                let found = extract_from(&re, text, enabled);
                results.extend(found);
                break;
            }
        }
    }

    results
}

#[aoc(day3, part1)]
pub fn part1(muls: &Vec<Mul>) -> u32 {
    muls.iter().map(|m| m.a * m.b).sum()
}

#[aoc(day3, part2)]
pub fn part2(muls: &Vec<Mul>) -> u32 {
    muls.iter().filter(|m| m.enabled).map(|m| m.a * m.b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str =
        r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn input() {
        let parsed = parse_input(TEST_INPUT);
        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 161);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
