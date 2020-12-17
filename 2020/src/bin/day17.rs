use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    string::ParseError,
};

use anyhow::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use utils::{read_file, InputParseError};

type Point = (isize, isize, isize);

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
enum State {
    #[display("#")]
    Active,
    #[display(".")]
    Inactive,
}

struct PocketDimension {
    state: HashMap<Point, State>,
}

impl PocketDimension {
    fn tick(&mut self) {
        self.state = self
            .state
            .keys()
            .flat_map(|p| get_neighborhood(p))
            .unique()
            .map(|p| (p, self.new_state_for_point(&p)))
            .collect();
    }

    fn state_for_point(&self, point: &Point) -> &State {
        self.state.get(point).unwrap_or(&State::Inactive)
    }

    fn new_state_for_point(&self, point: &Point) -> State {
        let neighborhood = get_neighborhood(point);
        let active_neighbor_count = neighborhood
            .iter()
            .filter(|&p| p != point)
            .filter(|p| self.state_for_point(p) == &State::Active)
            .count();
        match (self.state_for_point(point), active_neighbor_count) {
            (State::Active, 2..=3) => State::Active,
            (State::Inactive, 3) => State::Active,
            _ => State::Inactive,
        }
    }

    fn count_active(&self) -> usize {
        self.state.values().filter(|&s| s == &State::Active).count()
    }
}

impl FromStr for PocketDimension {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = s
            .split_whitespace()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .map(|c| c.to_string())
                    .filter_map(|c| c.parse::<State>().ok())
                    .enumerate()
                    .map(|(x, s)| ((x as isize, y as isize, 0), s))
                    .collect::<Vec<_>>() // need to collect so y can keep living
            })
            .collect();
        Ok(PocketDimension { state })
    }
}

// this will include the point passed in
fn get_neighborhood(point: &Point) -> Vec<Point> {
    (point.0 - 1..=point.0 + 1)
        .cartesian_product(point.1 - 1..=point.1 + 1)
        .cartesian_product(point.2 - 1..=point.2 + 1)
        .map(|((x, y), z)| (x, y, z))
        .collect()
}

fn part1(mut pocket_dimension: PocketDimension) -> usize {
    (0..6).for_each(|_| pocket_dimension.tick());
    pocket_dimension.count_active()
}

fn part2(pocket_dimension: PocketDimension) -> usize {
    0
}

fn main() -> Result<()> {
    let input = read_file("input/day17.txt")?;
    let result = part1(input);
    println!("part 1: {}", result);

    let input = read_file("input/day17.txt")?;
    let result = part2(input);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = read_file("input/test/day17.txt")?;
        let result = part1(input);
        assert_eq!(result, 112);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = read_file("input/test/day17.txt")?;
        let result = part2(input);
        assert_eq!(result, 848);
        Ok(())
    }
}
