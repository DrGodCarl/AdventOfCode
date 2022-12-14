use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use utils::{read_file, InputParseError};

fn parse_point_ranges(s: &str) -> Result<HashSet<(i32, i32)>> {
    let points = s
        .split(" -> ")
        .map(|p| {
            let mut p = p.split(",");
            let x = p.next().unwrap().parse()?;
            let y = p.next().unwrap().parse()?;
            Ok((x, y))
        })
        .collect::<Result<Vec<(i32, i32)>>>()?;

    Ok(points
        .iter()
        .tuple_windows()
        .flat_map(|(s, e)| {
            (s.0.min(e.0)..=s.0.max(e.0)).cartesian_product(s.1.min(e.1)..=s.1.max(e.1))
        })
        .collect())
}

#[derive(Clone, Debug)]
struct CrossSectionMap {
    rock: HashSet<(i32, i32)>,
    sand_pile: HashSet<(i32, i32)>,
    max_y: i32,
    sand_path: Vec<(i32, i32)>,
    floor: Option<i32>,
}

impl CrossSectionMap {
    fn new(rock: HashSet<(i32, i32)>, sand_origin: (i32, i32)) -> Self {
        let max_y = rock.iter().map(|p| p.1).max().unwrap_or(0);
        Self {
            rock,
            sand_pile: HashSet::new(),
            max_y,
            sand_path: vec![sand_origin],
            floor: None,
        }
    }

    fn add_floor(&mut self, floor_distance: i32) {
        self.max_y = self.max_y + floor_distance;
        self.floor = Some(self.max_y);
    }

    #[allow(dead_code)]
    fn draw(&self) {
        let min_x = self.rock.iter().map(|p| p.0).min().unwrap_or(0) - 2;
        let max_x = self.rock.iter().map(|p| p.0).max().unwrap_or(0) + 2;
        let min_y = 0;
        let max_y = self.rock.iter().map(|p| p.1).max().unwrap_or(0) + 2;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = (x, y);
                if self.rock.contains(&point) {
                    print!("#");
                } else if self.sand_pile.contains(&point) {
                    print!("o");
                } else if self.floor == Some(y) {
                    print!("=");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl FromStr for CrossSectionMap {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rock = s
            .lines()
            .filter_map(|l| parse_point_ranges(l).ok())
            .flatten()
            .collect();

        Ok(Self::new(rock, (500, 0)))
    }
}

fn has_next_step(map: &CrossSectionMap) -> bool {
    map.sand_path.len() > 0
        && map
            .sand_path
            .last()
            .map(|p| p.1 < map.max_y)
            .unwrap_or(false)
}

fn collides(map: &CrossSectionMap, point: &(i32, i32)) -> bool {
    map.rock.contains(&point) || map.sand_pile.contains(&point) || map.floor == Some(point.1)
}

fn take_next_step(map: &mut CrossSectionMap) {
    let sand_point = map.sand_path.pop().unwrap();
    let mut next_point = (sand_point.0, sand_point.1 + 1);
    if collides(map, &next_point) {
        next_point = (sand_point.0 - 1, sand_point.1 + 1);
    }
    if collides(map, &next_point) {
        next_point = (sand_point.0 + 1, sand_point.1 + 1);
    }
    if collides(map, &next_point) {
        next_point = sand_point;
    }
    if sand_point == next_point {
        map.sand_pile.insert(sand_point);
    } else {
        map.sand_path.push(sand_point);
        map.sand_path.push(next_point);
    }
}

fn part1(map: &mut CrossSectionMap) -> u32 {
    while has_next_step(map) {
        take_next_step(map);
    }
    map.sand_pile.len() as u32
}

fn part2(map: &mut CrossSectionMap) -> u32 {
    map.add_floor(2);
    while has_next_step(map) {
        take_next_step(map);
    }
    map.sand_pile.len() as u32
}

fn main() -> Result<()> {
    let map: CrossSectionMap = read_file("input/day14.txt")?;
    let result = part1(&mut map.clone());
    println!("part 1: {}", result);
    let result = part2(&mut map.clone());
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let map: CrossSectionMap = read_file("input/test/day14.txt")?;
    let result = part1(&mut map.clone());
    assert_eq!(result, 24);

    let result = part2(&mut map.clone());
    assert_eq!(result, 93);

    Ok(())
}
