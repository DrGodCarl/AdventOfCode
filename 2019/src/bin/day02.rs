#[macro_use]
extern crate itertools;
#[macro_use]
extern crate anyhow;

use anyhow::Result;
use intcode::Computer;
use std::collections::VecDeque;
use common::read_input;

fn run_with(noun: i32, verb: i32, master_program: &[i32]) -> Result<Vec<i32>> {
    let mut program = master_program.to_owned();
    program[1] = noun;
    program[2] = verb;

    let speak = &mut |i| println!("computer says: {}", i);
    let mut computer = Computer::new(program, speak, VecDeque::new());
    computer.run()?;
    return Ok(computer.mem)
}

fn part1(master_program: &[i32]) -> i32 {
    let result = run_with(12, 2, master_program);
    result.unwrap()[0]
}

fn part2(master_program: &[i32]) -> Result<i32> {
    for (noun, verb) in iproduct!(0..100, 0..100) {
        let result = Some(run_with(noun, verb, master_program)?[0])
            .filter(|r| *r == 19690720)
            .map(|_| 100 * noun + verb);
        if result.is_some() {
            return Ok(result.unwrap());
        }
    }
    Err(anyhow!("not found."))
}

fn main() -> Result<()> {
    let program = read_input("input/day02.txt")?;

    let result1 = part1(&program);
    println!("part 1: {}", result1);

    let result2 = part2(&program);
    println!("part 2: {}", result2.unwrap());

    Ok(())
}

#[test]
fn test_parts() {
    let program = read_input("input/day02.txt").unwrap();

    let result1 = part1(&program);
    assert_eq!(result1, 7210630);

    let result2 = part2(&program);
    assert_eq!(result2.unwrap(), 3892);
}
