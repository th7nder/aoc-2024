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
    paths: &mut HashSet<Vec<&'a String>>,
) {
    if let Some(neighbours) = graph.get(current) {
        for n in neighbours {
            if path.len() == path_len {
                let mut path = path.clone();
                path.sort();
                if n == start {
                    paths.insert(path);
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

    let mut paths = HashSet::new();
    for computer in graph.keys() {
        println!("{computer} -> {:?}", graph[computer]);
        let mut visited = HashSet::new();
        visited.insert(computer);
        let mut path = vec![computer];
        dfs(
            graph,
            computer,
            computer,
            3,
            &mut visited,
            &mut path,
            &mut paths,
        );
    }

    for path in paths {
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

fn search(
    graph: &HashMap<String, Vec<String>>,
    computer: String,
    required: BTreeSet<String>,
    paths: &mut HashSet<BTreeSet<String>>,
) {
    if paths.contains(&required) {
        return;
    }
    paths.insert(required.clone());

    let connections = graph.get(&computer).unwrap().clone();
    for neighbor in &connections {
        if required.contains(neighbor) {
            continue;
        }

        let mut contains = true;
        for requirement in required.iter() {
            if !graph.get(neighbor).unwrap().contains(requirement) {
                contains = false;
                break;
            }
        }
        if !contains {
            continue;
        }

        let mut required = required.clone();
        required.insert(neighbor.clone());
        search(graph, neighbor.clone(), required, paths);
    }
}

#[aoc(day23, part2)]
fn part2(graph: &HashMap<String, Vec<String>>) -> String {
    let mut paths = HashSet::new();
    for computer in graph.keys() {
        // println!("{computer} -> {:?}", graph[computer]);
        let mut set = BTreeSet::new();
        set.insert(computer.clone());
        search(graph, computer.clone(), set, &mut paths);
    }

    let (_, ans) = paths.into_iter().map(|s| (s.len(), s)).max().unwrap();

    ans.into_iter().collect::<Vec<_>>().join(",")
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

    #[test]
    fn p2() {
        assert_eq!("co,de,ka,ta", part2(&parse_input(TEST_INPUT)));
    }
}
