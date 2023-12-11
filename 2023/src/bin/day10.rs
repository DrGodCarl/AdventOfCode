use std::{collections::HashSet, fmt::Debug, iter};

use anyhow::Result;
use parse_display::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use utils::{read_grid, Grid};

type Point = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, FromStr, EnumIter)]
enum Tile {
    #[display(".")]
    Ground,
    #[display("|")]
    Vertical,
    #[display("-")]
    Horizontal,
    #[display("F")]
    DownRight,
    #[display("7")]
    DownLeft,
    #[display("J")]
    UpLeft,
    #[display("L")]
    UpRight,
    #[display("S")]
    Start,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn coordinate_diff(&self) -> Point {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl Tile {
    fn direction(&self) -> Vec<Direction> {
        match self {
            Tile::Ground => vec![],
            Tile::Vertical => vec![Direction::Up, Direction::Down],
            Tile::Horizontal => vec![Direction::Left, Direction::Right],
            Tile::UpRight => vec![Direction::Up, Direction::Right],
            Tile::UpLeft => vec![Direction::Up, Direction::Left],
            Tile::DownRight => vec![Direction::Down, Direction::Right],
            Tile::DownLeft => vec![Direction::Down, Direction::Left],
            Tile::Start => vec![],
        }
    }

    fn neighbors_diff(&self) -> Vec<Point> {
        self.direction()
            .iter()
            .map(|a| a.coordinate_diff())
            .collect()
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        self.neighbors_diff()
            .iter()
            .map(|(x, y)| (point.0 + x, point.1 + y))
            .collect()
    }
}

fn next_point_and_direction(
    grid: &Grid<i32, Tile>,
    point: &Point,
    direction: &Direction,
) -> Option<(Point, Direction)> {
    let (x, y) = *point;
    let (dx, dy) = direction.coordinate_diff();
    let next_point = (x + dx, y + dy);
    let next_tile = grid.get(&next_point).unwrap_or(&Tile::Ground);
    let direction = next_tile
        .direction()
        .into_iter()
        .filter(|d| d != &direction.opposite())
        .next();
    direction.map(|direction| (next_point, direction))
}

fn all_neighbors(point: &Point) -> Vec<Point> {
    let (x, y) = *point;
    vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
}

fn find_start(grid: &Grid<i32, Tile>) -> Option<Point> {
    grid.iter()
        .find(|(_, tile)| tile == &&Tile::Start)
        .map(|(point, _)| *point)
}

// Thankfully I don't have to deal with errant pipes seeming to connect to
// the starting position.
fn determine_starting_tile(grid: &Grid<i32, Tile>, start: &Point) -> Option<Tile> {
    let neighbors = all_neighbors(start);
    let adjoining_neighbors = neighbors
        .iter()
        .filter_map(|point| grid.get(point).map(|t| (point, t)))
        .filter(|(p, t)| t.neighbors(p).contains(start))
        .map(|(p, _)| p)
        .collect::<Vec<_>>();
    Tile::iter()
        .map(|t| (t, t.neighbors(start)))
        .filter(|(_, neighbors)| {
            neighbors
                .iter()
                .filter(|n| adjoining_neighbors.contains(n))
                .count()
                == 2
        })
        .next()
        .map(|(t, _)| t)
}

fn find_path(pipe_map: &Grid<i32, Tile>) -> Vec<Point> {
    let start = find_start(pipe_map).unwrap();
    let starting_tile = determine_starting_tile(pipe_map, &start).unwrap();
    let mut direction = *starting_tile.direction().first().unwrap();
    let mut point = start;
    let mut path = vec![point];
    loop {
        if let Some((next_point, next_direction)) =
            next_point_and_direction(pipe_map, &point, &direction)
        {
            point = next_point;
            direction = next_direction;
            path.push(point);
        } else {
            break;
        }
    }
    path
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Flooding {
    Flooded,
    NotFlooded,
    Blocked,
}

impl Debug for Flooding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flooded => write!(f, "-"),
            Self::NotFlooded => write!(f, "."),
            Self::Blocked => write!(f, "#"),
        }
    }
}

fn floodable_grid(pipe_map: &Grid<i32, Tile>) -> Grid<i32, Flooding> {
    let path = find_path(pipe_map).into_iter().collect::<HashSet<_>>();
    let minimum_x = pipe_map.keys().map(|(x, _)| x).min().unwrap();
    let minimum_y = pipe_map.keys().map(|(_, y)| y).min().unwrap();
    let maximum_x = pipe_map.keys().map(|(x, _)| x).max().unwrap();
    let maximum_y = pipe_map.keys().map(|(_, y)| y).max().unwrap();
    let mut floodable = pipe_map
        .keys()
        .flat_map(|&(x, y)| {
            vec![
                (2 * x, 2 * y),
                (2 * x + 1, 2 * y),
                (2 * x, 2 * y + 1),
                (2 * x + 1, 2 * y + 1),
            ]
        })
        // This chain wraps everything in a floodable border
        .chain(
            (minimum_x - 1..=maximum_x * 2 + 2)
                .flat_map(|x| vec![(x, minimum_y - 1), (x, maximum_y * 2 + 2)]),
        )
        .chain(
            (minimum_y - 1..=maximum_y * 2 + 2)
                .flat_map(|y| vec![(minimum_x - 1, y), (maximum_x * 2 + 2, y)]),
        )
        .map(|point| (point, Flooding::NotFlooded))
        .collect::<Grid<i32, Flooding>>();
    for point in path {
        let tile = pipe_map.get(&point).unwrap();
        let point = (2 * point.0, 2 * point.1);
        let neighbors = tile.neighbors(&point);
        for neighbor in neighbors.iter().chain(iter::once(&point)) {
            if let Some(flooding) = floodable.get_mut(&neighbor) {
                *flooding = Flooding::Blocked;
            }
        }
    }
    floodable
}

fn descale_floodable_grid(floodable_grid: Grid<i32, Flooding>) -> Grid<i32, Flooding> {
    floodable_grid
        .into_iter()
        .filter(|(p, _)| p.0 % 2 == 0 && p.1 % 2 == 0)
        .map(|(p, f)| ((p.0 / 2, p.1 / 2), f))
        .collect()
}

fn flood(floodable_grid: &mut Grid<i32, Flooding>) {
    let point = (-1, -1);
    let mut queue = vec![point];
    floodable_grid.insert(point, Flooding::Flooded);
    while let Some(point) = queue.pop() {
        let neighbors = all_neighbors(&point);
        for neighbor in neighbors {
            if let Some(flooding) = floodable_grid.get_mut(&neighbor) {
                if *flooding == Flooding::NotFlooded {
                    *flooding = Flooding::Flooded;
                    queue.push(neighbor);
                }
            }
        }
    }
}

fn part1(pipe_map: &Grid<i32, Tile>) -> i32 {
    let path = find_path(pipe_map);
    let steps = path.len() as i32;
    steps / 2
}

fn part2(pipe_map: &Grid<i32, Tile>) -> i32 {
    let mut floodable = floodable_grid(pipe_map);
    flood(&mut floodable);
    floodable = descale_floodable_grid(floodable);
    floodable
        .values()
        .filter(|&&f| f == Flooding::NotFlooded)
        .count() as i32
}

fn main() -> Result<()> {
    let pipe_map: Grid<i32, Tile> = read_grid("input/day10.txt")?;
    let result = part1(&pipe_map);
    println!("part 1: {}", result);
    let result = part2(&pipe_map);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let pipe_map: Grid<i32, Tile> = read_grid("input/test/day10.txt")?;
    let result = part1(&pipe_map);
    assert_eq!(result, 4);
    let pipe_map: Grid<i32, Tile> = read_grid("input/test/day10_2.txt")?;
    let result = part1(&pipe_map);
    assert_eq!(result, 8);
    let pipe_map: Grid<i32, Tile> = read_grid("input/test/day10_3.txt")?;
    let result = part2(&pipe_map);
    assert_eq!(result, 4);
    let pipe_map: Grid<i32, Tile> = read_grid("input/test/day10_4.txt")?;
    let result = part2(&pipe_map);
    assert_eq!(result, 8);
    let pipe_map: Grid<i32, Tile> = read_grid("input/test/day10_5.txt")?;
    let result = part2(&pipe_map);
    assert_eq!(result, 10);
    Ok(())
}
