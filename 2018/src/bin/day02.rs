use failure::Error;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Result<Vec<String>, Error> {
    let input = File::open("input/day02.txt")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|l| Ok(l?)).collect()
}

fn first() -> Result<i32, Error> {
    let (twice, thrice) = read_input()?.iter().fold((0, 0), |(twice, thrice), val| {
        (
            twice + contains_n_duplicates(val, 2) as i32,
            thrice + contains_n_duplicates(val, 3) as i32,
        )
    });
    Ok(twice * thrice)
}

fn second() -> Result<String, Error> {
    Ok("two".to_string())
}

fn contains_n_duplicates(word: &String, count: i32) -> bool {
    word.chars().collect().sorted().dedup().group_by(|c| word.matches(c).count());
    return word.len() as i32 > count;
}

fn main() {
    println!("Day 1");
    println!("Part One: {}", first().unwrap());
    println!("Part Two: {}", second().unwrap());
}
