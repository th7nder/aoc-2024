use aoc_runner_derive::{aoc, aoc_generator};

struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    target: (usize, usize),
}

#[aoc_generator(day13)]
fn parse_input(text: &str) -> Vec<Machine> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(sections(
        line("Button A: X+" usize ", Y+" usize)
        line("Button B: X+" usize ", Y+" usize)
        line("Prize: X=" usize ", Y=" usize)
    ));
    let s: Vec<((usize, usize), (usize, usize), (usize, usize))> = pairs.parse(text).unwrap();

    let mut machines = vec![];
    for sect in s {
        machines.push(Machine {
            a: (sect.0 .0, sect.0 .1),
            b: (sect.1 .0, sect.1 .1),
            target: (sect.2 .0, sect.2 .1),
        })
    }

    machines
}

#[aoc(day13, part1)]
fn part1(machines: &Vec<Machine>) -> usize {
    let mut min_total = 0;
    for (i, machine) in machines.iter().enumerate() {
        let (ax, ay) = machine.a;
        let (bx, by) = machine.b;
        let (target_x, target_y) = machine.target;

        let mut min = None;
        for i in 0..=100 {
            for j in 0..=100 {
                if (ax * i) + (bx * j) == target_x && (ay * i + by * j) == target_y {
                    if min.is_none() {
                        min = Some(i * 3 + j);
                    } else {
                        min = Some(std::cmp::min(min.unwrap(), i * 3 + j));
                    }
                }
            }
        }

        if let Some(min) = min {
            min_total += min;
        }

        // println!("Machine {}, min: {:?}", i, min);
    }

    min_total
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn input() {
        let _ = parse_input(TEST_INPUT);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 480);
    }
}
