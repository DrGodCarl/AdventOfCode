use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn run(letters: &str, length: usize) -> u32 {
    letters
        .chars()
        .collect::<Vec<_>>()
        .as_slice()
        .windows(length)
        .enumerate()
        .find(|(_, window)| window.iter().duplicates().count() == 0)
        .map(|(i, _)| i as u32)
        .unwrap()
        + length as u32
}

fn part1(letters: &str) -> u32 {
    run(letters, 4)
}

fn part2(letters: &str) -> u32 {
    run(letters, 14)
}

fn main() -> Result<()> {
    let letters: String = fs::read_to_string("input/day06.txt")?;
    let result = part1(&letters);
    println!("part 1: {}", result);
    let result = part2(&letters);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let letters = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let result = part1(&letters);
    assert_eq!(result, 5);
    let letters = "nppdvjthqldpwncqszvftbrmjlhg";
    let result = part1(&letters);
    assert_eq!(result, 6);
    let letters = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let result = part1(&letters);
    assert_eq!(result, 10);
    let letters = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let result = part1(&letters);
    assert_eq!(result, 11);

    let letters = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let result = part2(&letters);
    assert_eq!(result, 19);
    let letters = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let result = part2(&letters);
    assert_eq!(result, 23);
    let letters = "nppdvjthqldpwncqszvftbrmjlhg";
    let result = part2(&letters);
    assert_eq!(result, 23);
    let letters = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let result = part2(&letters);
    assert_eq!(result, 29);
    let letters = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let result = part2(&letters);
    assert_eq!(result, 26);

    Ok(())
}
