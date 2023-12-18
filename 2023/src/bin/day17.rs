use anyhow::Result;
use pathfinding::prelude::dijkstra;
use utils::{read_grid, Grid};

type Point = (i16, i16);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    fn next(&self, point: &Point) -> Point {
        match self {
            Direction::North => (point.0, point.1 - 1),
            Direction::West => (point.0 - 1, point.1),
            Direction::South => (point.0, point.1 + 1),
            Direction::East => (point.0 + 1, point.1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct StraightPath(Direction, i16);

impl StraightPath {
    fn left(&self) -> StraightPath {
        StraightPath(self.0.left(), 1)
    }

    fn right(&self) -> StraightPath {
        StraightPath(self.0.right(), 1)
    }

    fn straight(&self) -> StraightPath {
        StraightPath(self.0, self.1 + 1)
    }
}

fn neighbor_nodes(
    map: &Grid<i16, i16>,
    point: &Point,
    path: &Option<StraightPath>,
) -> Vec<((Point, Option<StraightPath>), i16)> {
    match path {
        Some(path) => match path {
            StraightPath(_, 3) => vec![path.left(), path.right()],
            _ => vec![path.left(), path.right(), path.straight()],
        },
        None => vec![
            StraightPath(Direction::South, 1),
            StraightPath(Direction::East, 1),
        ],
    }
    .into_iter()
    .filter_map(|path| {
        let next_point = path.0.next(point);
        let next_value = map.get(&next_point)?;
        Some(((next_point, Some(path)), *next_value))
    })
    .collect()
}

fn neighbor_nodes_ultra_crucible(
    map: &Grid<i16, i16>,
    target_point: &Point,
    point: &Point,
    path: &Option<StraightPath>,
) -> Vec<((Point, Option<StraightPath>), i16)> {
    match path {
        Some(path) => match path {
            StraightPath(_, 1..=3) => vec![path.straight()],
            StraightPath(_, 10) => vec![path.left(), path.right()],
            _ => vec![path.left(), path.right(), path.straight()],
        },
        None => vec![
            StraightPath(Direction::South, 1),
            StraightPath(Direction::East, 1),
        ],
    }
    .into_iter()
    .filter_map(|path| {
        let next_point = path.0.next(point);
        let next_value = map.get(&next_point)?;
        if &next_point == target_point && path.1 < 3 {
            return None;
        }
        Some(((next_point, Some(path)), *next_value))
    })
    .collect()
}

fn max_x_y(grid: &Grid<i16, i16>) -> (i16, i16) {
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();
    (*max_x, *max_y)
}

fn part1(map: &Grid<i16, i16>) -> i16 {
    let (max_x, max_y) = max_x_y(map);
    let start: (Point, Option<StraightPath>) = ((0, 0), None);
    dijkstra(
        &start,
        |&(p, dir)| neighbor_nodes(map, &p, &dir),
        |&(p, _)| p == (max_x, max_y),
    )
    .unwrap()
    .1
}

fn part2(map: &Grid<i16, i16>) -> i16 {
    let target = max_x_y(map);
    let start: (Point, Option<StraightPath>) = ((0, 0), None);
    dijkstra(
        &start,
        |&(p, dir)| neighbor_nodes_ultra_crucible(map, &target, &p, &dir),
        |&(p, _)| p == target,
    )
    .unwrap()
    .1
}

fn main() -> Result<()> {
    let map = read_grid("input/day17.txt")?;
    let result = part1(&map);
    println!("part 1: {}", result);
    let result = part2(&map);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let map = read_grid("input/test/day17.txt")?;
    let result = part1(&map);
    assert_eq!(result, 102);
    let result = part2(&map);
    assert_eq!(result, 94);
    Ok(())
}
