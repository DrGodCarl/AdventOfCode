use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use rayon::prelude::*;
use utils::{read_grid, Grid};

type Point = (i16, i16);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam(Point, Direction);

impl Beam {
    fn next(&self) -> Beam {
        Beam(self.1.next(&self.0), self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, FromStr, Display)]
enum Tile {
    #[display(".")]
    Empty,
    #[display("/")]
    UpRightMirror,
    #[display("\\")]
    DownRightMirror,
    #[display("-")]
    HorizontalSplitter,
    #[display("|")]
    VerticalSplitter,
}

impl Tile {
    fn resulting_beams(&self, beam: &Beam) -> Vec<Beam> {
        match (self, beam.1) {
            (Tile::Empty, _)
            | (Tile::HorizontalSplitter, Direction::East)
            | (Tile::HorizontalSplitter, Direction::West)
            | (Tile::VerticalSplitter, Direction::North)
            | (Tile::VerticalSplitter, Direction::South) => vec![beam.clone()],
            (Tile::UpRightMirror, Direction::North) => vec![Beam(beam.0, Direction::East)],
            (Tile::UpRightMirror, Direction::West) => vec![Beam(beam.0, Direction::South)],
            (Tile::UpRightMirror, Direction::South) => vec![Beam(beam.0, Direction::West)],
            (Tile::UpRightMirror, Direction::East) => vec![Beam(beam.0, Direction::North)],
            (Tile::DownRightMirror, Direction::North) => vec![Beam(beam.0, Direction::West)],
            (Tile::DownRightMirror, Direction::West) => vec![Beam(beam.0, Direction::North)],
            (Tile::DownRightMirror, Direction::South) => vec![Beam(beam.0, Direction::East)],
            (Tile::DownRightMirror, Direction::East) => vec![Beam(beam.0, Direction::South)],
            (Tile::HorizontalSplitter, Direction::North)
            | (Tile::HorizontalSplitter, Direction::South) => {
                vec![Beam(beam.0, Direction::East), Beam(beam.0, Direction::West)]
            }
            (Tile::VerticalSplitter, Direction::West)
            | (Tile::VerticalSplitter, Direction::East) => vec![
                Beam(beam.0, Direction::North),
                Beam(beam.0, Direction::South),
            ],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn next(&self, point: &Point) -> Point {
        match self {
            Direction::North => (point.0, point.1 - 1),
            Direction::West => (point.0 - 1, point.1),
            Direction::South => (point.0, point.1 + 1),
            Direction::East => (point.0 + 1, point.1),
        }
    }
}

fn run_with_initial_beam(grid: &Grid<i16, Tile>, beam: Beam) -> usize {
    let mut has_beam: HashSet<Beam> = HashSet::new();
    let mut beams: Vec<Beam> = vec![beam];
    while let Some(beam) = beams.pop() {
        if has_beam.contains(&beam) {
            // We have already been here, going the same direction.
            continue;
        }
        has_beam.insert(beam.clone());
        let next_beam = beam.next();
        let next_tile = grid.get(&next_beam.0);
        let new_beams = match next_tile {
            Some(tile) => tile.resulting_beams(&next_beam),
            // We have reached the end of the grid.
            None => continue,
        };
        beams.extend(new_beams);
    }

    // The first beam starts outside the grid, so we need to subtract 1.
    has_beam.into_iter().map(|b| b.0).unique().count() - 1
}

fn max_x_y(grid: &Grid<i16, Tile>) -> (i16, i16) {
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();
    (*max_x, *max_y)
}

fn part1(grid: &Grid<i16, Tile>) -> usize {
    run_with_initial_beam(grid, Beam((-1, 0), Direction::East))
}

fn part2(grid: &Grid<i16, Tile>) -> usize {
    let (x_max, y_max) = max_x_y(grid);
    // We need to start the beam outside the grid, so generate all beams starting outside and pointing in.
    (0..=x_max)
        .into_par_iter()
        .map(|x| Beam((x, -1), Direction::South))
        .chain(
            (0..=x_max)
                .into_par_iter()
                .map(|x| Beam((x, y_max + 1), Direction::North)),
        )
        .chain(
            (0..=y_max)
                .into_par_iter()
                .map(|y| Beam((-1, y), Direction::East)),
        )
        .chain(
            (0..=y_max)
                .into_par_iter()
                .map(|y| Beam((x_max + 1, y), Direction::West)),
        )
        .map(|beam| run_with_initial_beam(grid, beam))
        .max()
        .unwrap_or_default()
}

fn main() -> Result<()> {
    let grid = read_grid("input/day16.txt")?;
    let result = part1(&grid);
    println!("part 1: {}", result);
    let result = part2(&grid);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let grid = read_grid("input/test/day16.txt")?;
    let result = part1(&grid);
    assert_eq!(result, 46);
    let result = part2(&grid);
    assert_eq!(result, 51);
    Ok(())
}
