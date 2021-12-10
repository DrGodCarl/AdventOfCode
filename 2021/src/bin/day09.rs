use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;
use utils::read_lines;

type SeaFloorReadings = HashMap<(isize, isize), u8>;

fn neighbor_points(point: &(isize, isize)) -> [(isize, isize); 4] {
    [
        (point.0 + 1, point.1),
        (point.0 - 1, point.1),
        (point.0, point.1 + 1),
        (point.0, point.1 - 1),
    ]
}

fn find_max_points(readings: &SeaFloorReadings) -> (isize, isize) {
    readings
        .keys()
        .fold((0isize, 0isize), |(x, y), (a, b)| (*a.max(&x), *b.max(&y)))
}

fn find_low_points(readings: &SeaFloorReadings) -> HashSet<(isize, isize)> {
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
    point: &(isize, isize),
    fill_color: u16,
    fills: &mut HashMap<u16, HashSet<(isize, isize)>>,
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
    let mut fills: HashMap<u16, HashSet<(isize, isize)>> = HashMap::new();
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

fn read_input(path: &str) -> Result<SeaFloorReadings> {
    let lines: Vec<String> = read_lines(path)?;
    let layout_vec = lines
        .iter()
        .map(|l| {
            l.chars()
                .filter(|&c| c != '\n')
                .map(|s| s.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<_>>>();
    let result = layout_vec
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &num)| ((x as isize, y as isize), num))
        })
        .collect();
    Ok(result)
}

fn main() -> Result<()> {
    let readings = read_input("input/day09.txt")?;
    let result = part1(&readings);
    println!("part 1: {}", result);
    let result = part2(&readings);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let readings = read_input("input/test/day09.txt")?;
    let result = part1(&readings);
    assert_eq!(result, 15);
    let result = part2(&readings);
    assert_eq!(result, 1134);

    Ok(())
}
