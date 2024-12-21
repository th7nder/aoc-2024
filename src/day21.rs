use std::{
    collections::{HashMap, HashSet, VecDeque},
    i64,
};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day21)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(any_char+));
    let map: Vec<Vec<char>> = pairs.parse(text).unwrap();

    map
}

fn numpad_neighbours(start: char) -> Vec<(char, char)> {
    match start {
        '0' => vec![('2', '^'), ('A', '>')],
        '1' => vec![('2', '>'), ('4', '^')],
        '2' => vec![('0', 'v'), ('1', '<'), ('3', '>'), ('5', '^')],
        '3' => vec![('2', '<'), ('6', '^'), ('A', 'v')],
        '4' => vec![('1', 'v'), ('5', '>'), ('7', '^')],
        '5' => vec![('2', 'v'), ('4', '<'), ('6', '>'), ('8', '^')],
        '6' => vec![('3', 'v'), ('5', '<'), ('9', '^')],
        '7' => vec![('4', 'v'), ('8', '>')],
        '8' => vec![('5', 'v'), ('7', '<'), ('9', '>')],
        '9' => vec![('6', 'v'), ('8', '<')],
        'A' => vec![('0', '<'), ('3', '^')],
        _ => unimplemented!("{}", start),
    }
}

fn dir_neighbours(start: char) -> Vec<(char, char)> {
    match start {
        '^' => vec![('A', '>'), ('v', 'v')],
        '<' => vec![('v', '>')],
        'v' => vec![('<', '<'), ('^', '^'), ('>', '>')],
        '>' => vec![('v', '<'), ('A', '^')],
        'A' => vec![('^', '<'), ('>', 'v')],
        _ => unimplemented!("{}", start),
    }
}

fn bfs(from: char, to: char, directional: bool) -> Vec<Vec<char>> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((from, vec![]));

    let mut res: Vec<Vec<char>> = vec![];

    while let Some((cur, mut path)) = queue.pop_front() {
        seen.insert(cur);

        if cur == to {
            if let Some(last_path) = res.last() {
                if path.len() > last_path.len() {
                    continue;
                }
            }

            path.push('A');
            res.push(path);
            continue;
        }

        let nbhs = if !directional {
            numpad_neighbours(cur)
        } else {
            dir_neighbours(cur)
        };

        for (n, direction) in nbhs {
            if seen.contains(&n) {
                continue;
            }
            let mut p = path.clone();
            p.push(direction);

            queue.push_back((n, p));
        }
    }

    res
}

fn dfs(
    s: Vec<char>,
    level: i64,
    directional: bool,
    cache: &mut HashMap<(Vec<char>, i64, bool), i64>,
) -> i64 {
    if cache.contains_key(&(s.clone(), level, directional)) {
        return *cache.get(&(s, level, directional)).unwrap();
    }

    let mut seq = vec!['A'];
    seq.extend(s.clone());

    let mut res = 0;
    for pair in seq.windows(2) {
        // println!("Checking pair: {:?}", pair);
        let from = pair[0];
        let to = pair[1];

        let paths = bfs(from, to, directional);
        // println!("Shortest paths: {:?}", paths);
        if level == 0 {
            res += paths.into_iter().map(|p| p.len() as i64).min().unwrap();
        } else {
            let mut x = i64::MAX;
            for path in paths {
                x = std::cmp::min(x, dfs(path, level - 1, true, cache));
            }
            res += x;
        }
    }

    cache.insert((s, level, directional), res);

    res
}

#[aoc(day21, part1)]
fn part1(codes: &Vec<Vec<char>>) -> i64 {
    let mut ans = 0;

    for code in codes {
        let mut cache = HashMap::new();
        let multiplier = String::from_iter(&code[..code.len() - 1])
            .parse::<i64>()
            .unwrap();
        ans += multiplier * dfs(code.clone(), 2, false, &mut cache);
    }

    ans
}

#[aoc(day21, part2)]
fn part2(codes: &Vec<Vec<char>>) -> i64 {
    let mut ans = 0;
    for code in codes {
        let mut cache = HashMap::new();
        let multiplier = String::from_iter(&code[..code.len() - 1])
            .parse::<i64>()
            .unwrap();
        ans += multiplier * dfs(code.clone(), 25, false, &mut cache);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"029A";

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 68 * 29);
    }

    // v<A<AA>^>AvA^<Av>A^Av<<A>^>AvA^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAA<Av>A^A, len: 70
    // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    // v<<A>^>A<A>A<AAv>A^Av<AAA^>A
    // <A^A>^^AvvvA
    // <A^A^^>AvvvA
    // 029A
}
