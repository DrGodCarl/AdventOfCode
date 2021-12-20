#![feature(closure_to_fn_coercion)]

use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::Scan,
};

use anyhow::{Context, Result};
use itertools::Itertools;
use nom::Parser;
use parse_display::FromStr;
use utils::{read_file, read_lines, InputParseError};

#[derive(parse_display::FromStr, Clone, Copy, Hash, PartialEq, Eq)]
#[display("{0},{1},{2}")]
struct Point(i32, i32, i32);

#[derive(parse_display::FromStr, Clone)]
#[display("--- scanner {0} ---")]
struct Id(u8);

#[derive(Clone)]
struct Scanner {
    id: Id,
    points: HashSet<Point>,
    full_rotations: HashMap<usize, HashSet<Point>>,
}

impl Scanner {
    fn new(id: Id, points: HashSet<Point>) -> Self {
        let full_rotations = transformations()
            .iter()
            .map(|(&id, rot)| (id, points.iter().map(rot).collect()))
            .collect();
        Self {
            id,
            points,
            full_rotations,
        }
    }
}

impl std::str::FromStr for Scanner {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let id = lines
            .next()
            .unwrap()
            .parse::<Id>()
            .map_err(|_| InputParseError)?;
        let points = lines
            .map(|p| p.parse::<Point>())
            .collect::<Result<_, _>>()
            .map_err(|_| InputParseError)?;

        Ok(Scanner::new(id, points))
    }
}

type Transformation = fn(&Point) -> Point;

fn transformations() -> HashMap<usize, Transformation> {
    fn ident(p: &Point) -> Point {
        *p
    }
    fn shift(p: &Point) -> Point {
        Point(p.2, p.0, p.1)
    }
    fn even1(p: &Point) -> Point {
        Point(p.0, -p.1, -p.2)
    }
    fn even2(p: &Point) -> Point {
        Point(-p.0, p.1, -p.2)
    }
    fn even3(p: &Point) -> Point {
        Point(-p.0, -p.1, p.2)
    }
    fn odd1(p: &Point) -> Point {
        Point(p.0, p.2, p.1)
    }
    fn odd2(p: &Point) -> Point {
        Point(-p.0, p.2, p.1)
    }
    fn odd3(p: &Point) -> Point {
        Point(p.0, -p.2, p.1)
    }
    fn odd4(p: &Point) -> Point {
        Point(p.0, p.2, -p.1)
    }
    let transformations: Vec<Transformation> = vec![
        ident,
        even1,
        even2,
        even3,
        odd1,
        odd2,
        odd3,
        odd4,
        shift,
        |p| even1(&shift(p)),
        |p| even2(&shift(p)),
        |p| even3(&shift(p)),
        |p| odd1(&shift(p)),
        |p| odd2(&shift(p)),
        |p| odd3(&shift(p)),
        |p| odd4(&shift(p)),
        |p| shift(&shift(p)),
        |p| even1(&shift(&shift(p))),
        |p| even2(&shift(&shift(p))),
        |p| even3(&shift(&shift(p))),
        |p| odd1(&shift(&shift(p))),
        |p| odd2(&shift(&shift(p))),
        |p| odd3(&shift(&shift(p))),
        |p| odd4(&shift(&shift(p))),
    ];
    transformations.into_iter().enumerate().collect()
}

fn part1(scanners: &[Scanner]) -> usize {
    let mut scanner_iter = scanners.iter();
    let mut next_check = vec![scanner_iter.next().unwrap()];
    let mut to_scan: Vec<&Scanner> = scanner_iter.collect();

    while let Some(scanner) = next_check.pop() {
        for &other in to_scan.iter() {
            // If I ever return...
            todo!("Need to compare the translation needed between all points in all rotations of `other` and fine a set with 12+ matching translations")
        }
    }
    todo!()
}

fn part2(numbers: &[Scanner]) -> usize {
    todo!()
}

fn read_input(path: &str) -> Result<Vec<Scanner>> {
    let s = fs::read_to_string("input/test/day19.txt")?;
    Ok(s.split("\n\n")
        .map(|s| s.parse::<Scanner>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn main() -> Result<()> {
    let numbers = read_input("input/day19.txt")?;
    let result = part1(&numbers);
    println!("part 1: {}", result);
    let result = part2(&numbers);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let numbers: Vec<Scanner> = read_input("input/test/day19.txt")?;
    let result = part1(&numbers);
    assert_eq!(result, 79);

    Ok(())
}
