use std::str::FromStr;

use anyhow::Result;
use std::marker::PhantomData;
use utils::{read_file, InputParseError};

trait SpaceSetting {}

struct WithSpaces;
impl SpaceSetting for WithSpaces {}

struct WithoutSpaces;
impl SpaceSetting for WithoutSpaces {}

struct Race {
    duration: i64,
    distance_to_beat: i64,
}

struct Races<T: SpaceSetting = WithSpaces> {
    races: Vec<Race>,
    _phantom: PhantomData<T>,
}

impl FromStr for Races<WithSpaces> {
    type Err = InputParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut values = s
            .lines()
            .map(|l| l.trim_start_matches(|c: char| !c.is_numeric()))
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<i64>>()
            });
        let races = values
            .next()
            .unwrap()
            .iter()
            .zip(values.next().unwrap())
            .map(|(&duration, distance_to_beat)| Race {
                duration,
                distance_to_beat,
            })
            .collect();
        Ok(Races {
            races,
            _phantom: PhantomData,
        })
    }
}

impl FromStr for Races<WithoutSpaces> {
    type Err = InputParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut lines = s
            .lines()
            .map(|l| l.chars().filter(|c| c.is_numeric()).collect::<String>())
            .map(|n| n.parse().unwrap());
        let race = Race {
            duration: lines.next().unwrap(),
            distance_to_beat: lines.next().unwrap(),
        };
        Ok(Races {
            races: vec![race],
            _phantom: PhantomData,
        })
    }
}

fn find_roots_of_quadratic_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b * b - 4.0 * a * c;
    let sqrt_discriminant = discriminant.sqrt();
    let root1 = (-b + sqrt_discriminant) / (2.0 * a);
    let root2 = (-b - sqrt_discriminant) / (2.0 * a);
    (root1.min(root2), root2.max(root1))
}

fn run<T: SpaceSetting>(races: &Races<T>) -> u32 {
    races
        .races
        .iter()
        .map(|r| {
            find_roots_of_quadratic_equation(-1.0, r.duration as f64, -r.distance_to_beat as f64)
        })
        .map(|(root1, root2)| {
            let first_integer_between_roots = (root1 + 1.0).floor() as u32;
            let last_integer_between_roots = (root2 - 1.0).ceil() as u32;
            last_integer_between_roots - first_integer_between_roots + 1
        })
        .product()
}

fn part1(races: &Races<WithSpaces>) -> u32 {
    run(races)
}

fn part2(races: &Races<WithoutSpaces>) -> u32 {
    run(races)
}

fn main() -> Result<()> {
    let races = read_file("input/day06.txt")?;
    let result = part1(&races);
    println!("part 1: {}", result);
    let races = read_file("input/day06.txt")?;
    let result = part2(&races);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let races = read_file("input/test/day06.txt")?;
    let result = part1(&races);
    assert_eq!(result, 288);
    let races = read_file("input/test/day06.txt")?;
    let result = part2(&races);
    assert_eq!(result, 71503);
    Ok(())
}
