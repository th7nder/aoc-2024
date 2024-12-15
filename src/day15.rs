use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn parse_input(text: &str) -> (Vec<Vec<char>>, Vec<char>) {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(
        section(lines(any_char+))
        section(lines(any_char+))
    );
    let (map, movements): (Vec<Vec<char>>, Vec<Vec<char>>) = pairs.parse(text).unwrap();

    let movements = movements.into_iter().flat_map(|l| l).collect();

    (map, movements)
}

fn find_start(map: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == '@' {
                return (row as i32, col as i32);
            }
        }
    }

    unreachable!("map should always have a start");
}

fn score(map: &Vec<Vec<char>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 'O' {
                ans += row * 100 + col;
            }
        }
    }

    ans
}

fn score2(map: &Vec<Vec<char>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == '[' {
                ans += row * 100 + col;
            }
        }
    }

    ans
}

fn move_rec(map: &mut Vec<Vec<char>>, r: i32, c: i32, direction: char) -> (i32, i32) {
    let (dr, dc) = match direction {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => unreachable!("there is no other direction :("),
    };

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let (nr, nc) = (r + dr, c + dc);
    if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
        return (r, c);
    }
    let next_tile = map[nr as usize][nc as usize];
    // Wall
    if next_tile == '#' {
        return (r, c);
    }

    // Pushing nothing
    if next_tile == '.' {
        return (nr, nc);
    }

    // Moving forwards, cause we made space
    if !push(map, nr, nc, direction) {
        return (r, c);
    }

    (nr, nc)
}

fn push(map: &mut Vec<Vec<char>>, r: i32, c: i32, direction: char) -> bool {
    let (dr, dc) = match direction {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => unreachable!("there is no other direction :("),
    };

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    let (nr, nc) = (r + dr, c + dc);
    if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
        return false;
    }
    let next_tile = map[nr as usize][nc as usize];
    if next_tile == '#' {
        return false;
    }
    if next_tile == '.' {
        map[r as usize][c as usize] = '.';
        map[nr as usize][nc as usize] = 'O';
        return true;
    }

    let pushed = push(map, nr, nc, direction);
    if pushed {
        map[r as usize][c as usize] = '.';
        map[nr as usize][nc as usize] = 'O';
    }

    pushed
}

fn can_push(
    map: &Vec<Vec<char>>,
    r: i32,
    c: i32,
    direction: char,
    boxes: &mut HashSet<(i32, i32)>,
) -> bool {
    if map[r as usize][c as usize] != '[' && map[r as usize][c as usize] != ']' {
        unreachable!("broken");
    }

    boxes.insert((r, c));
    let (dr, dc) = match direction {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => unreachable!("there is no other direction :("),
    };

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    let (nr, nc) = (r + dr, c + dc);
    if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
        return false;
    }
    let next_tile = map[nr as usize][nc as usize];
    if next_tile == '#' {
        return false;
    }
    if next_tile == '.' {
        return true;
    }

    if direction == '>' || direction == '<' {
        return can_push(map, nr, nc, direction, boxes);
    }

    let mut can = can_push(map, nr, nc, direction, boxes);
    if map[nr as usize][nc as usize] == ']' {
        can = can && can_push(map, nr, nc - 1, direction, boxes);
    } else if map[nr as usize][nc as usize] == '[' {
        can = can && can_push(map, nr, nc + 1, direction, boxes);
    }

    return can;
}

fn move_rec2(map: &mut Vec<Vec<char>>, r: i32, c: i32, direction: char) -> (i32, i32) {
    let (dr, dc) = match direction {
        '^' => (-1, 0),
        'v' => (1, 0),
        '>' => (0, 1),
        '<' => (0, -1),
        _ => unreachable!("there is no other direction :("),
    };

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let (nr, nc) = (r + dr, c + dc);
    if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
        return (r, c);
    }
    let next_tile = map[nr as usize][nc as usize];
    // Wall
    if next_tile == '#' {
        return (r, c);
    }

    // Pushing nothing
    if next_tile == '.' {
        return (nr, nc);
    }

    // Moving forwards, cause we made space
    let mut boxes = HashSet::new();
    let a = can_push(map, nr, nc, direction, &mut boxes);
    let b = if next_tile == ']' {
        can_push(map, nr, nc - 1, direction, &mut boxes)
    } else {
        can_push(map, nr, nc + 1, direction, &mut boxes)
    };

    if a && b {
        println!("Can push: {:?}", boxes);
        move_nice(map, boxes, direction);
        (nr, nc)
    } else {
        println!("CANNOT push: {:?}", boxes);
        (r, c)
    }
}

fn move_nice(map: &mut Vec<Vec<char>>, boxes: HashSet<(i32, i32)>, direction: char) {
    let rows = map.len();
    let cols = map[0].len();

    if direction == '^' {
        for row in 0..rows {
            for col in 0..cols {
                if boxes.contains(&(row as i32, col as i32)) {
                    map[row - 1][col] = map[row][col];
                    map[row][col] = '.';
                }
            }
        }
    } else if direction == 'v' {
        for row in (0..rows).rev() {
            for col in 0..cols {
                if boxes.contains(&(row as i32, col as i32)) {
                    map[row + 1][col] = map[row][col];
                    map[row][col] = '.';
                }
            }
        }
    } else if direction == '<' {
        for row in 0..rows {
            for col in 0..cols {
                if boxes.contains(&(row as i32, col as i32)) {
                    map[row][col - 1] = map[row][col];
                    map[row][col] = '.';
                }
            }
        }
    } else {
        for row in 0..rows {
            for col in (0..cols).rev() {
                if boxes.contains(&(row as i32, col as i32)) {
                    map[row][col + 1] = map[row][col];
                    map[row][col] = '.';
                }
            }
        }
    }
}

fn print(map: &Vec<Vec<char>>, rr: i32, rc: i32) {
    let rows = map.len();
    let cols = map[0].len();

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            if rr as usize == row && rc as usize == col {
                print!("@");
            } else {
                print!("{}", map[row][col]);
            }
        }
        println!();
    }

    println!();
}

#[aoc(day15, part1)]
fn part1(input: &(Vec<Vec<char>>, Vec<char>)) -> usize {
    let (mut map, movements) = input.clone();

    let (mut r, mut c) = find_start(&map);
    map[r as usize][c as usize] = '.';

    // println!("Initial state ({r}) ({c}):");
    // print(&map, r, c);

    for dir in movements {
        let (nr, nc) = move_rec(&mut map, r, c, dir);
        r = nr;
        c = nc;

        // println!("Move {dir} ({r} {c})");
        // print(&map, r, c);
    }
    print(&map, r, c);
    score(&map)
}

#[aoc(day15, part2)]
fn part2(input: &(Vec<Vec<char>>, Vec<char>)) -> usize {
    let (mut map, movements) = input.clone();
    for row in 0..map.len() {
        let mut new_row = Vec::with_capacity(map[row].len() * 2);
        for col in &map[row] {
            match col {
                '#' => new_row.extend(['#', '#']),
                'O' => new_row.extend(['[', ']']),
                '.' => new_row.extend(['.', '.']),
                '@' => new_row.extend(['@', '.']),
                _ => unreachable!("nope"),
            }
        }
        map[row] = new_row;
    }

    let (mut r, mut c) = find_start(&map);
    map[r as usize][c as usize] = '.';

    println!("Initial state ({r}) ({c}):");
    print(&map, r, c);

    for dir in movements {
        let (nr, nc) = move_rec2(&mut map, r, c, dir);
        r = nr;
        c = nc;

        // println!("Move {dir} ({r} {c})");
        // print(&map, r, c);
    }
    print(&map, r, c);

    score2(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn part1_example_small() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 2028);
    }

    #[test]
    fn part1_example_big() {
        assert_eq!(
            part1(&parse_input(
                r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            )),
            10092
        );
    }

    #[test]
    fn part2_input_small() {
        assert_eq!(
            part2(&parse_input(
                r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            )),
            618
        );
    }

    #[test]
    fn part2_input_big() {
        assert_eq!(
            part2(&parse_input(
                r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            )),
            9021
        );
    }
}
