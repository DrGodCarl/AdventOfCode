use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;
use utils::read_grid;

type SeaFloorReadings = HashMap<(i64, i64), u8>;

fn neighbor_points(point: &(i64, i64)) -> [(i64, i64); 4] {
    [
        (point.0 + 1, point.1),
        (point.0 - 1, point.1),
        (point.0, point.1 + 1),
        (point.0, point.1 - 1),
    ]
}

fn find_max_points(readings: &SeaFloorReadings) -> (i64, i64) {
    readings
        .keys()
        .fold((0i64, 0i64), |(x, y), (a, b)| (*a.max(&x), *b.max(&y)))
}

fn find_low_points(readings: &SeaFloorReadings) -> HashSet<(i64, i64)> {
    let (x_max, y_max) = find_max_points(readings);
    (0..=x_max)
        .cartesian_product(0..=y_max)
        .filter(|p| {
            neighbor_points(p)
                .iter()
                .all(|n| readings.get(n).unwrap_or(&u8::MAX) > readings.get(p).unwrap())
        })
        .collect()
}

// https://en.wikipedia.org/wiki/Flood_fill
fn flood_fill(
    readings: &SeaFloorReadings,
    point: &(i64, i64),
    fill_color: u16,
    fills: &mut HashMap<u16, HashSet<(i64, i64)>>,
) -> bool {
    if readings.get(point).unwrap_or(&9) == &9 {
        return false;
    }
    if fills.values().any(|s| s.contains(point)) {
        return false;
    }
    fills.get_mut(&fill_color).unwrap().insert(*point);
    neighbor_points(point)
        .iter()
        .map(|p| flood_fill(readings, p, fill_color, fills))
        .for_each(|_| {});
    true
}

fn part1(readings: &SeaFloorReadings) -> usize {
    find_low_points(readings)
        .iter()
        .map(|p| *readings.get(p).unwrap() as usize)
        .map(|i| i + 1)
        .sum()
}

fn part2(readings: &SeaFloorReadings) -> usize {
    let (x_max, y_max) = find_max_points(readings);
    let mut fill_color = 1;
    let mut fills: HashMap<u16, HashSet<(i64, i64)>> = HashMap::new();
    fills.insert(fill_color, HashSet::new());
    for point in (0..=x_max).cartesian_product(0..=y_max) {
        if flood_fill(readings, &point, fill_color, &mut fills) {
            fill_color += 1;
            fills.insert(fill_color, HashSet::new());
        }
    }
    fills
        .values()
        .map(|ps| ps.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn main() -> Result<()> {
    let readings = read_grid("input/day09.txt")?;
    let result = part1(&readings);
    println!("part 1: {}", result);
    let result = part2(&readings);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let readings = read_grid("input/test/day09.txt")?;
    let result = part1(&readings);
    assert_eq!(result, 15);
    let result = part2(&readings);
    assert_eq!(result, 1134);

    Ok(())
}
