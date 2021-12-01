use std::collections::HashSet;

use anyhow::{Context, Result};
use itertools::{Itertools, MinMaxResult};
use utils::read_lines;

fn part1(numbers: &[i64], preamble: usize) -> Option<i64> {
    numbers
        .windows(preamble + 1)
        .find(|&nums| {
            let sum = nums.last().unwrap();
            let possible: HashSet<_> = nums
                .iter()
                .take(preamble)
                .map(|&i| sum - i)
                .filter(|i| i * 2 != *sum)
                .collect();
            !nums.iter().take(preamble).any(|j| possible.contains(j))
        })
        .and_then(|a| a.last())
        .copied()
}

fn part2(numbers: &[i64], preamble: usize) -> Option<i64> {
    let target = part1(numbers, preamble).unwrap();
    (2..numbers.len())
        .map(|range| {
            numbers
                .windows(range)
                .find(|&nums| nums.iter().sum::<i64>() == target)
                .and_then(|found| match found.iter().minmax() {
                    MinMaxResult::MinMax(a, b) => Some(a + b),
                    _ => None,
                })
        })
        .find(|a| a.is_some())
        .flatten()
}

fn main() -> Result<()> {
    let numbers = read_lines("input/day09.txt")?;
    let result = part1(&numbers, 25).context("Failed to find an answer to part one.")?;
    println!("part 1: {}", result);

    let result = part2(&numbers, 25).context("Failed to find an answer to part two.")?;
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() -> Result<()> {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let result = part1(&numbers, 5).context("Part 1 failed to find an answer for the test.")?;
        assert_eq!(result, 127);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let result = part2(&numbers, 5).context("Part 2 failed to find an answer for the test.")?;
        assert_eq!(result, 62);

        Ok(())
    }
}
