use std::{collections::HashSet, str::FromStr, string::ParseError};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use utils::read_file;

#[allow(clippy::upper_case_acronyms)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize),
}

struct Program {
    instructions: Vec<Instruction>,
    idx: usize,
    acc: isize,
    current_swap: Option<usize>,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions,
            idx: 0,
            acc: 0,
            current_swap: None,
        }
    }

    fn swap_nop_jmp(&mut self, index: usize) -> bool {
        match self.instructions[index] {
            Instruction::NOP(amt) => {
                self.instructions[index] = Instruction::JMP(amt);
                self.current_swap = Some(index);
                true
            }
            Instruction::ACC(_) => false,
            Instruction::JMP(amt) => {
                self.instructions[index] = Instruction::NOP(amt);
                self.current_swap = Some(index);
                true
            }
        }
    }

    fn reset(&mut self) {
        self.idx = 0;
        self.acc = 0;
        self.current_swap.map(|index| self.swap_nop_jmp(index));
        self.current_swap = None;
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instuction_re = Regex::new(r"(nop|acc|jmp) (\+|\-)(\d+)").unwrap();
        let instructions = instuction_re
            .captures_iter(s)
            .map(|c| c.get(1).zip(c.get(2)).zip(c.get(3)))
            .flatten()
            .map(|((instr, sign), amt)| {
                let sign_factor = match sign.as_str() {
                    "-" => -1,
                    _ => 1,
                };
                let amount = amt.as_str().parse::<isize>().unwrap() * sign_factor;
                match instr.as_str() {
                    "acc" => Instruction::ACC(amount),
                    "jmp" => Instruction::JMP(amount),
                    _ => Instruction::NOP(amount),
                }
            })
            .collect();
        Ok(Program::new(instructions))
    }
}

#[derive(Clone, Debug)]
struct ProgramState {
    idx: usize,
    acc: isize,
}

impl Iterator for Program {
    type Item = ProgramState;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_instruction = self.instructions.get(self.idx);
        if let Some(instruction) = maybe_instruction {
            match instruction {
                Instruction::ACC(amt) => {
                    self.acc += amt;
                    self.idx += 1
                }
                Instruction::JMP(amt) => self.idx = ((self.idx as isize) + amt) as usize,
                Instruction::NOP(_) => self.idx += 1,
            };
            Some(ProgramState {
                idx: self.idx,
                acc: self.acc,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Debugger {
    visited: HashSet<usize>,
    terminal_state: Option<ProgramState>,
}

impl Debugger {
    fn new() -> Self {
        let mut init = HashSet::new();
        init.insert(0);
        Debugger {
            visited: init,
            terminal_state: None,
        }
    }
}

fn detect_loop(program: &mut Program) -> Result<ProgramState, ProgramState> {
    match program
        .tuple_windows()
        .try_fold(Debugger::new(), |mut debug, (curr, next)| {
            if debug.visited.contains(&next.idx) {
                Err(curr)
            } else {
                debug.visited.insert(curr.idx);
                debug.terminal_state = Some(next);
                Ok(debug)
            }
        }) {
        Ok(debug) => Ok(debug.terminal_state.unwrap()),
        Err(res) => Err(res),
    }
}

fn part1(program: &mut Program) -> Option<isize> {
    detect_loop(program).map_err(|state| state.acc).err()
}

fn part2(program: &mut Program) -> Option<isize> {
    for i in 0..program.instructions.len() {
        program.reset();
        if !program.swap_nop_jmp(i) {
            continue;
        }
        if let Ok(res) = detect_loop(program) {
            return Some(res.acc);
        }
    }
    None
}

fn main() -> Result<()> {
    let mut program: Program = read_file("input/day08.txt")?;
    let result = part1(&mut program).unwrap();
    println!("part 1: {}", result);
    program.reset();
    let result = part2(&mut program).unwrap();
    println!("part 2: {}", result);
    Ok(())
}
