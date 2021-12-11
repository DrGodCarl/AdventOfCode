use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;
use utils::read_grid;

type Point = (i32, i32);
type OctopusConfiguration = HashMap<Point, u8>;

struct OctopusConfigurationStepper {
    config: OctopusConfiguration,
    keys: HashSet<Point>,
}

fn neighbor_points(point: &Point) -> Vec<Point> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .map(|(x, y)| (point.0 + x, point.1 + y))
        .filter(|&(x, y)| x != point.0 || y != point.1)
        .collect()
}

impl OctopusConfigurationStepper {
    fn new(config: OctopusConfiguration) -> Self {
        let keys = config.keys().cloned().collect();
        Self { config, keys }
    }

    fn step(&mut self) {
        for &p in self.keys.iter() {
            self.config.insert(p, self.config[&p] + 1);
        }
        while self.resolve_flashes() {}
    }

    fn resolve_flashes(&mut self) -> bool {
        let mut octopodes_to_adjust = vec![];
        for &p in self.keys.iter() {
            if self.config[&p] > 9 {
                octopodes_to_adjust.push(p);
                self.config.insert(p, 0);
            }
        }
        for p in octopodes_to_adjust
            .iter()
            .flat_map(|&p| neighbor_points(&p))
        {
            if self.config.contains_key(&p) && self.config[&p] != 0 {
                self.config.insert(p, self.config[&p] + 1);
            }
        }
        !octopodes_to_adjust.is_empty()
    }

    fn count_recent_flashes(&self) -> usize {
        self.config.values().filter(|&&o| o == 0).count()
    }
}

fn part1(octopodes: &mut OctopusConfigurationStepper) -> usize {
    let mut count = 0;
    for _ in 0..100 {
        octopodes.step();
        count += octopodes.count_recent_flashes();
    }
    count
}

fn part2(octopodes: &mut OctopusConfigurationStepper) -> usize {
    let mut count = 0;
    let size = octopodes.config.len();
    while octopodes.count_recent_flashes() != size {
        octopodes.step();
        count += 1;
    }
    count
}

fn main() -> Result<()> {
    let mut octopodes = OctopusConfigurationStepper::new(read_grid("input/day11.txt")?);
    let result = part1(&mut octopodes);
    println!("part 1: {}", result);
    let mut octopodes = OctopusConfigurationStepper::new(read_grid("input/day11.txt")?);
    let result = part2(&mut octopodes);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let mut octopodes = OctopusConfigurationStepper::new(read_grid("input/test/day11.txt")?);
    let result = part1(&mut octopodes);
    assert_eq!(result, 1656);
    let mut octopodes = OctopusConfigurationStepper::new(read_grid("input/test/day11.txt")?);
    let result = part2(&mut octopodes);
    assert_eq!(result, 195);

    Ok(())
}
