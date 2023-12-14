use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    iter,
};

use anyhow::Result;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use utils::{read_grid, Grid};

type Point = (u32, u32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, FromStr, Display)]
enum Tile {
    #[display(".")]
    Nothing,
    #[display("#")]
    SquareBoulder,
    #[display("O")]
    RoundBoulder,
    Blocked,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn roll(&self, point: &Point) -> Point {
        match self {
            Direction::North => (point.0, point.1.saturating_sub(1)),
            Direction::West => (point.0.saturating_sub(1), point.1),
            Direction::South => (point.0, point.1 + 1),
            Direction::East => (point.0 + 1, point.1),
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Movement {
    Moved,
    Stationary,
}

struct Platform {
    grid: Grid<u32, Tile>,
    max_x: u32,
    max_y: u32,
    direction: Option<Direction>,
}

impl Hash for Platform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.values().for_each(|v| v.hash(state));
    }
}

fn hash_platform(platform: &Platform) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    platform.hash(&mut hasher);
    hasher.finish()
}

impl Platform {
    fn new(grid: Grid<u32, Tile>) -> Self {
        let (max_x, max_y) = max_x_y(&grid);
        Self {
            grid,
            max_x,
            max_y,
            direction: None,
        }
    }

    fn tilt(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }

    fn movement_iter(&self) -> Box<dyn Iterator<Item = Point>> {
        match self.direction {
            Some(Direction::North) => Box::new((0..=self.max_x).cartesian_product(0..=self.max_y)),
            Some(Direction::West) => Box::new(
                (0..=self.max_y)
                    .cartesian_product(0..=self.max_x)
                    .map(|(y, x)| (x, y)),
            ),
            Some(Direction::South) => {
                Box::new((0..=self.max_x).cartesian_product((0..=self.max_y).rev()))
            }
            Some(Direction::East) => Box::new(
                (0..=self.max_y)
                    .cartesian_product((0..=self.max_x).rev())
                    .map(|(y, x)| (x, y)),
            ),
            _ => Box::new(iter::empty::<Point>()),
        }
    }

    fn tick(&mut self) -> Movement {
        self.movement_iter().fold(Movement::Stationary, |m, p| {
            let current_tile = self.grid.get(&p).unwrap_or(&Tile::Blocked);
            match current_tile {
                Tile::RoundBoulder => {
                    let next_point = self.direction.map_or(p, |d| d.roll(&p));
                    let next_tile = self.grid.get(&next_point).unwrap_or(&Tile::Blocked);
                    match next_tile {
                        Tile::Nothing => {
                            self.grid.insert(p, Tile::Nothing);
                            self.grid.insert(next_point, Tile::RoundBoulder);
                            Movement::Moved
                        }
                        _ => m,
                    }
                }
                _ => m,
            }
        })
    }

    fn calculate_load(&self) -> u32 {
        self.grid
            .iter()
            .filter(|(_, tile)| **tile == Tile::RoundBoulder)
            .map(|((_, y), _)| self.max_y + 1 - *y)
            .sum()
    }
}

fn spin_cycle(platform: &mut Platform) {
    let mut direction = Direction::North;
    for _ in 0..4 {
        platform.tilt(direction);
        while platform.tick() == Movement::Moved {}
        direction = direction.turn_left();
    }
}

#[derive(Debug)]
struct Recurrence {
    loop_start: usize,
    load_history: Vec<u32>,
}

impl Recurrence {
    fn load_at(&self, cycle: usize) -> u32 {
        let index = if cycle < self.load_history.len() {
            cycle
        } else {
            let loop_length = self.load_history.len() - self.loop_start;
            let cycle_after_first_loop = cycle - self.load_history.len() - 1;
            cycle_after_first_loop % loop_length + self.loop_start + 1
        };
        self.load_history[index]
    }
}

fn detect_recurrence(platform: &mut Platform) -> Recurrence {
    let mut platform_to_idx = HashMap::new();
    let mut load_history = Vec::new();
    let loop_start = loop {
        let hash = hash_platform(&platform);
        if platform_to_idx.contains_key(&hash) {
            break platform_to_idx[&hash];
        }
        platform_to_idx.insert(hash, load_history.len());
        load_history.push(platform.calculate_load());
        spin_cycle(platform);
    };

    Recurrence {
        loop_start,
        load_history,
    }
}

fn max_x_y(grid: &Grid<u32, Tile>) -> (u32, u32) {
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();
    (*max_x, *max_y)
}

fn part1(platform: &mut Platform) -> u32 {
    platform.tilt(Direction::North);
    while platform.tick() == Movement::Moved {}
    platform.calculate_load()
}

fn part2(platform: &mut Platform) -> u32 {
    detect_recurrence(platform).load_at(1_000_000_000)
}

fn read_platform(path: &str) -> Result<Platform> {
    let grid = read_grid(path)?;
    Ok(Platform::new(grid))
}

fn main() -> Result<()> {
    let mut platform = read_platform("input/day14.txt")?;
    let result = part1(&mut platform);
    println!("part 1: {}", result);
    let mut platform = read_platform("input/day14.txt")?;
    let result = part2(&mut platform);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let mut platform = read_platform("input/test/day14.txt")?;
    let result = part1(&mut platform);
    assert_eq!(result, 136);
    let mut platform = read_platform("input/test/day14.txt")?;
    let result = part2(&mut platform);
    assert_eq!(result, 64);
    Ok(())
}
