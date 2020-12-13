use std::{str::FromStr, string::ParseError};

use anyhow::{Context, Result};
use itertools::Itertools;
use utils::read_file;

#[derive(Debug)]
struct World {
    time: u64,
    bus_schedule: Vec<Option<u64>>,
}

impl FromStr for World {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<&str> = s.split_whitespace().collect();
        let time = input[0].parse().unwrap();
        let bus_schedule = input[1].split(",").map(|num| num.parse().ok()).collect();
        Ok(World { time, bus_schedule })
    }
}

fn part1(world: &World) -> Option<u64> {
    let result = world
        .bus_schedule
        .iter()
        .filter_map(|&t| t)
        .map(|t| (t, t - (world.time % t)))
        .sorted_by_key(|(_, r)| r.clone())
        .next();
    result.map(|(bus_id, num_of_min)| bus_id * num_of_min)
}

fn part2(world: &World) -> u64 {
    world
        .bus_schedule
        .iter()
        .enumerate()
        .filter_map(|(diff, modulo)| modulo.map(|m| (diff as u64, m)))
        // https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
        .fold((1, 0), |(step, mut ans), (diff, modulo)| {
            while (ans + diff) % modulo != 0 {
                ans += step;
            }
            (step * modulo, ans)
        })
        .1
}

fn main() -> Result<()> {
    let world: World = read_file("input/day13.txt")?;
    let result = part1(&world).context("Couldn't find an answer for part 1")?;
    println!("part 1: {}", result);

    let result = part2(&world);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2() -> Result<()> {
        let world = "0\n67,7,59,61".parse()?;
        let result = part2(&world);
        assert_eq!(result, 754018);

        let world = "0\n67,7,x,59,61".parse()?;
        let result = part2(&world);
        assert_eq!(result, 1261476);
        Ok(())
    }
}
