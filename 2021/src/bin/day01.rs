use anyhow::Result;
use itertools::Itertools;
use utils::read_lines;

fn part1(numbers: &[i32]) -> usize {
    numbers
        .iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn part2(numbers: &[i32]) -> usize {
    numbers
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn main() -> Result<()> {
    let numbers = read_lines("input/day01.txt")?;
    let result = part1(&numbers);
    println!("part 1: {}", result);
    let result = part2(&numbers);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let numbers = read_lines("input/test/day01.txt")?;
    let result = part1(&numbers);
    assert_eq!(result, 7);

    let result = part2(&numbers);
    assert_eq!(result, 5);

    Ok(())
}
