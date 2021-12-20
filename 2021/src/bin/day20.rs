use std::{collections::HashMap, fs};

use anyhow::Result;
use itertools::Itertools;

type Point = (i32, i32);

struct ImageEncoding {
    algorithm: Vec<bool>,
    image: HashMap<Point, bool>,
    infinite_state: bool,
    bounds: (Point, Point),
}

fn get_neighborhood(p: &Point) -> Vec<Point> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .map(|(dy, dx)| (p.0 + dx, p.1 + dy))
        .collect()
}

impl ImageEncoding {
    fn new(algorithm: Vec<bool>, image: HashMap<Point, bool>) -> Self {
        let upper_bounds = image
            .keys()
            .fold((0, 0), |(a, b), p| (a.max(p.0), b.max(p.1)));
        Self {
            algorithm,
            image,
            infinite_state: false,
            bounds: ((-1, -1), (upper_bounds.0 + 1, upper_bounds.1 + 1)),
        }
    }

    fn new_value(&self, point: &Point) -> bool {
        let index = get_neighborhood(point)
            .iter()
            .map(|p| *self.image.get(p).unwrap_or(&self.infinite_state) as usize)
            .fold(0, |index, b| (index << 1) ^ b);
        self.algorithm[index]
    }

    fn update_bounds(&mut self) {
        let ((x_min, y_min), (x_max, y_max)) =
            self.image.keys().fold(((0, 0), (0, 0)), |(min, max), p| {
                (
                    (min.0.min(p.0), min.1.min(p.1)),
                    (max.0.max(p.0), max.1.max(p.1)),
                )
            });
        self.bounds = ((x_min - 1, y_min - 1), (x_max + 1, y_max + 1))
    }

    fn refresh_infinite_state(&mut self) {
        let num = if self.infinite_state { 511 } else { 0 };
        self.infinite_state = self.algorithm[num];
    }

    fn run(&mut self, times: u8) {
        for _ in 0..times {
            let ((x_min, y_min), (x_max, y_max)) = self.bounds;
            self.image = (x_min..=x_max)
                .cartesian_product(y_min..=y_max)
                .map(|p| (p, self.new_value(&p)))
                .collect();
            self.update_bounds();
            self.refresh_infinite_state();
        }
    }

    fn count_on(&self) -> usize {
        self.image.values().filter(|&b| *b).count()
    }
}

fn part1(image: &mut ImageEncoding) -> usize {
    image.run(2);
    image.count_on()
}

fn part2(image: &mut ImageEncoding) -> usize {
    image.run(48);
    image.count_on()
}

fn read_input(path: &str) -> Result<ImageEncoding> {
    let s = fs::read_to_string(path)?;
    let mut splt = s.split("\n\n");
    let algo_str = splt.next().unwrap();
    let image_str = splt.next().unwrap();

    let algorithm = algo_str.chars().map(|c| c == '#').collect();
    let image = image_str
        .split('\n')
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((col as i32, row as i32), c == '#'))
        })
        .collect();
    Ok(ImageEncoding::new(algorithm, image))
}

fn main() -> Result<()> {
    let mut image = read_input("input/day20.txt")?;
    let result = part1(&mut image);
    println!("part 1: {}", result);
    let result = part2(&mut image);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let mut image = read_input("input/test/day20.txt")?;
    let result = part1(&mut image);
    assert_eq!(result, 35);
    let result = part2(&mut image);
    assert_eq!(result, 3351);

    Ok(())
}
