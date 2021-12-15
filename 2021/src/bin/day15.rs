use std::collections::HashMap;

use anyhow::Result;

use pathfinding::prelude::dijkstra;
use utils::read_grid;

type Point = (i16, i16);
type RiskReadings = HashMap<Point, u32>;

fn calculate_max_bounds(points: &RiskReadings) -> Point {
    points
        .keys()
        .fold((0, 0), |mb, p| (p.0.max(mb.0), p.1.max(mb.1)))
}

fn neighbor_points(point: &Point) -> [(i16, i16); 4] {
    [
        (point.0 + 1, point.1),
        (point.0 - 1, point.1),
        (point.0, point.1 + 1),
        (point.0, point.1 - 1),
    ]
}

fn adjusted_repeating_map_get(
    readings: &RiskReadings,
    point: &Point,
    map_bounds: &Point,
    actual_bounds: &Point,
) -> Option<u32> {
    if point.0 < 0 || point.1 < 0 || point.0 > actual_bounds.0 || point.1 > actual_bounds.1 {
        return Option::None;
    }
    let (x_divisor, y_divisor) = (map_bounds.0 + 1, map_bounds.1 + 1);
    let adjusted_point = (point.0 % x_divisor, point.1 % y_divisor);
    let x_block = (point.0 / x_divisor) as u32;
    let y_block = (point.1 / y_divisor) as u32;
    let value = readings.get(&adjusted_point)?;
    Some((value - 1 + x_block + y_block) % 9 + 1)
}

fn part1(readings: &RiskReadings) -> u32 {
    let start = (0, 0);
    let end = calculate_max_bounds(readings);
    let distances = dijkstra(
        &start,
        |p| {
            neighbor_points(p)
                .iter()
                .filter_map(|a| readings.get(a).map(|r| (*a, *r)))
                .collect::<Vec<_>>()
        },
        |&p| p == end,
    );
    distances.unwrap().1
}

fn part2(readings: &RiskReadings) -> u32 {
    let start = (0, 0);
    let bounds = calculate_max_bounds(readings);
    let end = ((bounds.0 + 1) * 5 - 1, (bounds.1 + 1) * 5 - 1);
    let distances = dijkstra(
        &start,
        |p| {
            neighbor_points(p)
                .iter()
                .filter_map(|a| {
                    adjusted_repeating_map_get(readings, a, &bounds, &end).map(|r| (*a, r))
                })
                .collect::<Vec<_>>()
        },
        |&p| p == end,
    );
    distances.unwrap().1
}

fn main() -> Result<()> {
    let risks = read_grid("input/day15.txt")?;
    let result = part1(&risks);
    println!("part 1: {}", result);
    let result = part2(&risks);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let risks = read_grid("input/test/day15.txt")?;
    let result = part1(&risks);
    assert_eq!(result, 40);
    let result = part2(&risks);
    assert_eq!(result, 315);

    Ok(())
}
