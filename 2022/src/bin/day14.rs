use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

use anyhow::Result;
use utils::{read_file, InputParseError};

#[derive(Clone, Debug)]
enum PointRange {
    XRange(RangeInclusive<i32>, i32),
    YRange(i32, RangeInclusive<i32>),
}

impl PointRange {
    fn contains(&self, (x, y): &(i32, i32)) -> bool {
        match self {
            PointRange::XRange(range, y2) => range.contains(&x) && y == y2,
            PointRange::YRange(x2, range) => range.contains(&y) && x == x2,
        }
    }
}

// Parse strings to create PointRanges. String looks like:
// 498,4 -> 498,6 -> 496,6
// Output for this would a PointRange::YRange(498, 4..=6) and a PointRange::XRange(496..=498, 6)
fn parse_point_ranges(s: &str) -> Result<Vec<PointRange>> {
    let mut ranges = Vec::new();
    let points = s
        .split(" -> ")
        .map(|p| {
            let mut p = p.split(",");
            let x = p.next().unwrap().parse()?;
            let y = p.next().unwrap().parse()?;
            Ok((x, y))
        })
        .collect::<Result<Vec<(i32, i32)>>>()?;

    let mut prev = points[0];
    for point in points.iter().skip(1) {
        if point.0 == prev.0 {
            ranges.push(PointRange::YRange(
                point.0,
                (prev.1.min(point.1))..=(prev.1.max(point.1)),
            ));
        } else {
            ranges.push(PointRange::XRange(
                (prev.0.min(point.0))..=(prev.0.max(point.0)),
                point.1,
            ));
        }
        prev = point.clone();
    }
    Ok(ranges)
}

#[derive(Clone, Debug)]
struct CrossSectionMap<const P2: bool> {
    ranges: Vec<PointRange>,
    sand_origin: (i32, i32),
    sand_point: (i32, i32),
    sand_pile: HashSet<(i32, i32)>,
    max_y: i32,
}

impl<const P2: bool> CrossSectionMap<P2> {
    fn new(ranges: Vec<PointRange>, sand_origin: (i32, i32)) -> Self {
        let max_y = ranges
            .iter()
            .map(|r| match r {
                PointRange::XRange(_, y) => *y,
                PointRange::YRange(_, ys) => *ys.start(),
            })
            .max()
            .unwrap()
            .clone()
            + if P2 { 2 } else { 0 };
        Self {
            ranges,
            sand_origin,
            sand_point: sand_origin,
            sand_pile: HashSet::new(),
            max_y,
        }
    }

    fn next_point_falling(&self, point: &(i32, i32)) -> (i32, i32) {
        let (x, y) = point;
        let mut next_point = (*x, y + 1);
        if self.collides(&next_point) {
            next_point = (x - 1, y + 1);
        }
        if self.collides(&next_point) {
            next_point = (x + 1, y + 1);
        }
        if self.collides(&next_point) {
            next_point = point.clone();
        }
        next_point
    }

    fn collides(&self, point: &(i32, i32)) -> bool {
        (point.1 == self.max_y && P2)
            || self.sand_pile.contains(point)
            || self.ranges.iter().any(|r| r.contains(point))
    }

    fn step_sand(&mut self) -> bool {
        let next_point = self.next_point_falling(&self.sand_point);

        if next_point.1 > self.max_y {
            return false;
        }

        if next_point == self.sand_origin {
            self.sand_pile.insert(self.sand_point);
            return false;
        }

        if next_point == self.sand_point {
            self.sand_pile.insert(self.sand_point);
            self.sand_point = self.sand_origin;
        } else {
            self.sand_point = next_point;
        }

        true
    }
}

impl<const P2: bool> FromStr for CrossSectionMap<P2> {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .lines()
            .filter_map(|l| parse_point_ranges(l).ok())
            .flatten()
            .collect();

        Ok(Self::new(ranges, (500, 0)))
    }
}

fn part1(map: &mut CrossSectionMap<false>) -> u32 {
    while map.step_sand() {}
    map.sand_pile.len() as u32
}

fn part2(map: &mut CrossSectionMap<true>) -> u32 {
    while map.step_sand() {}
    map.sand_pile.len() as u32
}

fn main() -> Result<()> {
    let map: CrossSectionMap<false> = read_file("input/day14.txt")?;
    let result = part1(&mut map.clone());
    println!("part 1: {}", result);
    let map: CrossSectionMap<true> = read_file("input/day14.txt")?;
    let result = part2(&mut map.clone());
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let map: CrossSectionMap<false> = read_file("input/test/day14.txt")?;
    let result = part1(&mut map.clone());
    assert_eq!(result, 24);

    let map: CrossSectionMap<true> = read_file("input/test/day14.txt")?;
    let result = part2(&mut map.clone());
    assert_eq!(result, 93);

    Ok(())
}
