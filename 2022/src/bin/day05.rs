use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use parse_display::FromStr;
use utils::{read_sections, InputParseError, VecWrapper};

#[derive(FromStr, PartialEq, Debug, Clone)]
#[display("[{0}]")]
struct Crate(char);

#[derive(Clone)]
struct CrateStacks(HashMap<u32, Vec<Crate>>);

#[derive(FromStr, PartialEq, Debug)]
#[display("move {count} from {from} to {to}")]
struct Instruction {
    from: u32,
    to: u32,
    count: u32,
}

impl std::str::FromStr for CrateStacks {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _last = lines.next_back().unwrap();
        let stacks = lines
            .map(|l| {
                l.chars()
                    // The position of a crate either contains three spaces followed by a space, or a crate (e.g. [Z]) followed by a space.
                    .chunks(4)
                    .into_iter()
                    // Try to parse a crate if present.
                    .map(|c| c.collect::<String>().trim().parse::<Crate>())
                    .enumerate()
                    // Keep track of the index for each successfully parsed crate
                    .filter_map(|(i, maybe_crate)| maybe_crate.map(|c| (i as u32 + 1, c)).ok())
                    .collect::<Vec<_>>()
            })
            .rev()
            // From the bottom up, store them in the hashmap at the appropriate index.
            .fold(HashMap::new(), |mut acc, v| {
                v.into_iter().for_each(|(i, c)| {
                    acc.entry(i).or_insert_with(Vec::new).push(c);
                });
                acc
            });
        Ok(CrateStacks(stacks))
    }
}

fn run(stacks: &mut CrateStacks, instructions: &[Instruction], reverse_moved: bool) -> String {
    for instruction in instructions {
        let crates = stacks.0.get_mut(&instruction.from).unwrap();
        let mut crates = crates.split_off(crates.len() - instruction.count as usize);
        if reverse_moved {
            crates.reverse();
        }
        stacks.0.get_mut(&instruction.to).unwrap().extend(crates);
    }
    stacks
        .0
        .iter()
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .filter_map(|(_, v)| v.last().map(|c| c.0))
        .collect()
}

fn part1(stacks: &mut CrateStacks, instructions: &[Instruction]) -> String {
    run(stacks, instructions, true)
}

fn part2(stacks: &mut CrateStacks, instructions: &[Instruction]) -> String {
    run(stacks, instructions, false)
}

fn main() -> Result<()> {
    let (stacks, instructions): (CrateStacks, VecWrapper<Instruction>) =
        read_sections("input/day05.txt")?;
    let mut p1_stacks = stacks.clone();
    let result = part1(&mut p1_stacks, &instructions.0);
    println!("part 1: {}", result);
    let mut p2_stacks = stacks.clone();
    let result = part2(&mut p2_stacks, &instructions.0);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let (stacks, instructions): (CrateStacks, VecWrapper<Instruction>) =
        read_sections("input/test/day05.txt")?;
    let mut p1_stacks = stacks.clone();
    let result = part1(&mut p1_stacks, &instructions.0);
    assert_eq!(result, "CMZ");
    let mut p2_stacks = stacks.clone();
    let result = part2(&mut p2_stacks, &instructions.0);
    assert_eq!(result, "MCD");

    Ok(())
}
