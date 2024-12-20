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
        _ => unimplemented!("don't be here"),
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
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                b = combo(operand, &a, &b, &c) % 8;
            }
            3 => {
                if a != 0 {
                    op = operand as usize;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                out.push(combo(operand, &a, &b, &c) % 8);
            }
            6 => {
                b = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            }
            7 => {
                c = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            }
            _ => unreachable!("i'm stupid."),
        }

        op += 2;
    }

    out.into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn single_program(a: u64, program: &Vec<u64>) -> u64 {
    let (mut a, mut b, mut c) = (a, 0, 0);

    let mut op = 0;
    while op < program.len() {
        let instruction = program[op];
        let operand = program[op + 1];

        match instruction {
            0 => {
                a = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                b = combo(operand, &a, &b, &c) % 8;
            }
            3 => {
                if a != 0 {
                    op = operand as usize;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                return combo(operand, &a, &b, &c) % 8;
            }
            6 => {
                b = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            }
            7 => {
                c = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
            }
            _ => unreachable!("i'm stupid."),
        }

        op += 2;
    }

    unreachable!("xddd")
}

fn try_sth(a: u64, whole_program: &Vec<u64>, index: usize, results: &mut Vec<u64>) {
    let expected_output = whole_program[index];
    // println!("Expected bits: {}", expected_output);
    for t in 0..8 {
        let output = single_program(a | t, whole_program);
        if output == expected_output {
            // println!("found last bits: {}", t);
            let mut a = a | t;
            if index == 0 {
                println!("FOUND A: {a}");
                results.push(a);
                return;
            }

            a <<= 3;
            try_sth(a, whole_program, index - 1, results);
        }
    }
}

#[aoc(day17, part2)]
fn part2((_, _, _, program): &(u64, u64, u64, Vec<u64>)) -> String {
    let a = 0;
    let mut results = Vec::new();
    try_sth(a, &program, program.len() - 1, &mut results);

    println!("{a}");

    for a in results {
        let p1 = part1(&(a, 0, 0, program.clone()));
        println!("a:{a}, P1: {p1}");
    }

    String::new()
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

    #[test]
    fn part2_example_small() {
        debug_print(&parse_input(
            r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        ));

        assert_eq!(
            part2(&parse_input(
                r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            )),
            "0,3,5,4,3,0"
        );
    }

    fn debug_print((a, b, c, program): &(u64, u64, u64, Vec<u64>)) -> String {
        let (mut a, mut b, mut c) = (*a, *b, *c);

        let mut out = Vec::new();

        let mut op = 0;
        while op < program.len() {
            let instruction = program[op];
            let operand = program[op + 1];

            match instruction {
                0 => {
                    a = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
                    println!("A = A / 2^{}", combo_str(operand, &a, &b, &c))
                }
                1 => {
                    b = b ^ operand;
                    println!("B = B XOR {}", operand)
                }
                2 => {
                    b = combo(operand, &a, &b, &c) % 8;
                    println!("B = {} % 8", combo_str(operand, &a, &b, &c))
                }
                3 => {
                    println!("if A != 0");
                    println!("    goto: {}", operand);
                    // if a != 0 {
                    //     op = operand as usize;
                    //     continue;
                    // }
                }
                4 => {
                    b = b ^ c;
                    println!("B = B XOR C");
                }
                5 => {
                    out.push(combo(operand, &a, &b, &c) % 8);
                    println!("out({} % 8)", combo_str(operand, &a, &b, &c))
                }
                6 => {
                    b = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
                    println!("B = A / 2^{}", combo_str(operand, &a, &b, &c))
                }
                7 => {
                    c = a / 2u64.pow(combo(operand, &a, &b, &c).try_into().expect("< u64"));
                    println!("C = A / 2^{}", combo_str(operand, &a, &b, &c))
                }
                _ => unreachable!("i'm stupid."),
            }

            op += 2;
        }

        out.into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn combo_str(operand: u64, a: &u64, b: &u64, c: &u64) -> String {
        match operand {
            0..=3 => operand.to_string(),
            4 => "A".to_string(),
            5 => "B".to_string(),
            6 => "C".to_string(),
            _ => unimplemented!("don't be here"),
        }
    }
}
