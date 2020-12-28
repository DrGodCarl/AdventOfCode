#[macro_use]
extern crate itertools;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::once;
use std::ops::Add;
use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
}

fn to_point_change(direction: Direction) -> PointWithDistance {
    match direction {
        Direction::UP(s) => PointWithDistance { x: 0, y: s as i32, distance: s as i32 },
        Direction::DOWN(s) => PointWithDistance { x: 0, y: -(s as i32), distance: s as i32 },
        Direction::LEFT(s) => PointWithDistance { x: -(s as i32), y: 0, distance: s as i32 },
        Direction::RIGHT(s) => PointWithDistance { x: s as i32, y: 0, distance: s as i32 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PointWithDistance {
    x: i32,
    y: i32,
    distance: i32,
}

impl Add for PointWithDistance {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            distance: self.distance + other.distance,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Axis { X, Y }

trait Between {
    fn start(&self) -> PointWithDistance;
    fn end(&self) -> PointWithDistance;

    fn parallel_axis(&self) -> Axis {
        if self.start().x == self.end().x {
            Axis::Y
        } else {
            Axis::X
        }
    }

    fn length(&self) -> i32 {
        (self.start().x - self.end().x).abs() + (self.start().y - self.end().y).abs()
    }

    fn bottom(&self) -> i32 {
        min(self.start().y, self.end().y)
    }

    fn top(&self) -> i32 {
        max(self.start().y, self.end().y)
    }

    fn left(&self) -> i32 {
        min(self.start().x, self.end().x)
    }

    fn right(&self) -> i32 {
        max(self.start().x, self.end().x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: PointWithDistance,
    end: PointWithDistance,
}

impl Between for Line {
    fn start(&self) -> PointWithDistance {
        self.start
    }

    fn end(&self) -> PointWithDistance {
        self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WirePiece {
    line: Line,
    cumulative_length: i32,
}

impl Between for WirePiece {
    fn start(&self) -> PointWithDistance {
        self.line.start
    }

    fn end(&self) -> PointWithDistance {
        self.line.end
    }
}

fn parse_line(line: &str) -> Option<Vec<Direction>> {
    line.split(',')
        .map(|dir| {
            let count = dir.chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .ok()?;
            match dir.chars().next()? {
                'U' => Some(Direction::UP(count)),
                'D' => Some(Direction::DOWN(count)),
                'L' => Some(Direction::LEFT(count)),
                'R' => Some(Direction::RIGHT(count)),
                _ => None
            }
        }).collect()
}

fn read_input() -> Result<Vec<Vec<Direction>>> {
    let input = File::open("input/day03.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| parse_line(line?.as_ref()).context("f"))
        .collect()
}

fn to_points(directions: &Vec<Direction>) -> Vec<PointWithDistance> {
    let mut current_point = PointWithDistance { x: 0, y: 0, distance: 0 };
    once(current_point).chain(directions.iter().map(|dir| {
        let change = to_point_change(*dir);
        current_point = current_point + change;
        return current_point;
    })).collect()
}

fn to_wire_pieces(directions: &Vec<Direction>) -> Vec<WirePiece> {
    let points = to_points(directions).into_iter();
    let end_points = points.clone().skip(1);
    let mut cumulative_length = 0;
    points.zip(end_points)
        .map(|(start, end)| Line { start, end })
        .map(|line| {
            cumulative_length += line.length();
            WirePiece { line, cumulative_length }
        })
        .collect()
}

fn find_intersection(first: &WirePiece, second: &WirePiece) -> Option<PointWithDistance> {
    if first.parallel_axis() == second.parallel_axis() {
        return None;
    }
    let (x_line, y_line) = if first.parallel_axis() == Axis::X { (first, second) } else { (second, first) };
    if x_line.start().y > y_line.bottom()
        && x_line.start().y < y_line.top()
        && y_line.start().x > x_line.left()
        && y_line.start().x < x_line.right() {
        let (x, y) = (y_line.start().x, x_line.start().y);
        return Some(PointWithDistance {
            x, y,
            distance: x_line.cumulative_length + y_line.cumulative_length - (y_line.end().y - y).abs() - (x_line.end().x - x).abs(),
        });
    }
    None
}

fn part1(directions: &Vec<Vec<Direction>>) -> Option<i32> {
    let lines: Vec<Vec<WirePiece>> = directions.iter()
        .map(|dirs| to_wire_pieces(dirs))
        .collect();
    let (first, second) = (lines.get(0).unwrap(), lines.get(1).unwrap());
    iproduct!(first, second)
        .map(|(f, s)| find_intersection(f, s))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .map(|p| p.x.abs() + p.y.abs())
        .min()
}

fn part2(directions: &Vec<Vec<Direction>>) -> Result<i32> {
    let pieces: Vec<Vec<WirePiece>> = directions.iter()
        .map(|dirs| to_wire_pieces(dirs))
        .collect();
    let (first, second) = (pieces.get(0).unwrap(), pieces.get(1).unwrap());
    let intersection_distances: Vec<i32> = iproduct!(first, second)
        .map(|(f, s)| find_intersection(f, s))
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .map(|p| p.distance)
        .collect();

    let min_value = intersection_distances.into_iter().min().unwrap();
    Ok(min_value)
}

#[test]
fn test_part2() -> Result<()> {
    let input = vec![
        parse_line("U8,R5,D5,L5").context("parse failed")?,
        parse_line("R7,U6,L4,D4").context("parse failed")?,
    ];

    assert_eq!(part2(&input)?, 30);

    let input2 = vec![
        parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72").context("parse failed")?,
        parse_line("U62,R66,U55,R34,D71,R55,D58,R83").context("parse failed")?,
    ];

    assert_eq!(part2(&input2)?, 610);

    let input3 = vec![
        parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").context("parse failed")?,
        parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").context("parse failed")?,
    ];

    assert_eq!(part2(&input3)?, 410);

    Ok(())
}

fn main() -> Result<()> {
    let directions = read_input()?;

    let result1 = part1(&directions).unwrap();
    println!("part 1: {}", result1);

    let result2 = part2(&directions).unwrap();
    println!("part 2: {}", result2);

    Ok(())
}
