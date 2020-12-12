use std::collections::HashSet;

use anyhow::{Context, Result};
use itertools::{Itertools, MinMaxResult};
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
enum Instr {
    #[display("N{0}")]
    North(isize),
    #[display("S{0}")]
    South(isize),
    #[display("E{0}")]
    East(isize),
    #[display("W{0}")]
    West(isize),
    #[display("L{0}")]
    Left(isize),
    #[display("R{0}")]
    Right(isize),
    #[display("F{0}")]
    Forward(isize),
}

struct State {
    bearing: isize,
    position: (isize, isize),
}

impl State {
    fn new() -> Self {
        State {
            bearing: 0,
            position: (0, 0),
        }
    }

    fn move_y(&self, y: isize) -> Self {
        State {
            bearing: self.bearing,
            position: (self.position.0, self.position.1 + y),
        }
    }

    fn move_x(&self, x: isize) -> Self {
        State {
            bearing: self.bearing,
            position: (self.position.0 + x, self.position.1),
        }
    }

    fn rotate(&self, deg: isize) -> Self {
        State {
            bearing: (self.bearing + deg).rem_euclid(360),
            position: self.position,
        }
    }
}

fn instruction_for_bearing(state: &State, distance: isize) -> Instr {
    match state.bearing {
        0 => Instr::East(distance),
        90 => Instr::North(distance),
        180 => Instr::West(distance),
        _ => Instr::South(distance),
    }
}

fn next_state(state: State, instruction: &Instr) -> State {
    match instruction {
        Instr::North(dist) => state.move_y(*dist),
        Instr::South(dist) => state.move_y(-dist),
        Instr::East(dist) => state.move_x(*dist),
        Instr::West(dist) => state.move_x(-dist),
        Instr::Left(angle) => state.rotate(*angle),
        Instr::Right(angle) => state.rotate(-angle),
        Instr::Forward(dist) => {
            let instr = instruction_for_bearing(&state, *dist);
            next_state(state, &instr)
        }
    }
}

fn drive_boat(state: State, instructions: &Vec<Instr>) -> State {
    instructions.iter().fold(state, next_state)
}

fn part1(instructions: &Vec<Instr>) -> usize {
    let initial = State::new();
    let result = drive_boat(initial, &instructions);
    (result.position.0.abs() + result.position.1.abs()) as usize
}

fn part2() {}

fn main() -> Result<()> {
    let instructions = read_lines("input/day12.txt")?;
    let result = part1(&instructions);
    println!("part 1: {}", result);

    // let result = part2(&numbers);
    // println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        Ok(())
    }
}
