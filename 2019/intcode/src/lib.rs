#[macro_use]
extern crate anyhow;

use anyhow::{Result, Context};
use std::collections::VecDeque;

type Memory = Vec<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Parameter {
    Position { address: usize },
    Immediate { value: i32 },
}

impl Parameter {
    fn value(self, mem: &Memory) -> Result<i32> {
        let result = match self {
            Parameter::Immediate { value } => value,
            Parameter::Position { address } => *mem.get(address)
                .context("Nothing at memory address")?,
        };
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add(Parameter, Parameter, usize),
    Multiply(Parameter, Parameter, usize),
    Input(usize),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),
    Terminate,
}

impl Operation {
    fn exec(&self, computer: &mut Computer) -> Result<()> {
        match self {
            Operation::Add(param1, param2, addr) => {
                computer.pointer += 4;
                Ok(computer.mem[*addr] = param1.value(&computer.mem)? + param2.value(&computer.mem)?)
            }
            Operation::Multiply(param1, param2, addr) => {
                computer.pointer += 4;
                Ok(computer.mem[*addr] = param1.value(&computer.mem)? * param2.value(&computer.mem)?)
            }
            Operation::Input(addr) => {
                computer.pointer += 2;
                Ok(computer.mem[*addr] = computer.inputs.pop_front().context("No input. Error.")?)
            }
            Operation::Output(param) => {
                computer.pointer += 2;
                Ok((computer.out)(param.value(&computer.mem)?))
            }
            Operation::JumpIfTrue(param1, param2) => {
                Ok(if param1.value(&computer.mem)? == 0 {
                    computer.pointer += 3;
                } else {
                    computer.pointer = param2.value(&computer.mem)? as usize;
                })
            }
            Operation::JumpIfFalse(param1, param2) => {
                Ok(if param1.value(&computer.mem)? != 0 {
                    computer.pointer += 3;
                } else {
                    computer.pointer = param2.value(&computer.mem)? as usize;
                })
            }
            Operation::LessThan(param1, param2, addr) => {
                computer.pointer += 4;
                Ok(if param1.value(&computer.mem)? < param2.value(&computer.mem)? {
                    computer.mem[*addr] = 1;
                } else {
                    computer.mem[*addr] = 0;
                })
            }
            Operation::Equals(param1, param2, addr) => {
                computer.pointer += 4;
                Ok(if param1.value(&computer.mem)? == param2.value(&computer.mem)? {
                    computer.mem[*addr] = 1;
                } else {
                    computer.mem[*addr] = 0;
                })
            }
            Operation::Terminate => Ok(computer.terminated = true)
        }
    }
}

pub struct Computer<'a> {
    pub mem: Memory,
    out: &'a mut dyn FnMut(i32),
    pub inputs: VecDeque<i32>,
    pointer: usize,
    terminated: bool,
}

impl Computer<'_> {
    pub fn new(mem: Memory, out: &mut impl FnMut(i32), inputs: VecDeque<i32>) -> Computer {
        Computer { mem, out, inputs, pointer: 0, terminated: false }
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.terminated {
            let op = self.next_operation()?;
            op.exec(self)?;
        }

        Ok(())
    }

    fn next_operation(&self) -> Result<Operation> {
        let code_and_modes = self.mem.get(self.pointer).context("Memory pointer out of range.")?;
        let just_code = (code_and_modes % 100) as usize;
        let just_modes = code_and_modes / 100;
        self.create_operation(just_code, just_modes)
    }

    fn create_operation(&self, code: usize, modes: i32) -> Result<Operation> {
        let result = match code {
            1 => Operation::Add(
                mode_for_position(modes, 0)(self.get_memory(1)?),
                mode_for_position(modes, 1)(self.get_memory(2)?),
                to_address(self.get_memory(3)?)),
            2 => Operation::Multiply(
                mode_for_position(modes, 0)(self.get_memory(1)?),
                mode_for_position(modes, 1)(self.get_memory(2)?),
                to_address(self.get_memory(3)?)),
            3 => Operation::Input(to_address(self.get_memory(1)?)),
            4 => Operation::Output(mode_for_position(modes, 0)(self.get_memory(1)?)),
            5 => Operation::JumpIfTrue(
                mode_for_position(modes, 0)(self.get_memory(1)?),
                mode_for_position(modes, 1)(self.get_memory(2)?)),
            6 => Operation::JumpIfFalse(
                mode_for_position(modes, 0)(self.get_memory(1)?),
                mode_for_position(modes, 1)(self.get_memory(2)?)),
            7 => Operation::LessThan(
                mode_for_position(modes, 0)(self.get_memory(1)?),
                mode_for_position(modes, 1)(self.get_memory(2)?),
                to_address(self.get_memory(3)?)),
            8 => Operation::Equals(
                mode_for_position(modes, 0)(self.get_memory(1)?),
                mode_for_position(modes, 1)(self.get_memory(2)?),
                to_address(self.get_memory(3)?)),
            99 => Operation::Terminate,
            _ => return Err(anyhow!("Something went wrong creating an operation with code {}.", code))
        };
        Ok(result)
    }

    fn get_memory(&self, offset: usize) -> Result<&i32> {
        Ok(self.mem.get((self.pointer) + offset).context("Failed to access memory.")?)
    }
}

fn mode_for_position(mode_indicator: i32, position: u32) -> fn(&i32) -> Parameter {
    let mode = (mode_indicator / ((10 as i32).pow(position))) % 10;
    match mode {
        0 => |i| Parameter::Position { address: to_address(i) },
        _ => |i| Parameter::Immediate { value: *i }
    }
}

fn to_address(would_be_address: &i32) -> usize {
    *would_be_address as usize
}


#[cfg(test)]
mod tests {
    use crate::Computer;
    use std::collections::VecDeque;

    #[test]
    fn test_execution() {
        let mut computer = Computer {
            mem: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            out: &mut |_| {},
            inputs: VecDeque::new(),
            pointer: 0,
            terminated: false,
        };
        computer.run();
        let expected: Vec<i32> = vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50);
        assert_eq!(
            computer.mem.clone(),
            expected
        );
    }

    #[test]
    fn test_execution_less_than() {
        let program = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                           1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                           999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let mut result = 0;
        let update = &mut |i| result = i;
        let mut computer = Computer::new(program.clone(), update, VecDeque::from(vec![7]));
        computer.run();
        assert_eq!(result, 999);
    }

    #[test]
    fn test_execution_equal_to() {
        let program = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                           1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                           999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let mut result = 0;
        let update = &mut |i| result = i;
        let mut computer = Computer::new(program.clone(), update, VecDeque::from(vec![8]));
        computer.run();
        assert_eq!(result, 1000);
    }

    #[test]
    fn test_execution_greater_than() {
        let program = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                           1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                           999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        let mut result = 0;
        let update = &mut |i| result = i;
        let mut computer = Computer::new(program.clone(), update, VecDeque::from(vec![9]));
        computer.run();
        assert_eq!(result, 1001);
    }
}
