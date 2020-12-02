use anyhow::{Context, Result};
use itertools::Itertools;
use utils::read_lines;

fn part1(numbers: &[i32]) -> Option<i32> {
    numbers
        .iter()
        .cartesian_product(numbers)
        .find(|(a, b)| *a + *b == 2020)
        .map(|(a, b)| a * b)
}

fn part2(numbers: &[i32]) -> Option<i32> {
    numbers
        .iter()
        .cartesian_product(numbers)
        .cartesian_product(numbers)
        .find(|((a, b), c)| *a + *b + *c == 2020)
        .map(|((a, b), c)| a * b * c)
}

fn main() -> Result<()> {
    let numbers = read_lines("input/day01.txt")?;
    let result = part1(&numbers).context("Failed to find an answer to part one.")?;
    println!("part 1: {}", result);
    let result = part2(&numbers).context("Failed to find an answer to part two.")?;
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let numbers = vec![1721, 979, 366, 299, 675, 1456];
    let result = part1(&numbers).context("Part 1 failed to find an answer for the test.")?;
    assert_eq!(result, 514579);

    let result = part2(&numbers).context("Part 2 failed to find an answer for the test.")?;
    assert_eq!(result, 241861950);

    Ok(())
}
