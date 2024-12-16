
use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet, VecDeque}};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
fn parse_input(text: &str) -> Vec<Vec<char>> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        lines(any_char+)
    );
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

fn find_end(map: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 'E' {
                return (row as i32, col as i32);
            }
        }
    }

    unreachable!("map should always have a start");
}



fn print(map: &Vec<Vec<char>>) {
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            print!("{}", map[row][col]);
        }
        println!();
    }

    println!();
}

fn dijkstra(map: &Vec<Vec<char>>) -> (i32, Vec<Vec<(i32, i32)>>) {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let (r, c) = find_start(&map);
    
    println!("Initial state ({r}) ({c}):");
    print(&map);

    const EAST: (i32, i32) = (0, 1);

    let mut distances = Vec::new();
    for _ in 0..rows {
        let mut column = Vec::new();
        for _ in 0..cols {
            column.push(i32::MAX);
        }
        distances.push(column);
    }
    distances[r as usize][c as usize] = 0;

    let mut parents = Vec::new();
    for _ in 0..rows {
        let mut column = Vec::new();
        for _ in 0..cols {
            column.push((0, 0));
        }
        parents.push(column);
    }

    distances[r as usize][c as usize] = 0;

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), (r, c), EAST));

    let mut seen = HashSet::new();

    while let Some((distance, pos, direction)) = queue.pop() {
        seen.insert((pos, direction));

        // println!("Visting: {:?}", pos);

        if map[pos.0 as usize][pos.1 as usize] == 'E' {
            println!("Found it!");
            return (distance.0, parents);
        }

        let rotate_left = (-direction.1, direction.0);
        let rotate_right = (direction.1, -direction.0);
        
        for (nr, nc, dir, next_distance) in vec![
            (pos.0 + direction.0, pos.1 + direction.1, direction, distance.0 + 1),
            (pos.0, pos.1, rotate_left, distance.0 + 1000),
            (pos.0, pos.1, rotate_right, distance.0 + 1000)
        ] {
            // println!("Ha: {:?} {:?}", (nr, nc), dir);
            if seen.contains(&((nr, nc), dir)) {
                // println!("Contains: {:?} {:?}", (nr, nc), dir);
                continue;
            }
            if map[nr as usize][nc as usize] == '#' {
                continue;
            }

            queue.push((Reverse(next_distance), (nr, nc), dir));
            
            if next_distance < distances[nr as usize][nc as usize] {
                distances[nr as usize][nc as usize] = next_distance;
                parents[nr as usize][nc as usize] = pos;
            }

        }
    }

    let end = find_end(map);

    (distances[end.0 as usize][end.1 as usize], parents)
}


fn dijkstra2(map: &Vec<Vec<char>>) -> i32 {
    let (r, c) = find_start(&map);
    
    println!("Initial state ({r}) ({c}):");
    print(&map);

    const EAST: (i32, i32) = (0, 1);

    let mut distances = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), (r, c), EAST));

    let mut best_distance = i32::MAX;

    let mut end_states = VecDeque::new();
    let mut backtrack = HashMap::new();

    while let Some((distance, pos, direction)) = queue.pop() {
        let lowest = *distances.get(&(pos, direction)).unwrap_or(&i32::MAX);
        if distance.0 > lowest {
            continue;
        }
        if map[pos.0 as usize][pos.1 as usize] == 'E' {
            if distance.0 > best_distance {
                break;
            }
            best_distance = distance.0;
            end_states.push_back((pos, direction));
        }

        let rotate_left = (-direction.1, direction.0);
        let rotate_right = (direction.1, -direction.0);
        for (nr, nc, dir, next_distance) in vec![
            (pos.0 + direction.0, pos.1 + direction.1, direction, distance.0 + 1),
            (pos.0, pos.1, rotate_left, distance.0 + 1000),
            (pos.0, pos.1, rotate_right, distance.0 + 1000)
        ] {
            if map[nr as usize][nc as usize] == '#' {
                continue;
            }
            let lowest = *distances.get(&((nr, nc), dir)).unwrap_or(&i32::MAX);
            if next_distance > lowest {
                continue;
            }
            if next_distance < lowest {
                backtrack.insert(((nr, nc), dir), HashSet::new());
                distances.insert(((nr, nc), dir), next_distance);
            }

            backtrack.get_mut(&((nr, nc), dir)).unwrap().insert((pos, direction));
            queue.push((Reverse(next_distance), (nr, nc), dir));
        }
    }

    let mut seen: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    seen.extend(end_states.iter().map(|x| x.clone()));

    while let Some((pos, distance)) = end_states.pop_front() {
        for (np, nd) in backtrack.get(&(pos, distance)).unwrap() {
            if seen.contains(&(*np, *nd)) {
                continue;
            }
            seen.insert((*np, *nd));
            end_states.push_back((*np, *nd));
        }
    }

    let mut dedup = HashSet::new();
    for (np, _) in seen {
        dedup.insert(np);
    }

   dedup.len() as i32
}

#[aoc(day16, part1)]
fn part1(map: &Vec<Vec<char>>) -> i32 {
    let mut end = find_end(map);

    let (score, parents) = dijkstra(map);

    let mut result = map.clone();
    loop {
        let p = parents[end.0 as usize][end.1 as usize];
        if p.0 != 0 && p.1 != 0 {
            result[p.0 as usize][p.1 as usize] = 'X';
        } else {
            break;
        }

        end = p;
    }   

    print(&result);

    score
}

#[aoc(day16, part2)]
fn part2(map: &Vec<Vec<char>>) -> i32 {
    let score = dijkstra2(map);

    score
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(r"###############
#....E#...#.#.#
#.#####.#.#.#.#
#S..#.....#...#
###############")), 2006);
    }

    #[test]
    fn part1_examplel() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 7036);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################")), 11048);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse_input(r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################")), 64);
    }
}