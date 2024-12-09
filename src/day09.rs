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

#[aoc(day9, part1)]
pub fn part1(sizes: &Vec<u8>) -> u64 {
    let mut blocks = create_blocks(sizes);

    let blanks: Vec<usize> = blocks.iter().enumerate().filter(|(i, c)| c.is_none()).map(|(i, c)| i).collect();

    for i in blanks {
        while let Some(None) = blocks.last() {
            blocks.pop();
        }
        if i >= blocks.len() { 
            break;
        }

        blocks[i] = blocks.pop().unwrap();
    }
    
    let mut checksum = 0;
    
    for (pos, block) in blocks.into_iter().enumerate() {
        if let Some(block) = block {
            checksum += (pos as u64) * block as u64;
        } else {
            unreachable!("blocks not correctly computed")
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"23331331214141314025";
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1928);
    }

}