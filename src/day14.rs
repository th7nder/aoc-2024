use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn step(&mut self, width: i32, height: i32) {
        self.px = (self.px + self.vx).rem_euclid(width);
        self.py = (self.py + self.vy).rem_euclid(height);
    }
}

#[aoc_generator(day14)]
fn parse_input(text: &str) -> Vec<Robot> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(lines("p=" i32 "," i32 " v=" i32 "," i32));
    let lists: Vec<(i32, i32, i32, i32)> = pairs.parse(text).unwrap();
    lists
        .into_iter()
        .map(|(px, py, vx, vy)| Robot { px, py, vx, vy })
        .collect()
}

fn step_all(robots: &mut Vec<Robot>, width: i32, height: i32) {
    for robot in robots.iter_mut() {
        robot.step(width, height);
    }
}

fn print(robots: &Vec<Robot>, width: i32, height: i32) {
    for robot in robots {
        println!("Robot: {} {} -> {}", robot.px, robot.py, robot.py % height);
    }

    for y in 0..height {
        for x in 0..width {
            let mut count = 0;
            for robot in robots {
                if robot.px == x && robot.py == y {
                    count += 1;
                }
            }
            if count == 0 {
                print!(".");
            } else {
                print!("{}", count);
            }
        }
        println!();
    }
    println!();
}

fn print_tree(robots: &Vec<Vec<bool>>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            if robots[y as usize][x as usize] {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn safety_factor(robots: &Vec<Robot>, width: i32, height: i32) -> i32 {
    // 7 / 2 = 3
    // 11 / 2 = 5
    let mid_width = width / 2;
    let mid_height = height / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in robots {
        if robot.px < mid_width {
            if robot.py < mid_height {
                top_left += 1;
            } else if robot.py > mid_height {
                top_right += 1;
            }
        } else if robot.px > mid_width {
            if robot.py < mid_height {
                bottom_left += 1;
            } else if robot.py > mid_height {
                bottom_right += 1;
            }
        }
    }

    top_left * top_right * bottom_left * bottom_right
}

#[aoc(day14, part1)]
fn part1(robots: &Vec<Robot>) -> i32 {
    let mut robots: Vec<Robot> = robots.clone();

    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    for _ in 0..100 {
        step_all(&mut robots, WIDTH, HEIGHT);
    }

    safety_factor(&robots, WIDTH, HEIGHT)
}

fn generate_map(robots: &Vec<Robot>, width: usize, height: usize) -> Vec<Vec<bool>> {
    let mut map = Vec::with_capacity(height);
    for _ in 0..height {
        map.push(vec![false; width]);
    }

    for robot in robots {
        map[robot.py as usize][robot.px as usize] = true;
    }

    map
}

fn flood(x: i32, y: i32, map: &mut Vec<Vec<bool>>) -> usize {
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    if x < 0 || x >= width || y < 0 || y >= height {
        return 0;
    }

    if !map[y as usize][x as usize] {
        return 0;
    }

    map[y as usize][x as usize] = false;

    return 1
        + flood(x + 1, y, map)
        + flood(x - 1, y, map)
        + flood(x, y + 1, map)
        + flood(x, y - 1, map);
}

#[aoc(day14, part2)]
fn part2(robots: &Vec<Robot>) -> i32 {
    let mut robots: Vec<Robot> = robots.clone();

    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    for i in 0..1000000 {
        step_all(&mut robots, WIDTH, HEIGHT);

        let mut map = generate_map(&robots, WIDTH as usize, HEIGHT as usize);

        let mut max = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if map[y as usize][x as usize] {
                    let count = flood(x, y, &mut map);
                    max = std::cmp::max(max, count);
                }
            }
        }

        if max > 20 {
            println!("Checking at step: {i} => {max}");
            let map = generate_map(&robots, WIDTH as usize, HEIGHT as usize);
            print_tree(&map, WIDTH, HEIGHT);
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_simple() {
        let mut robots = parse_input("p=2,4 v=2,-3");

        const WIDTH: i32 = 11;
        const HEIGHT: i32 = 7;

        println!("Initial state");
        print(&robots, WIDTH, HEIGHT);

        for i in 0..5 {
            step_all(&mut robots, WIDTH, HEIGHT);
            println!("After {} second: ", i + 1);
            print(&robots, WIDTH, HEIGHT);
        }
    }

    #[test]
    fn part1_example() {
        let mut robots = parse_input(TEST_INPUT);

        const WIDTH: i32 = 11;
        const HEIGHT: i32 = 7;

        for _ in 0..100 {
            step_all(&mut robots, WIDTH, HEIGHT);
        }

        assert_eq!(safety_factor(&robots, WIDTH, HEIGHT), 12);
    }
}
