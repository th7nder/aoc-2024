
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
fn parse_input(text: &str) -> (u64, u64, u64, Vec<u64>) {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        section(line("Register A: " u64)
            line("Register B: " u64)
            line("Register C: " u64)
        )
        section(
            line("Program: " repeat_sep(u64, ","))
        )
    );

    let ((a, b, c), program) = pairs.parse(text).unwrap();

    
    (a, b, c, program)
}

fn combo(operand: u64, a: &u64, b: &u64, c: &u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => *a,
        5 => *b,
        6 => *c,
        _ => unimplemented!("don't be here")
    }
}

#[aoc(day17, part1)]
fn part1((a, b, c, program): &(u64, u64, u64, Vec<u64>)) -> String {
    let (mut a, mut b, mut c) = (*a, *b, *c);

    let mut out = Vec::new();

    let mut op = 0;
    while op < program.len() {
        let instruction = program[op];
        let operand = program[op + 1];

        match instruction {
            0 => {
                a = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            },
            1 => {
                b = b ^ operand;
            },
            2 => {
                b = combo(operand, &a, &b, &c) % 8;
            },
            3 => {
                if a != 0 {
                    op = operand as usize;
                    continue;
                }
            },
            4 => {
                b = b ^ c;
            },
            5 => {
                out.push(combo(operand, &a, &b, &c) % 8);
            },
            6 => {
                b = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            },
            7 => {
                c = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            }
            _ => unreachable!("i'm stupid.")
        }

        op += 2;
    }

    out.into_iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",")
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), "4,6,3,5,6,3,5,2,1,0");
    }
}
