use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use utils::{read_lines, InputParseError};

struct Rucksack(HashSet<char>, HashSet<char>);

impl FromStr for Rucksack {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s
            .chars()
            .chunks(s.len() / 2)
            .into_iter()
            .map(|c| c.collect::<HashSet<_>>())
            .collect_tuple()
        {
            Ok(Rucksack(a, b))
        } else {
            Err(InputParseError)
        }
    }
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("invalid item type"),
    }
}

fn part1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .filter_map(|sack| sack.0.intersection(&sack.1).next().copied())
        .map(to_priority)
        .sum()
}

fn part2(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.map(|sack| sack.0.union(&sack.1).copied().collect::<HashSet<_>>()))
        .filter_map(|sacks| sacks.reduce(|s1, s2| s1.intersection(&s2).copied().collect()))
        .filter_map(|badge| badge.iter().next().copied())
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
