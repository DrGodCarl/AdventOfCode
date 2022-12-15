use gcollections::ops::*;

use anyhow::Result;
use interval::interval_set::*;
use parse_display::FromStr;
use utils::read_lines;

#[derive(FromStr, PartialEq, Debug)]
#[display("x={0}, y={1}")]
struct Point(i64, i64);

#[derive(FromStr, PartialEq, Debug)]
#[display("Sensor at {location}: closest beacon is at {beacon}")]
struct Sensor {
    location: Point,
    beacon: Point,
}

impl Sensor {
    fn knowledge_range_at_row(&self, row: i64) -> IntervalSet<i64> {
        let furthest_distance = manhattan_distance(&self.location, &self.beacon);
        let distance_to_row = (self.location.1 - row).abs();
        let distance_to_edge = furthest_distance - distance_to_row;
        if distance_to_edge < 0 {
            return IntervalSet::empty();
        }
        let left_edge = self.location.0 - distance_to_edge;
        let right_edge = self.location.0 + distance_to_edge;
        vec![(left_edge, right_edge)].to_interval_set()
    }
}

fn manhattan_distance(a: &Point, b: &Point) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn part1(sensors: &[Sensor], row: i64) -> u32 {
    let beacons = sensors
        .iter()
        .filter(|s| s.beacon.1 == row)
        .map(|s| IntervalSet::singleton(s.beacon.0))
        .reduce(|a, b| a.union(&b))
        .unwrap_or(IntervalSet::empty());
    sensors
        .iter()
        .map(|s| s.knowledge_range_at_row(row))
        .reduce(|a, b| a.union(&b))
        .unwrap_or(IntervalSet::empty())
        .difference(&beacons)
        .iter()
        .map(|r| r.upper() - r.lower() + 1)
        .sum::<i64>() as u32
}

fn part2(sensors: &[Sensor], bounds: (i64, i64)) -> Option<i64> {
    let Some((y, x_range)) = (bounds.0..=bounds.1)
        .map(|row| {
            (
                row,
                sensors
                    .iter()
                    .map(|s| s.knowledge_range_at_row(row))
                    .reduce(|a, b| a.union(&b))
                    .unwrap_or(IntervalSet::empty()),
            )
        })
        .filter(|(_, s)| s.iter().count() > 1 || s.lower() > bounds.0 || s.upper() < bounds.1)
        .next() else {
            return None;
        };
    let x = if x_range.lower() >= bounds.0 {
        bounds.0
    } else if x_range.upper() <= bounds.1 {
        bounds.1
    } else {
        x_range.iter().fold(i64::MAX, |a, b| a.min(b.upper())) + 1
    };
    Some(x * 4000000 + y)
}

fn main() -> Result<()> {
    let sensors = read_lines("input/day15.txt")?;
    let result = part1(&sensors, 2000000);
    println!("part 1: {}", result);
    let result = part2(&sensors, (0, 4000000));
    println!("part 2: {:?}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let sensors = read_lines("input/test/day15.txt")?;
    let result = part1(&sensors, 10);
    assert_eq!(result, 26);

    let result = part2(&sensors, (0, 20));
    assert_eq!(result, Some(56000011));

    Ok(())
}
