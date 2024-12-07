use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day7)]
fn parse_input(text: &str) -> Vec<(u128, Vec<u128>)> {
    use aoc_parse::{parser, prelude::*};
    
    let pairs = parser!(lines(u128 ": " repeat_sep(u128, " ")));
    let lists: Vec<(u128, Vec<u128>)> = pairs.parse(text).unwrap();
    lists
}

fn verify(test_value: u128, operands: &Vec<u128>, index: usize, current_value: u128) -> bool {
    if current_value == test_value && index == operands.len() {
        return true;
    }
    if current_value > test_value || index == operands.len() {
        return false;
    }

    verify(test_value, operands, index + 1, current_value + operands[index])
        || verify(test_value, operands, index + 1, current_value * operands[index]) 
}

fn times(num: u128) -> u128 {
    if num == 0 {
        return 10;
    }
    
    let zeros = (num as f64).log10().ceil() as u32;
    let res: u128 = 10u128.pow(zeros);

    if res == num {
        10u128.pow(zeros + 1)
    } else {
        res
    }
}

fn verify_concat(test_value: u128, operands: &Vec<u128>, index: usize, current_value: u128) -> bool {
    if current_value == test_value && index == operands.len() {
        return true;
    }
    if current_value > test_value || index == operands.len() {
        return false;
    }

    // let a: u128 = format!("{}{}", current_value, operands[index]).parse().unwrap();
    // let b = current_value * times(operands[index]) + operands[index];

    // if a != b {
    //     println!("a {} / b {} | {} || {}", a, b, current_value, operands[index]);
    // }

    verify_concat(test_value, operands, index + 1, current_value + operands[index])
        || verify_concat(test_value, operands, index + 1, current_value * operands[index]) 
        || verify_concat(test_value, operands, index + 1, 
            // format!("{}{}", current_value, operands[index]).parse().unwrap(),
            current_value * times(operands[index]) + operands[index]
        )
}


#[aoc(day7, part1)]
pub fn part1(equations: &Vec<(u128, Vec<u128>)>) -> u128 {
    let mut ans = 0;
    for (test_value, operands) in equations {
        // println!("checking: {} {:?}", test_value, operands);
        if verify(*test_value, &operands, 0, 0) {
            ans += test_value;
        }
    }

    ans
}

#[aoc(day7, part2)]
pub fn part2(equations: &Vec<(u128, Vec<u128>)>) -> u128 {
    let mut ans = 0;
    for (test_value, operands) in equations {
        if operands.len() == 1 {
            println!("woot");
        }
        // println!("checking: {} {:?}", test_value, operands);
        if verify_concat(*test_value, &operands, 0, 0) {
            ans += test_value;
        }
    }

    ans
}



#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3749);
    }

    #[test]
    fn weird() {
        let input = r"12345: 12 345";

        assert_eq!(part2(&parse_input(&input)), 12345);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 11387);
    }
}