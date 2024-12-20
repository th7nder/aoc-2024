use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines(any_char+));
    let map: Vec<Vec<char>> = pairs.parse(text).unwrap();

    map
}

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 'S' {
                return (row as i32, col as i32);
            }
        }
    }

    unreachable!("map should always have a start");
}

// fn print(map: &Vec<Vec<char>>) {
//     let rows = map.len();
//     let cols = map[0].len();

//     for row in 0..rows {
//         for col in 0..cols {
//             print!("{}", map[row][col]);
//         }
//         println!();
//     }

//     println!();
// }

fn bfs_distances(map: &Vec<Vec<char>>) -> (i32, HashMap<(i32, i32), i32>) {
    let (r, c) = find_start(&map);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((r, c, 0));

    let mut distances = HashMap::new();

    while let Some((r, c, distance)) = queue.pop_front() {
        visited.insert((r, c));

        distances.insert((r, c), distance);

        if map[r as usize][c as usize] == 'E' {
            return (distance, distances);
        }

        for (nr, nc) in vec![(r + 1, c), (r - 1, c), (r, c - 1), (r, c + 1)] {
            if map[nr as usize][nc as usize] == '#' {
                continue;
            }
            if visited.contains(&(nr, nc)) {
                continue;
            }

            queue.push_back((nr, nc, distance + 1));
        }
    }

    unreachable!("asd")
}

#[aoc(day20, part1)]
fn part1(map: &Vec<Vec<char>>) -> i32 {
    let map = map.clone();
    let (benchmark, distances) = bfs_distances(&map);

    // for ((r, c), _) in &distances {
    //     map[*r as usize][*c as usize] = 'O';
    // }
    // print(&map);
    let mut ans = 0;

    const CHEAT: i32 = 2;
    for ((r, c), distance_to_end) in &distances {
        let distance_to_end = benchmark - *distance_to_end;
        for nr in r - CHEAT * 2..r + CHEAT * 2 {
            for nc in c - CHEAT * 2..c + CHEAT * 2 {
                if r.abs_diff(nr) + c.abs_diff(nc) <= CHEAT as u32 {
                    if !distances.contains_key(&(nr, nc)) {
                        continue;
                    }

                    let d = benchmark - *distances.get(&(nr, nc)).unwrap();
                    if d < distance_to_end {
                        let saves = distance_to_end - d - CHEAT;
                        if saves >= 100 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    ans
}

#[aoc(day20, part2)]
fn part2(map: &Vec<Vec<char>>) -> i32 {
    let map = map.clone();

    let (benchmark, distances) = bfs_distances(&map);

    // for ((r, c), _) in &distances {
    //     map[*r as usize][*c as usize] = 'O';
    // }
    // print(&map);
    // let mut t = BTreeMap::new();
    let mut ans = 0;

    const CHEAT: i32 = 20;
    for ((r, c), distance_to_end) in &distances {
        let distance_to_end = benchmark - *distance_to_end;
        for nr in r - CHEAT..r + CHEAT + 1 as i32 {
            for nc in c - CHEAT..c + CHEAT + 1 as i32 {
                let asd = r.abs_diff(nr) + c.abs_diff(nc);
                if asd <= CHEAT as u32 {
                    if !distances.contains_key(&(nr, nc)) {
                        continue;
                    }

                    let d = benchmark - *distances.get(&(nr, nc)).unwrap();
                    if d < distance_to_end {
                        let saves = distance_to_end - d - asd as i32;
                        if saves >= 100 {
                            ans += 1;
                            // *t.entry(saves).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 84);
    }

    #[test]
    fn part2_example_small() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 84);
    }
}
