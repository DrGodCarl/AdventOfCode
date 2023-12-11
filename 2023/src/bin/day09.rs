use std::str::FromStr;

use anyhow::Result;
use utils::{read_lines, InputParseError};

struct Sequence {
    numbers: Vec<i32>,
}

impl FromStr for Sequence {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();
        Ok(Sequence { numbers })
    }
}

fn predict_next_number(sequence: &Sequence) -> i32 {
    if sequence.numbers.iter().all(|&n| n == sequence.numbers[0]) {
        return sequence.numbers[0];
    }

    let derivative_sequence: Vec<_> = sequence
        .numbers
        .iter()
        .zip(sequence.numbers.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect();

    sequence.numbers.last().unwrap()
        + predict_next_number(&Sequence {
            numbers: derivative_sequence,
        })
}

fn predict_previous_number(sequence: &Sequence) -> i32 {
    if sequence.numbers.iter().all(|&n| n == sequence.numbers[0]) {
        return sequence.numbers[0];
    }

    let derivative_sequence: Vec<_> = sequence
        .numbers
        .iter()
        .zip(sequence.numbers.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect();

    sequence.numbers.first().unwrap()
        - predict_previous_number(&Sequence {
            numbers: derivative_sequence,
        })
}

fn part1(sequences: &[Sequence]) -> i32 {
    sequences.iter().map(|s| predict_next_number(s)).sum()
}

fn part2(sequences: &[Sequence]) -> i32 {
    sequences.iter().map(|s| predict_previous_number(s)).sum()
}

fn main() -> Result<()> {
    let sequences: Vec<Sequence> = read_lines("input/day09.txt")?;
    let result = part1(&sequences);
    println!("part 1: {}", result);
    let result = part2(&sequences);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let numbers: Vec<Sequence> = read_lines("input/test/day09.txt")?;
    let result = part1(&numbers);
    assert_eq!(result, 114);
    let result = part2(&numbers);
    assert_eq!(result, 2);
    Ok(())
}
