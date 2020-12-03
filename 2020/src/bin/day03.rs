use std::{
    ops::{Add, AddAssign},
    str::FromStr,
    string::ParseError,
};

use anyhow::Result;
use utils::{read_file, InputParseError};

#[derive(Debug, Clone, PartialEq, Copy)]
struct Point {
    x: usize,
    y: usize,
}

type Vector = Point;

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

struct TreeMap {
    lines: Vec<String>,
    width: usize,
}

impl TreeMap {
    fn get(&self, point: &Point) -> Terrain {
        let adjusted_x = point.x % self.width;
        self.lines
            .get(point.y)
            .and_then(|l| {
                l.chars()
                    .nth(adjusted_x)
                    .and_then(|c| c.to_string().parse::<Terrain>().ok())
            })
            .unwrap_or(Terrain::Done)
    }

    fn count_trees_along_slope(&self, slope: &Vector) -> usize {
        let mut terrain = Terrain::Normal;
        let mut current_point = Point { x: 0, y: 0 };
        let mut tree_count = 0;

        while terrain != Terrain::Done {
            if terrain == Terrain::Tree {
                tree_count += 1;
            }
            terrain = self.get(&current_point);
            current_point += *slope;
        }
        tree_count
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Terrain {
    Normal,
    Tree,
    Done,
}

impl FromStr for Terrain {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "." {
            Ok(Terrain::Normal)
        } else {
            Ok(Terrain::Tree)
        }
    }
}

impl FromStr for TreeMap {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s.split('\n').map(|l| String::from(l)).collect();
        let width = lines.get(0).map(|l| l.len()).unwrap_or(0);
        if width == 0 {
            return Err(InputParseError);
        }
        Ok(TreeMap { lines, width })
    }
}

fn part1(trees: &TreeMap) -> usize {
    trees.count_trees_along_slope(&Vector { x: 3, y: 1 })
}

fn part2(trees: &TreeMap) -> usize {
    let slopes = vec![
        Vector { x: 1, y: 1 },
        Vector { x: 3, y: 1 },
        Vector { x: 5, y: 1 },
        Vector { x: 7, y: 1 },
        Vector { x: 1, y: 2 },
    ];
    slopes
        .iter()
        .map(|s| trees.count_trees_along_slope(s))
        .product()
}

fn main() -> Result<()> {
    let trees = read_file("input/day03.txt")?;
    let result = part1(&trees);
    println!("part 1: {}", result);
    let result = part2(&trees);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let map_input = "..##.......\n\
            #...#...#..\n\
            .#....#..#.\n\
            ..#.#...#.#\n\
            .#...##..#.\n\
            ..#.##.....\n\
            .#.#.#....#\n\
            .#........#\n\
            #.##...#...\n\
            #...##....#\n\
            .#..#...#.#\n\
    ";
    let trees: TreeMap = map_input.parse()?;
    let result = part1(&trees);
    assert_eq!(result, 7);

    let result = part2(&trees);
    assert_eq!(result, 336);

    Ok(())
}

#[test]
fn test_lookup() -> Result<()> {
    let map_input = "..##.......\n\
    #...#...#..";

    let trees: TreeMap = map_input.parse()?;
    let point = trees.get(&Point { x: 0, y: 0 });
    assert_eq!(point, Terrain::Normal);
    let point = trees.get(&Point { x: 2, y: 0 });
    assert_eq!(point, Terrain::Tree);
    let point = trees.get(&Point { x: 0, y: 1 });
    assert_eq!(point, Terrain::Tree);
    let point = trees.get(&Point { x: 22, y: 1 });
    assert_eq!(point, Terrain::Tree);
    let point = trees.get(&Point { x: 23, y: 1 });
    assert_eq!(point, Terrain::Normal);
    let point = trees.get(&Point { x: 23, y: 2 });
    assert_eq!(point, Terrain::Done);
    Ok(())
}
