use std::{ops::Add, str::FromStr, string::ParseError};

use anyhow::Result;
use utils::{read_file, InputParseError};

struct TreeMap {
    lines: Vec<String>,
    width: usize,
}

impl TreeMap {
    fn get(&self, point: &Point) -> Terrain {
        self.lines
            .get(point.y)
            .and_then(|l| {
                l.chars()
                    .nth(point.x % self.width)
                    .and_then(|c| c.to_string().parse().ok())
            })
            .unwrap_or(Terrain::Done)
    }

    fn count_trees_along_slope(&self, slope: &Vector) -> usize {
        std::iter::successors(Some(Point { x: 0, y: 0 }), move |&current_point| {
            Some(current_point + *slope)
        })
        .map(|p| self.get(&p))
        .take_while(|t| *t != Terrain::Done)
        .filter(|t| *t == Terrain::Tree)
        .count()
    }
}

impl FromStr for TreeMap {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s.split('\n').map(String::from).collect();
        let width = lines.get(0).map(|l| l.len()).unwrap_or(0);
        if width == 0 {
            return Err(InputParseError);
        }
        Ok(TreeMap { lines, width })
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
        match s {
            "." => Ok(Terrain::Normal),
            _ => Ok(Terrain::Tree),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
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

    static MAP_INPUT: &str = "..##.......\n\
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

    #[test]
    fn test_part1() -> Result<()> {
        let trees: TreeMap = MAP_INPUT.parse()?;
        let result = part1(&trees);
        assert_eq!(result, 7);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let trees: TreeMap = MAP_INPUT.parse()?;
        let result = part2(&trees);
        assert_eq!(result, 336);

        Ok(())
    }
}
