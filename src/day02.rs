use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse_input(text: &str) -> Vec<Vec<i32>> {
    use aoc_parse::{parser, prelude::*};
    
    let pairs = parser!(lines(repeat_sep(i32, " ")));
    let lists: Vec<Vec<i32>> = pairs.parse(text).unwrap();
    lists
}

#[aoc(day2, part1)]
pub fn part1(nums: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for nums in nums {
        let mut incrs = 0;
        let mut decrs = 0;

        for (i, num) in nums.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let res = *num - nums[i - 1];
            if res >= 1 && res <= 3 {
                incrs += 1;
            }
            if res <= -1 && res >= -3 {
                decrs += 1;
            }
        }   

        if incrs == nums.len() - 1 || decrs == nums.len() - 1 {
            ans += 1;
        }  
    }
    ans
} 

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut incrs = 0;
    let mut decrs = 0;

    for (i, num) in nums.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let res = *num - nums[i - 1];
        if res >= 1 && res <= 3 {
            incrs += 1;
        }
        if res <= -1 && res >= -3 {
            decrs += 1;
        }
    }   

    if incrs == nums.len() - 1 || decrs == nums.len() - 1 {
        return true;
    }  

    return false;

}

#[aoc(day2, part2)]
pub fn part2(nums: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for nums in nums {
        if is_safe(nums) {
            ans += 1;
            continue;
        }

        'asd: for not_selected in 0..nums.len() {
            let mut new_nums = Vec::new();

            for i in 0..nums.len() {
                if i != not_selected {
                    new_nums.push(nums[i]);
                }
            }

            if is_safe(&new_nums) {
                ans += 1;
                break 'asd;
            }
        }

    }
    ans
} 

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn input() {
        let parsed = parse_input(TEST_INPUT);
        assert_eq!(parsed.len(), 6);
        assert_eq!(parsed[0].len(), 5);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 4);
    }
}