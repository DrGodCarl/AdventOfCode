use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn read_input() -> Result<Vec<i32>> {
    let input = File::open("input/day01.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Ok(line?.parse::<i32>()?))
        .collect()
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_fuel_considering_fuel(mass: i32) -> i32 {
    let fuel = calculate_fuel(mass);
    if fuel < 0 {
        return 0;
    }
    fuel + calculate_fuel_considering_fuel(fuel)
}

fn main() -> Result<()> {
    let input = read_input()?;

    let result1: i32 = input.iter().map(|i| calculate_fuel(*i)).sum();
    println!("part 1: {}", result1);

    let result2: i32 = input
        .iter()
        .map(|i| calculate_fuel_considering_fuel(*i))
        .sum();
    println!("part 2: {}", result2);

    Ok(())
}
