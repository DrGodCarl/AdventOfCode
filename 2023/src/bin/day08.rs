use std::{
    collections::{HashMap, HashSet},
    iter,
};

use anyhow::Result;
use itertools::{FoldWhile, Itertools};
use parse_display::FromStr;
use utils::{read_file, InputParseError};

#[derive(FromStr, Clone, Copy)]
enum Instruction {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(FromStr)]
#[display("{current} = ({left}, {right})")]
struct Direction {
    current: String,
    left: String,
    right: String,
}

struct Map {
    instructions: Vec<Instruction>,
    directions: Vec<Direction>,
}

impl std::str::FromStr for Map {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instruction_str = lines.next().unwrap();
        let instructions = instruction_str
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        lines.next();
        let directions = lines.map(|l| l.parse().unwrap()).collect();
        Ok(Map {
            instructions,
            directions,
        })
    }
}

fn to_hash_map(map: &Map) -> HashMap<&String, (&String, &String)> {
    map.directions
        .iter()
        .map(|d| (&d.current, (&d.left, &d.right)))
        .collect()
}

#[allow(dead_code)]
fn detect_loop(map: &Map) {
    let location_to_options = to_hash_map(map);
    let a_keys: Vec<_> = location_to_options
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect();
    let z_keys: HashSet<_> = location_to_options
        .keys()
        .filter(|k| k.ends_with('Z'))
        .collect();
    let a_to_z_keys: HashMap<_, _> = a_keys
        .iter()
        .map(|&&k| {
            if let FoldWhile::Done(x) = iter::repeat_with(|| map.instructions.clone())
                .flatten()
                .fold_while((k, 1 as u64), |(loc, count), instr| {
                    let next_step = location_to_options
                        .get(&loc)
                        .map(|(left, right)| match instr {
                            Instruction::Left => left,
                            Instruction::Right => right,
                        })
                        .unwrap();
                    if z_keys.contains(next_step) {
                        return FoldWhile::Done((next_step, count));
                    }
                    FoldWhile::Continue((next_step, count + 1))
                })
            {
                (k, x)
            } else {
                panic!("no result")
            }
        })
        .collect();
    let z_key_loop_length: HashMap<_, _> = z_keys
        .iter()
        .map(|&&k| {
            if let FoldWhile::Done(x) = iter::repeat_with(|| map.instructions.clone())
                .flatten()
                .fold_while((k, 1 as u64), |(loc, count), instr| {
                    let next_step = location_to_options
                        .get(&loc)
                        .map(|(left, right)| match instr {
                            Instruction::Left => left,
                            Instruction::Right => right,
                        })
                        .unwrap();
                    if *next_step == k {
                        return FoldWhile::Done((next_step, count));
                    }
                    FoldWhile::Continue((next_step, count + 1))
                })
            {
                (k, x.1)
            } else {
                panic!("no result")
            }
        })
        .collect();
    for (a, (z, a_to_z_count)) in a_to_z_keys {
        let z_to_z_count = z_key_loop_length.get(z).unwrap();
        println!(
            "{} -> {}({}), {} -> {}({})",
            a, z, a_to_z_count, z, z, z_to_z_count
        );
    }
}

fn part1(map: &Map) -> u64 {
    let location_to_options = to_hash_map(map);

    if let FoldWhile::Done((_, result)) = iter::repeat_with(|| map.instructions.clone())
        .flatten()
        .fold_while((&"AAA".to_string(), 1 as u64), |(loc, count), instr| {
            let next_step = location_to_options
                .get(&loc)
                .map(|(left, right)| match instr {
                    Instruction::Left => left,
                    Instruction::Right => right,
                })
                .unwrap();
            if next_step == &"ZZZ" {
                return FoldWhile::Done((next_step, count));
            }
            FoldWhile::Continue((next_step, count + 1))
        })
    {
        result
    } else {
        panic!("no result")
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(nums: Vec<u64>) -> u64 {
    let mut nums = nums;
    nums.sort();
    let mut result = nums[0];
    for &num in &nums[1..] {
        result = result * num / gcd(result, num);
    }
    result
}

fn part2(map: &Map) -> u64 {
    let location_to_options = to_hash_map(map);
    let a_keys: Vec<_> = location_to_options
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect();
    let z_keys: HashSet<_> = location_to_options
        .keys()
        .filter(|k| k.ends_with('Z'))
        .collect();
    let loop_lengths: Vec<_> = a_keys
        .iter()
        .map(|&&k| {
            if let FoldWhile::Done((_, count)) = iter::repeat_with(|| map.instructions.clone())
                .flatten()
                .fold_while((k, 1 as u64), |(loc, count), instr| {
                    let next_step = location_to_options
                        .get(&loc)
                        .map(|(left, right)| match instr {
                            Instruction::Left => left,
                            Instruction::Right => right,
                        })
                        .unwrap();
                    // Distance from a to z is the same as the distance from z to z,
                    // as confirmed by detect_loop() + visual inspection.
                    if z_keys.contains(next_step) {
                        return FoldWhile::Done((next_step, count));
                    }
                    FoldWhile::Continue((next_step, count + 1))
                })
            {
                count
            } else {
                panic!("no result")
            }
        })
        .collect();
    lcm(loop_lengths)
}

fn main() -> Result<()> {
    let map = read_file("input/day08.txt")?;
    let result = part1(&map);
    println!("part 1: {}", result);
    let result = part2(&map);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let map: Map = read_file("input/test/day08.txt")?;
    let result = part1(&map);
    assert_eq!(result, 2);
    let map: Map = read_file("input/test/day08_2.txt")?;
    let result = part1(&map);
    assert_eq!(result, 6);
    let map: Map = read_file("input/test/day08_3.txt")?;
    let result = part2(&map);
    assert_eq!(result, 6);
    Ok(())
}
