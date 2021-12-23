use std::{collections::HashSet, ops::Sub};

use anyhow::Result;
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy, Eq, Hash)]
#[display("{}")]
#[display(style = "snake_case")]
enum State {
    On,
    Off,
}

impl Default for State {
    fn default() -> Self {
        State::Off
    }
}

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy, Default, Eq, Hash)]
#[display("{left}..{right}")]
struct SideRange {
    left: i64,
    right: i64,
}

impl SideRange {
    fn overlaps(&self, other: &Self) -> bool {
        self.right >= other.left && self.left <= other.right
    }

    fn length(&self) -> i64 {
        self.right - self.left + 1
    }
}

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Default, Eq, Hash, Copy)]
#[display("{state} x={x},y={y},z={z}")]
struct Cuboid {
    state: State,
    x: SideRange,
    y: SideRange,
    z: SideRange,
}

impl Sub for Cuboid {
    type Output = HashSet<Self>;

    fn sub(self, other: Self) -> Self::Output {
        if !self.intersects(&other) {
            return HashSet::from([self]);
        }
        let x_range = SideRange {
            left: self.x.left.max(other.x.left),
            right: self.x.right.min(other.x.right),
        };
        let y_range = SideRange {
            left: self.y.left.max(other.y.left),
            right: self.y.right.min(other.y.right),
        };
        let z_range = SideRange {
            left: self.z.left.max(other.z.left),
            right: self.z.right.min(other.z.right),
        };
        let mut res = HashSet::new();
        if other.x.left > self.x.left {
            let left_chunk = Cuboid {
                state: self.state,
                x: SideRange {
                    left: self.x.left,
                    right: x_range.left - 1,
                },
                ..self
            };
            res.insert(left_chunk);
        }
        if other.x.right < self.x.right {
            let right_chunk = Cuboid {
                state: self.state,
                x: SideRange {
                    left: x_range.right + 1,
                    right: self.x.right,
                },
                ..self
            };
            res.insert(right_chunk);
        }
        if other.y.left > self.y.left {
            let top_chunk = Cuboid {
                state: self.state,
                x: x_range,
                y: SideRange {
                    left: self.y.left,
                    right: y_range.left - 1,
                },
                ..self
            };
            res.insert(top_chunk);
        }
        if other.y.right < self.y.right {
            let bottom_chunk = Cuboid {
                state: self.state,
                x: x_range,
                y: SideRange {
                    left: y_range.right + 1,
                    right: self.y.right,
                },
                ..self
            };
            res.insert(bottom_chunk);
        }
        if other.z.left > self.z.left {
            let front_chunk = Cuboid {
                state: self.state,
                x: x_range,
                y: y_range,
                z: SideRange {
                    left: self.z.left,
                    right: z_range.left - 1,
                },
            };
            res.insert(front_chunk);
        }
        if other.z.right < self.z.right {
            let back_chunk = Cuboid {
                state: self.state,
                x: x_range,
                y: y_range,
                z: SideRange {
                    left: z_range.right + 1,
                    right: self.z.right,
                },
            };
            res.insert(back_chunk);
        }
        res
    }
}

impl Cuboid {
    fn intersects(&self, other: &Self) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y) && self.z.overlaps(&other.z)
    }

    fn size(&self) -> i64 {
        self.x.length() * self.y.length() * self.z.length()
    }
}

fn run(cubes: &[Cuboid]) -> i64 {
    let cubes = cubes.to_vec();
    let mut all_on = HashSet::new();
    for cube in cubes {
        all_on = all_on.iter().flat_map(|&c| c - cube).collect();
        if cube.state == State::On {
            all_on.insert(cube);
        }
    }
    all_on.iter().map(|&c| c.size()).sum()
}

fn part1(cubes: &[Cuboid]) -> i64 {
    let cubes: Vec<Cuboid> = cubes
        .iter()
        .filter(|&c| {
            c.x.left >= -50
                && c.x.right <= 50
                && c.y.left >= -50
                && c.y.right <= 50
                && c.z.left >= -50
                && c.z.right <= 50
        })
        .copied()
        .collect();
    run(&cubes)
}

fn part2(cubes: &[Cuboid]) -> i64 {
    run(cubes)
}

fn main() -> Result<()> {
    let cubes = read_lines("input/day22.txt")?;
    let result = part1(&cubes);
    println!("part 1: {}", result);
    let result = part2(&cubes);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let cubes = read_lines("input/test/day22_small.txt")?;
    let result = part1(&cubes);
    assert_eq!(result, 39);

    let cubes = read_lines("input/test/day22_p1.txt")?;
    let result = part1(&cubes);
    assert_eq!(result, 590784);

    let cubes = read_lines("input/test/day22_p2.txt")?;
    let result = part2(&cubes);
    assert_eq!(result, 2758514936282235);

    Ok(())
}
