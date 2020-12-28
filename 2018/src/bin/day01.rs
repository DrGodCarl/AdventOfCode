use failure::Error;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Result<Vec<i32>, Error> {
    let input = File::open("input/day01.txt")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|l| Ok(l?.parse::<i32>()?)).collect()
}

fn first() -> Result<i32, Error> {
    Ok(read_input()?.iter().sum())
}

fn second() -> Result<i32, Error> {
    Ok(read_input()?
        .iter()
        .cycle()
        .fold_while((0, HashSet::new()), |(frequency, mut observed), val| {
            if observed.contains(&frequency) {
                Done((frequency, observed))
            } else {
                observed.insert(frequency);
                Continue((frequency + val, observed))
            }
        })
        .into_inner()
        .0)
}

fn main() {
    println!("Day 1");
    println!("Part One: {}", first().unwrap());
    println!("Part Two: {}", second().unwrap());
}
