use anyhow::Result;
use itertools::Itertools;
use parse_display::FromStr;
use utils::{read_grid, Grid};

type Point = (i64, i64);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, FromStr)]
enum Space {
    #[display(".")]
    Empty,
    #[display("#")]
    Galaxy,
}

fn rows_and_columns_with_nothing(grid: &Grid<i64, Space>) -> (Vec<i64>, Vec<i64>) {
    let min_y = grid.keys().map(|(_, y)| y).min().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();
    let min_x = grid.keys().map(|(x, _)| x).min().unwrap();
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let rows = (*min_y..=*max_y)
        .filter(|y| (*min_x..=*max_x).all(|x| grid[&(x, *y)] == Space::Empty))
        .collect();
    let columns = (*min_x..=*max_x)
        .filter(|x| (*min_y..=*max_y).all(|y| grid[&(*x, y)] == Space::Empty))
        .collect();
    (rows, columns)
}

fn all_galaxy_locations(grid: &Grid<i64, Space>) -> Vec<Point> {
    grid.iter()
        .filter(|&(_, s)| s == &Space::Galaxy)
        .map(|(p, _)| *p)
        .collect()
}

fn calculate_distance(galaxy_map: &Grid<i64, Space>, mut age_scale: u64) -> u64 {
    age_scale = age_scale - 1;
    let (empty_rows, empty_columns) = rows_and_columns_with_nothing(galaxy_map);
    let galaxy_locations = all_galaxy_locations(galaxy_map);
    galaxy_locations
        .iter()
        .cartesian_product(&galaxy_locations)
        .map(|(a, b)| {
            let (x_min, x_max) = (a.0.min(b.0), a.0.max(b.0));
            let (y_min, y_max) = (a.1.min(b.1), a.1.max(b.1));
            let observed_distance = (x_max - x_min) + (y_max - y_min);
            let empty_row_count = empty_rows
                .iter()
                .filter(|&y| *y > y_min && *y < y_max)
                .count() as u64;
            let empty_column_count = empty_columns
                .iter()
                .filter(|&x| *x > x_min && *x < x_max)
                .count() as u64;
            observed_distance as u64 + empty_row_count * age_scale + empty_column_count * age_scale
        })
        .sum::<u64>()
        / 2
}

fn part1(values: &Grid<i64, Space>) -> u64 {
    calculate_distance(values, 2)
}

fn part2(values: &Grid<i64, Space>) -> u64 {
    calculate_distance(values, 1000000)
}

fn main() -> Result<()> {
    let galaxy_map = read_grid("input/day11.txt")?;
    let result = part1(&galaxy_map);
    println!("part 1: {}", result);
    let result = part2(&galaxy_map);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let galaxy_map = read_grid("input/test/day11.txt")?;
    let result = part1(&galaxy_map);
    assert_eq!(result, 374);
    let result = calculate_distance(&galaxy_map, 10);
    assert_eq!(result, 1030);
    let result = calculate_distance(&galaxy_map, 100);
    assert_eq!(result, 8410);
    Ok(())
}
