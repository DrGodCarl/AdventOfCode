#![feature(iter_map_windows)]
use std::collections::HashMap;

use anyhow::Result;
use lazy_static::lazy_static;
use utils::read_lines;

fn part1(values: &[String]) -> u32 {
    values
        .iter()
        .filter_map(|s| {
            let mut digits = s.chars().filter(|c| c.is_ascii_digit());
            digits.next().map(|first| {
                let last = digits.last().unwrap_or_else(|| first);
                format!("{}{}", first, last)
            })
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum()
}

lazy_static! {
    static ref TEXT_MAP: HashMap<String, u32> = vec![
        ("0".to_string(), 0),
        ("one".to_string(), 1),
        ("1".to_string(), 1),
        ("two".to_string(), 2),
        ("2".to_string(), 2),
        ("three".to_string(), 3),
        ("3".to_string(), 3),
        ("four".to_string(), 4),
        ("4".to_string(), 4),
        ("five".to_string(), 5),
        ("5".to_string(), 5),
        ("six".to_string(), 6),
        ("6".to_string(), 6),
        ("seven".to_string(), 7),
        ("7".to_string(), 7),
        ("eight".to_string(), 8),
        ("8".to_string(), 8),
        ("nine".to_string(), 9),
        ("9".to_string(), 9),
    ]
    .into_iter()
    .collect();
}

fn read_number(five_chars: &[char; 5]) -> Option<u32> {
    let mut current = String::new();
    for c in five_chars {
        current.push(*c);
        if let Some(value) = TEXT_MAP.get(&current) {
            return Some(*value);
        }
    }
    None
}

fn part2(values: &[String]) -> u32 {
    values
        .iter()
        .filter_map(|s| {
            let mut digits = s
                .chars()
                // This chain is to make sure we get the last digit - map_windows stops at the end of the iterator
                // so if the text ends in "xxtwo" we wouldn't parse the two (we'd check "x", "xx", "xxt", "xxtw" and "xxtwo").
                // By adding the 'y's we make sure that we get the last digit.
                .chain(['y'; 4])
                .map_windows(|chars: &[_; 5]| read_number(chars))
                .filter_map(|x| x);
            digits.next().map(|first| {
                let last = digits.last().unwrap_or_else(|| first);
                format!("{}{}", first, last)
            })
        })
        .map(|s| s.parse::<u32>().unwrap())
        .sum()
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
    let numbers: Vec<String> = read_lines("input/test/day01.txt")?;
    let result = part1(&numbers);
    assert_eq!(result, 142);
    let numbers: Vec<String> = read_lines("input/test/day01_2.txt")?;
    let result = part2(&numbers);
    assert_eq!(result, 281);
    Ok(())
}
