use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(text: &str) -> Vec<u8> {
    use aoc_parse::{parser, prelude::*};

    let pairs = parser!(line(digit+));
    let sizes: Vec<usize> = pairs.parse(text).unwrap();

    sizes.into_iter().map(|c| c as u8).collect()
}

fn create_blocks(sizes: &Vec<u8>) -> Vec<Option<i32>> {
    let mut blocks = Vec::new();
    let mut id = 0;

    for (i, size) in sizes.into_iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*size {
                blocks.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..*size {
                blocks.push(None);
            }
        }
    }

    println!("Fid: {}", id);

    blocks
}

fn checksum(blocks: &Vec<Option<i32>>) -> u64 {
    let mut checksum = 0;

    for (pos, block) in blocks.iter().enumerate() {
        if let Some(block) = block {
            checksum += (pos as u64) * (*block as u64);
        } else {
            unreachable!("blocks not correctly computed")
        }
    }

    checksum
}

#[aoc(day9, part1)]
pub fn part1(sizes: &Vec<u8>) -> u64 {
    let mut blocks = create_blocks(sizes);

    let blanks: Vec<usize> = blocks
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_none())
        .map(|(i, _)| i)
        .collect();

    for i in blanks {
        while let Some(None) = blocks.last() {
            blocks.pop();
        }
        if i >= blocks.len() {
            break;
        }

        blocks[i] = blocks.pop().unwrap();
    }

    checksum(&blocks)
}

#[aoc(day9, part2)]
pub fn part2(sizes: &Vec<u8>) -> u64 {
    let mut id: i64 = 0;

    let mut files = HashMap::new();
    let mut blanks = Vec::new();

    let mut pos: usize = 0;
    for (i, size) in sizes.into_iter().enumerate() {
        if i % 2 == 0 {
            if *size == 0 {
                panic!("i don't know how to handle those files")
            }
            files.insert(id, (pos, *size));
            id += 1;
        } else if *size != 0 {
            blanks.push((pos, *size));
        }

        pos += *size as usize;
    }

    while id > 0 {
        id -= 1;
        let (pos, size) = files[&id];

        // println!("Checking {}", id);

        let num_blanks = blanks.len();
        for i in 0..num_blanks {
            let (start, length) = blanks[i];
            if start >= pos {
                // blanks.truncate(i);
                break;
            }
            if size <= length {
                files.insert(id, (start, size));
                // println!("Inserting: {}, at {}, {}", id, start, size);

                if size == length {
                    blanks.remove(i);
                } else {
                    blanks[i] = (start + size as usize, length - size);
                }
                break;
            }
        }
    }

    let mut checksum = 0;
    // println!("{:?}", files);
    for (fid, (pos, size)) in files.into_iter() {
        for x in pos..(pos + size as usize) {
            checksum += (fid as usize) * x;
        }
    }
    // q
    checksum as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2858);
    }
}
