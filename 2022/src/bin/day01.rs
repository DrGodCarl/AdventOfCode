use anyhow::Result;
use itertools::Itertools;
use utils::{read_chunks, VecWrapper};

fn part1(calories: &[VecWrapper<u32>]) -> u32 {
    calories.iter().map(|c| c.0.iter().sum()).max().unwrap_or(0)
}

fn part2(calories: &[VecWrapper<u32>]) -> u32 {
    calories
        .iter()
        .map(|c| c.0.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
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
    let numbers: Vec<VecWrapper<u32>> = read_chunks("input/test/day01.txt")?;
    let result = part1(&numbers);
    assert_eq!(result, 24000);

    let result = part2(&numbers);
    assert_eq!(result, 45000);

    Ok(())
}
