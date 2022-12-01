use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;
use utils::{read_chunks, InputParseError};

struct CalorieCount(usize);

impl FromStr for CalorieCount {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s
            .split('\n')
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| InputParseError)?
            .iter()
            .sum();
        Ok(CalorieCount(count))
    }
}

fn part1(calories: &[CalorieCount]) -> usize {
    calories.iter().map(|c| c.0).max().unwrap_or(0)
}

fn part2(calories: &[CalorieCount]) -> usize {
    calories.iter().map(|c| c.0).sorted().rev().take(3).sum()
}

fn main() -> Result<()> {
    let numbers = read_chunks("input/day01.txt")?;
    let result = part1(&numbers);
    println!("part 1: {}", result);
    let result = part2(&numbers);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let numbers: Vec<CalorieCount> = read_chunks("input/test/day01.txt")?;
    let result = part1(&numbers);
    assert_eq!(result, 24000);

    let result = part2(&numbers);
    assert_eq!(result, 45000);

    Ok(())
}
