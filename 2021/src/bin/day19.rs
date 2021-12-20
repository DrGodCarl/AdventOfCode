#![feature(let_else)]

use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::Sub,
};

use anyhow::Result;
use itertools::Itertools;
use utils::InputParseError;

#[derive(parse_display::FromStr, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[display("{0},{1},{2}")]
struct Point(i32, i32, i32);

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Point {
    fn manhattan_magnitude(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

#[derive(parse_display::FromStr, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[display("--- scanner {0} ---")]
struct Id(u8);

#[derive(Clone, Debug)]
struct Scanner {
    id: Id,
    points: HashSet<Point>,
    full_rotations: HashMap<usize, HashSet<Point>>,
}

impl Scanner {
    fn new(id: Id, points: HashSet<Point>) -> Self {
        let full_rotations = rotations()
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

fn rotations() -> HashMap<usize, Transformation> {
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
        Point(-p.0, -p.2, -p.1)
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

struct TransformToBasis {
    scanner: Id,
    basis_scanner_id: Id,
    rotation: Transformation,
    translation: Point,
}

impl TransformToBasis {
    fn apply(&self, p: &Point) -> Point {
        let p = (self.rotation)(p);
        p - self.translation
    }
}

struct BasisTransformations {
    basis_transformation: HashMap<Id, TransformToBasis>,
}

impl BasisTransformations {
    fn new() -> Self {
        Self {
            basis_transformation: HashMap::new(),
        }
    }

    fn add(&mut self, from: Id, to: Id, rotation: Transformation, translation: &Point) {
        self.basis_transformation.insert(
            from,
            TransformToBasis {
                scanner: from,
                basis_scanner_id: to,
                rotation,
                translation: *translation,
            },
        );
    }

    fn apply(&self, scanner: &Id, point: &Point) -> Point {
        if *scanner == Id(0) {
            return *point;
        }
        let transform = &self.basis_transformation[scanner];
        let point = transform.apply(point);
        self.apply(&transform.basis_scanner_id, &point)
    }
}

fn both_parts(scanners: &[Scanner]) -> (usize, i32) {
    let mut scanner_iter = scanners.iter();
    let mut next_check = vec![scanner_iter.next().unwrap()];
    let mut to_scan: Vec<&Scanner> = scanner_iter.collect();
    let mut basis_transformations = BasisTransformations::new();
    let rotations = rotations();

    while let Some(scanner) = next_check.pop() {
        println!("Testing to map to scanner {:?}", scanner.id);
        let mut to_scan_next = vec![];
        for &other in to_scan.iter() {
            if other.full_rotations.iter().any(|(r_id, points)| {
                let counts = points
                    .iter()
                    .cartesian_product(scanner.points.iter())
                    .map(|(&rotated, &basis)| rotated - basis)
                    .counts();
                let Some((p, _)) = counts
                    .iter()
                    .find(|(_, &count)| count >= 12) else {
                        return false;
                    };
                basis_transformations.add(other.id, scanner.id, rotations[r_id], p);
                true
            }) {
                println!("Found match! Pushing {:?}", other.id);
                next_check.push(other);
            } else {
                to_scan_next.push(other);
            }
        }
        to_scan = to_scan_next;
    }

    let all_points: HashSet<_> = scanners
        .iter()
        .flat_map(|s| {
            let basis_transformations = &basis_transformations;
            s.points
                .iter()
                .map(move |p| basis_transformations.apply(&s.id, p))
        })
        .collect();

    let translations: Vec<_> = basis_transformations
        .basis_transformation
        .values()
        .map(|b| basis_transformations.apply(&b.scanner, &Point(0, 0, 0)))
        .collect();

    let max_dist = translations
        .iter()
        .cartesian_product(translations.iter())
        .map(|(&p1, &p2)| p1 - p2)
        .map(|p| p.manhattan_magnitude())
        .max()
        .unwrap();

    (all_points.len(), max_dist)
}

fn read_input(path: &str) -> Result<Vec<Scanner>> {
    let s = fs::read_to_string(path)?;
    Ok(s.split("\n\n")
        .map(|s| s.parse::<Scanner>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn main() -> Result<()> {
    let scanners = read_input("input/day19.txt")?;
    let (r1, r2) = both_parts(&scanners);
    println!("part 1: {}", r1);
    println!("part 2: {}", r2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let scanners: Vec<Scanner> = read_input("input/test/day19.txt")?;
        let (r1, r2) = both_parts(&scanners);
        assert_eq!(r1, 79);
        assert_eq!(r2, 3621);

        Ok(())
    }
}
