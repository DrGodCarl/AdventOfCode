#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use anyhow::Result;
use regex::Regex;
use utils::{read_file, InputParseError};

#[derive(Copy, Clone)]
struct Mask {
    and: u64,
    or: u64,
}

enum Instr {
    Mask(Mask),
    Mem(u64, u64),
}

impl FromStr for Instr {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"mask = ([01X]+)").unwrap();
            static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }
        fn make_mask(mask: &str) -> Result<Instr, ParseIntError> {
            let and = u64::from_str_radix(mask.replace("X", "1").as_str(), 2)?;
            let or = u64::from_str_radix(mask.replace("X", "0").as_str(), 2)?;
            Ok(Instr::Mask(Mask { and, or }))
        }
        fn make_mem(addr: &str, val: &str) -> Result<Instr, ParseIntError> {
            let addr = addr.parse()?;
            let val = val.parse()?;
            Ok(Instr::Mem(addr, val))
        }
        let mask = MASK_RE.captures(s).and_then(|c| c.get(1));
        let mem_components = MEM_RE.captures(s).and_then(|c| c.get(1).zip(c.get(2)));

        match (mask, mem_components) {
            (Some(mask), _) => make_mask(mask.as_str()).map_err(|_| InputParseError),
            (_, Some((addr, val))) => {
                make_mem(addr.as_str(), val.as_str()).map_err(|_| InputParseError)
            }
            _ => Err(InputParseError),
        }
    }
}

struct Computer {
    state: HashMap<u64, u64>,
    instructions: Vec<Instr>,
    instr_idx: usize,
    mask: Option<Mask>,
}

impl Computer {
    fn new(instructions: Vec<Instr>) -> Self {
        Computer {
            state: HashMap::new(),
            instructions,
            instr_idx: 0,
            mask: None,
        }
    }

    fn apply_mask(&self, val: u64) -> u64 {
        self.mask.map(|m| val & m.and | m.or).unwrap_or(val)
    }

    fn tick(&mut self) -> Option<()> {
        if self.instr_idx >= self.instructions.len() {
            return None;
        }
        match self.instructions[self.instr_idx] {
            Instr::Mask(mask) => self.mask = Some(mask),
            Instr::Mem(addr, val) => {
                self.state.insert(addr, self.apply_mask(val));
            }
        }
        self.instr_idx += 1;
        Some(())
    }
}

impl FromStr for Computer {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split('\n')
            .map(|l| l.parse())
            .collect::<Result<_, _>>()
            .map(Computer::new)
    }
}

fn part1(computer: &mut Computer) -> u64 {
    while computer.tick() != None {}
    computer.state.values().sum()
}

fn part2() {}

fn main() -> Result<()> {
    let mut computer = read_file("input/day14.txt")?;
    let result = part1(&mut computer);
    println!("part 1: {}", result);

    let result = part2();
    // println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let mut computer = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
        mem[8] = 11\n\
        mem[7] = 101\n\
        mem[8] = 0"
            .parse()?;
        let result = part1(&mut computer);
        assert_eq!(result, 165);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        Ok(())
    }
}
