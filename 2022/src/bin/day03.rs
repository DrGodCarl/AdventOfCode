use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;
use parse_display::FromStr;
use utils::read_lines;

#[derive(FromStr, PartialEq, Debug)]
#[display("{0}")]
struct Rucksack(String);

impl Rucksack {
    fn sections(&self) -> (HashSet<char>, HashSet<char>) {
        let (first, second) = self.0.split_at(self.0.len() / 2);
        (first.chars().collect(), second.chars().collect())
    }

    fn contents(&self) -> HashSet<char> {
        self.0.chars().collect()
    }
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn part1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|sack| sack.sections())
        .filter_map(|(s0, s1)| s0.intersection(&s1).next().copied())
        .map(to_priority)
        .sum()
}

fn part2(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|sack| sack.contents())
        .chunks(3)
        .into_iter()
        .filter_map(|sacks| sacks.reduce(|s0, s1| s0.intersection(&s1).copied().collect()))
        .flatten()
        .map(to_priority)
        .sum()
}

fn main() -> Result<()> {
    let rucksacks = read_lines("input/day03.txt")?;
    let result = part1(&rucksacks);
    println!("part 1: {}", result);
    let result = part2(&rucksacks);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let rucksacks = read_lines("input/test/day03.txt")?;
    let result = part1(&rucksacks);
    assert_eq!(result, 157);

    let result = part2(&rucksacks);
    assert_eq!(result, 70);

    Ok(())
}
