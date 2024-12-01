use std::{collections::HashMap, fs::File, io};


fn load_lists(path: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let lines: io::Lines<io::BufReader<File>> = crate::read_lines(path)
        .unwrap();

    for line in lines {
            if let Ok(line) = line {
                let (l, r) = line.split_once(' ').unwrap();
                left.push(l.parse().unwrap());
                right.push(r.parse().unwrap());
            }
    }

    (left, right)
}

pub fn part1(path: &str) {
    let (mut left, mut right) = load_lists(path);
    left.sort();
    right.sort();

    let total_min_distance: u64 = left.iter().zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("Part1: {}", total_min_distance);
}

pub fn part2(path: &str) {
    let  (left, right) = load_lists(path);

    let count_right = right.into_iter()
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    let total: u64 = left.into_iter()
        .map(|left_num| left_num * count_right.get(&left_num).unwrap_or(&0))
        .sum();

    println!("Part2: {}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        part1("input/1_small.txt");
        part1("input/1.txt");
    }

    #[test]
    fn p2() {
        part2("input/1_small.txt");
        part2("input/1.txt");
    }
}