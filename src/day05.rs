use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse_input(text: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    use aoc_parse::{parser, prelude::*};
    
    let pairs = parser!(sections(
        lines(u32 "|" u32)
        lines(repeat_sep(u32, ","))
    ));
    let lists: Vec<(Vec<(u32, u32)>, Vec<Vec<u32>>)> = pairs.parse(text).unwrap();
    let (orders, _) = lists[0].clone();
    let (_, patches) = lists[1].clone();

    (orders, patches)
}



fn is_page_valid(page: &Vec<u32>, before: &HashMap<&u32, HashSet<&u32>>, after: &HashMap<&u32, HashSet<&u32>>) -> bool {
    for (index, update) in page.iter().enumerate() {
        for after_update in &page[index + 1..] {
            if let Some(bef) = before.get(update) {
                if bef.contains(after_update) {
                    return false;
                }
            }
        }

        for before_update in &page[..index] {
            if let Some(aft) = after.get(update) {
                if aft.contains(before_update) {
                    return false;
                }
            }
        }
    }

    return true;
}

fn invalid_updates(page: &Vec<u32>, before: &HashMap<&u32, HashSet<&u32>>, after: &HashMap<&u32, HashSet<&u32>>) -> Vec<u32> {
    let mut invalid_updates = Vec::new();
    for (index, update) in page.iter().enumerate() {
        let mut valid = true;
        for after_update in &page[index + 1..] {
            if let Some(bef) = before.get(update) {
                if bef.contains(after_update) {
                    valid = false;
                    break;
                }
            }
        }

        for before_update in &page[..index] {
            if let Some(aft) = after.get(update) {
                if aft.contains(before_update) {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            invalid_updates.push(*update);
        }
    }

    return invalid_updates;
}

#[aoc(day5, part1)]
pub fn part1((orders, pages): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    // which numbers are before key
    let mut before = HashMap::new();
    // which numbers are after key
    let mut after = HashMap::new();
    for (x, y) in orders {
        before.entry(y).or_insert(HashSet::new()).insert(x);
        after.entry(x).or_insert(HashSet::new()).insert(y);
    }


    let mut ans = 0;
    for page in pages {
        if is_page_valid(page, &before, &after) {
            println!("Valid page: {:?}", page);
            let middle = if page.len() % 2 == 0 {
                page.len() / 2 - 1
            } else {
                page.len() / 2
            };

            ans += page[middle];
            // println!("Mid: {}", page[middle]);
        }   
    }

    ans
}

#[aoc(day5, part2)]
pub fn part2((orders, pages): &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    // which numbers are before key
    let mut before = HashMap::new();
    // which numbers are after key
    let mut after = HashMap::new();
    for (x, y) in orders {
        before.entry(y).or_insert(HashSet::new()).insert(x);
        after.entry(x).or_insert(HashSet::new()).insert(y);
    }



    let mut ans = 0;
    for page in pages {
        let invalid_updates = invalid_updates(page, &before, &after);
        if invalid_updates.len() > 0 {
            println!("Invalid: {:?} -> {:?}", page, invalid_updates);
            let mut page = page.clone();
            page.sort_by(|a, b| {
                if let Some(bf) = before.get(b) {
                    if bf.contains(a) {
                        return Ordering::Less;
                    }
                }
                if let Some(af) = after.get(b) {
                    if af.contains(a) {
                        return Ordering::Greater;
                    }
                }

                Ordering::Equal
            });


            let middle = if page.len() % 2 == 0 {
                page.len() / 2 - 1
            } else {
                page.len() / 2
            };

            ans += page[middle];
        }
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";


#[test]
fn part1_example() {
    assert_eq!(part1(&parse_input(TEST_INPUT)), 143);
}

#[test]
fn part2_example() {
    assert_eq!(part2(&parse_input(TEST_INPUT)), 123);
}

}