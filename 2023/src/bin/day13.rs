use anyhow::Result;
use parse_display::{Display, FromStr};
use utils::{read_grids, Grid};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, FromStr, Display)]
enum Tile {
    #[display(".")]
    Ash,
    #[display("#")]
    Rocks,
}

fn max_x_y(grid: &Grid<u16, Tile>) -> (u16, u16) {
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();
    (*max_x, *max_y)
}

fn find_vertical_reflection_axis<F>(grid: &Grid<u16, Tile>, count_comparison: F) -> Option<u16>
where
    F: Fn(u16, u16) -> bool,
{
    let (max_x, max_y) = max_x_y(grid);
    (1..=max_x)
        .map(|x_test| {
            (
                x_test,
                (x_test.saturating_sub(max_x - x_test + 1)..x_test)
                    .rev()
                    .flat_map(|x| (0..=max_y).map(|y| (x, y)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                (x_test..(x_test * 2).min(max_x + 1))
                    .flat_map(|x| (0..=max_y).map(|y| (x, y)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .find(|(_, left, right)| {
            let (matching, total) =
                left.iter()
                    .zip(right)
                    .fold((0, 0), |(matching, total), (left, right)| {
                        if grid[&left] == grid[&right] {
                            (matching + 1, total + 1)
                        } else {
                            (matching, total + 1)
                        }
                    });
            count_comparison(matching, total)
        })
        .map(|(x, _, _)| x)
}

fn find_horizontal_reflection_axis<F>(grid: &Grid<u16, Tile>, count_comparison: F) -> Option<u16>
where
    F: Fn(u16, u16) -> bool,
{
    let (max_x, max_y) = max_x_y(grid);
    (1..=max_y)
        .map(|y_test| {
            (
                y_test,
                (y_test.saturating_sub(max_y - y_test + 1)..y_test)
                    .rev()
                    .flat_map(|y| (0..=max_x).map(|x| (x, y)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
                (y_test..(y_test * 2).min(max_y + 1))
                    .flat_map(|y| (0..=max_x).map(|x| (x, y)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .find(|(_, left, right)| {
            let (matching, total) =
                left.iter()
                    .zip(right)
                    .fold((0, 0), |(matching, total), (left, right)| {
                        if grid[&left] == grid[&right] {
                            (matching + 1, total + 1)
                        } else {
                            (matching, total + 1)
                        }
                    });
            count_comparison(matching, total)
        })
        .map(|(y, _, _)| y)
}

fn part1(maps: &[Grid<u16, Tile>]) -> u16 {
    let verticals: u16 = maps
        .iter()
        .filter_map(|m| find_vertical_reflection_axis(m, |matching, total| matching == total))
        .sum();
    let horizontals: u16 = maps
        .iter()
        .filter_map(|m| find_horizontal_reflection_axis(m, |matching, total| matching == total))
        .sum();
    verticals + 100 * horizontals
}

fn part2(maps: &[Grid<u16, Tile>]) -> u16 {
    let verticals: u16 = maps
        .iter()
        .filter_map(|m| find_vertical_reflection_axis(m, |matching, total| matching == total - 1))
        .sum();
    let horizontals: u16 = maps
        .iter()
        .filter_map(|m| find_horizontal_reflection_axis(m, |matching, total| matching == total - 1))
        .sum();
    verticals + 100 * horizontals
}

fn main() -> Result<()> {
    let maps = read_grids("input/day13.txt")?;
    let result = part1(&maps);
    println!("part 1: {}", result);
    let result = part2(&maps);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let maps = read_grids("input/test/day13.txt")?;
    let result = part1(&maps);
    assert_eq!(result, 405);
    let result = part2(&maps);
    assert_eq!(result, 400);
    Ok(())
}
