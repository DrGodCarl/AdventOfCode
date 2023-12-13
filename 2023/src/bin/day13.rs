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

enum ReflectionAxis {
    Vertical,
    Horizontal,
}

fn find_reflection_axis<F>(
    axis: ReflectionAxis,
    grid: &Grid<u16, Tile>,
    count_comparison: F,
) -> Option<u16>
where
    F: Fn(u16, u16) -> bool,
{
    // This is effectively the same as optionally rotating the grid 90 degrees
    // and then finding the reflection axis for the vertical axis.
    let tuple_fn: Box<dyn Fn(u16, u16) -> (u16, u16)> = match axis {
        ReflectionAxis::Vertical => Box::new(|x, y| (x, y)),
        ReflectionAxis::Horizontal => Box::new(|x, y| (y, x)),
    };

    let (max_x, max_y) = max_x_y(grid);
    let (max_x, max_y) = tuple_fn(max_x, max_y);

    // the line of symmetry (x_test) will be considered to be on the left side of the column we're testing (e.g. "1" means "between 0 and 1")
    (1..=max_x).find(|x_test| {
        let (matching, total) = 
        // for the left side we want to count from x_test down to either 0, or however many columns are on the right (i.e. max_x - x_test + 1)
        (x_test.saturating_sub(max_x - x_test + 1)..*x_test)
            // this is because we specifically want to count down so when we zip them later
            // their equality is testing the reflection
            .rev()
            // this gets us an iter of coordinates going from top to bottom, right to left
            .flat_map(|x| (0..=max_y).map(|y| tuple_fn(x, y)).collect::<Vec<_>>())
            .zip(
                // This is the same as above, but for the right side. From x_test to max_x, or however many columns are on the left (i.e. x_test)
                (*x_test..(x_test * 2).min(max_x + 1))
                    // this mapping is the same as above
                    .flat_map(|x| (0..=max_y).map(|y| tuple_fn(x, y)).collect::<Vec<_>>()),
            )
            .fold((0, 0), |(matching, total), (left, right)| {
                // count the number of matching tiles and the total number of tiles
                (matching + (grid[&left] == grid[&right]) as u16, total + 1)
            });
        // compare them according to the function that was passed in
        count_comparison(matching, total)
    })
}

fn find_vertical_reflection_axis<F>(grid: &Grid<u16, Tile>, count_comparison: F) -> Option<u16>
where
    F: Fn(u16, u16) -> bool,
{
    find_reflection_axis(ReflectionAxis::Vertical, grid, count_comparison)
}

fn find_horizontal_reflection_axis<F>(grid: &Grid<u16, Tile>, count_comparison: F) -> Option<u16>
where
    F: Fn(u16, u16) -> bool,
{
    find_reflection_axis(ReflectionAxis::Horizontal, grid, count_comparison)
}

fn part1(maps: &[Grid<u16, Tile>]) -> u16 {
    let count_comparison = |matching, total| matching == total;
    let verticals: u16 = maps
        .iter()
        .filter_map(|m| find_vertical_reflection_axis(m, count_comparison))
        .sum();
    let horizontals: u16 = maps
        .iter()
        .filter_map(|m| find_horizontal_reflection_axis(m, count_comparison))
        .sum();
    verticals + 100 * horizontals
}

fn part2(maps: &[Grid<u16, Tile>]) -> u16 {
    let count_comparison = |matching, total| matching == total - 1;
    let verticals: u16 = maps
        .iter()
        .filter_map(|m| find_vertical_reflection_axis(m, count_comparison))
        .sum();
    let horizontals: u16 = maps
        .iter()
        .filter_map(|m| find_horizontal_reflection_axis(m, count_comparison))
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
