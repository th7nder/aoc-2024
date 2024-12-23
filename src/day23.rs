use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
fn parse_input(text: &str) -> HashMap<String, Vec<String>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        lines(string(lower+) "-" string(lower+))
    );
    let map: Vec<(String, String)> = pairs.parse(text).unwrap();

    let mut graph = HashMap::new();
    for (left, right) in map {
        graph
            .entry(left.clone())
            .or_insert(Vec::new())
            .push(right.clone());
        graph.entry(right).or_insert(Vec::new()).push(left);
    }

    graph
}

fn dfs<'a>(
    graph: &'a HashMap<String, Vec<String>>,
    start: &'a String,
    current: &'a String,
    path_len: usize,
    visited: &mut HashSet<&'a String>,
    path: &mut Vec<&'a String>,
    paths: &mut Vec<Vec<&'a String>>,
) {
    if let Some(neighbours) = graph.get(current) {
        for n in neighbours {
            if path.len() == path_len {
                if n == start {
                    paths.push(path.clone());
                }
                continue;
            }

            if path.len() == path_len {
                continue;
            }

            if visited.contains(n) {
                continue;
            }

            visited.insert(n);
            path.push(n);
            dfs(graph, start, n, path_len, visited, path, paths);
            path.pop();
            visited.remove(n);
        }
    }
}

#[aoc(day23, part1)]
fn part1(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut ans = BTreeSet::new();

    let mut paths = Vec::new();
    for computer in graph.keys() {
        println!("{computer} -> {:?}", graph[computer]);
        let mut visited = HashSet::new();
        visited.insert(computer);
        let mut path = vec![computer];
        dfs(graph, computer, computer, 3, &mut visited, &mut path, &mut paths);
    }

    for mut path in paths {
        path.sort();
        let key = format!("{},{},{}", path[0], path[1], path[2]);
  
        if path[0].starts_with("t") || path[1].starts_with("t") || path[2].starts_with("t") {
            ans.insert(key);
        }
    }

    for a in &ans {
        println!("{}", a);
    }

    ans.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn p1() {
        assert_eq!(7, part1(&parse_input(TEST_INPUT)));
    }
}
