#![feature(min_const_generics)]

use std::{collections::HashMap, iter, str::FromStr, string::ParseError};

use anyhow::Result;
use itertools::Itertools;
use utils::read_file;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
enum State {
    #[display("#")]
    Active,
    #[display(".")]
    Inactive,
}

type Point = Vec<isize>;

struct PocketDimension<const N: usize> {
    state: HashMap<Point, State>,
}

impl<const N: usize> PocketDimension<N> {
    fn tick(&mut self) {
        self.state = self
            .state
            .keys()
            .flat_map(|p| self.get_neighborhood(p))
            .unique()
            .map(|p| {
                let new = self.new_state_for_point(&p);
                (p, new)
            })
            .collect();
    }

    #[allow(clippy::ptr_arg)]
    fn state_for_point(&self, point: &Vec<isize>) -> &State {
        self.state.get(point).unwrap_or(&State::Inactive)
    }

    #[allow(clippy::ptr_arg)]
    fn new_state_for_point(&self, point: &Vec<isize>) -> State {
        let neighborhood = self.get_neighborhood(point);
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

    // this will include the point passed in
    fn get_neighborhood(&self, point: &[isize]) -> Vec<Point> {
        (0..N)
            .map(|n| (point[n] - 1..=point[n] + 1))
            .fold(Vec::new(), |acc, r| {
                let mut new_acc = vec![];
                let is_empty = acc.is_empty();
                for i in r {
                    if is_empty {
                        new_acc.push(vec![i]);
                    } else {
                        for point in &acc {
                            let new_point = point
                                .iter()
                                .copied()
                                .chain(iter::once(i))
                                .collect::<Vec<isize>>();
                            new_acc.push(new_point);
                        }
                    }
                }
                new_acc
            })
    }
}

impl<const N: usize> FromStr for PocketDimension<N> {
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
                    .map(|(x, s)| {
                        let mut base_vec = vec![0; N];
                        base_vec[0] = x as isize;
                        base_vec[1] = y as isize;
                        (base_vec, s)
                    })
                    .collect::<Vec<_>>() // need to collect so y can keep living
            })
            .collect();
        Ok(PocketDimension { state })
    }
}

fn part1(mut pocket_dimension: PocketDimension<3>) -> usize {
    (0..6).for_each(|_| pocket_dimension.tick());
    pocket_dimension.count_active()
}

fn part2(mut pocket_dimension: PocketDimension<4>) -> usize {
    (0..6).for_each(|_| pocket_dimension.tick());
    pocket_dimension.count_active()
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
