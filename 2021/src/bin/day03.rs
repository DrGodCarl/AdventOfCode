use anyhow::{Context, Result};
use utils::read_lines;

fn part1(numbers: &[usize], size: usize) -> usize {
    let length = numbers.len();
    let gamma = (0..size)
        .map(|pos| numbers.iter().map(move |n| n >> pos & 1).sum())
        .map(|c: usize| c > length / 2)
        .fold((0usize, 0), |(pos, acc), bit| {
            (pos + 1, acc + ((bit as usize) << pos))
        })
        .1;
    let epsilon = !gamma & ((1 << size) - 1);
    epsilon * gamma
}

fn part2(numbers: &[usize], size: usize) -> Result<usize> {
    let oxy_consumption = (0..size)
        .rev()
        .fold(numbers.to_vec(), |remaining_numbers, pos| {
            let length = remaining_numbers.len() as f32;
            let count: f32 = remaining_numbers
                .iter()
                .map(|n| n >> pos & 1)
                .sum::<usize>() as f32;
            let bit = (count >= length / 2.0) as usize;
            remaining_numbers
                .into_iter()
                .filter(|&n| n >> pos & 1 == bit)
                .collect()
        })
        .first()
        .with_context(|| "No oxygen consumption")?
        .to_owned();
    let co2_scrubbing = (0..size)
        .rev()
        .fold(numbers.to_vec(), |remaining_numbers, pos| {
            let length = remaining_numbers.len() as f32;
            if remaining_numbers.len() == 1 {
                return remaining_numbers;
            }
            let count: f32 = remaining_numbers
                .iter()
                .map(|n| n >> pos & 1)
                .sum::<usize>() as f32;
            let bit = (count < length / 2.0) as usize;
            remaining_numbers
                .into_iter()
                .filter(|&n| n >> pos & 1 == bit)
                .collect()
        })
        .first()
        .with_context(|| "No co2 scrubbing")?
        .to_owned();
    Ok(oxy_consumption * co2_scrubbing)
}

fn read_input(path: &str) -> Result<(Vec<usize>, usize)> {
    let strings: Vec<String> = read_lines(path)?;
    let size = strings.first().with_context(|| "no string found")?.len();
    let numbers = strings
        .iter()
        .map(|s| usize::from_str_radix(s, 2))
        .collect::<Result<_, _>>()?;
    Ok((numbers, size))
}

fn main() -> Result<()> {
    let (numbers, size) = read_input("input/day03.txt")?;
    let result = part1(&numbers, size);
    println!("part 1: {}", result);
    let result = part2(&numbers, size)?;
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let (numbers, size) = read_input("input/test/day03.txt")?;
    let result = part1(&numbers, size);
    assert_eq!(result, 198);

    let result = part2(&numbers, size)?;
    assert_eq!(result, 230);

    Ok(())
}
