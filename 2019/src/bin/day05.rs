use anyhow::Result;
use intcode::Computer;
use common::read_input;
use std::collections::VecDeque;

fn run_with_input(program: Vec<i32>, input: i32) -> Result<i32> {
    let mut result = 0;
    let update = &mut |i| result = i;
    let mut computer = Computer::new(program, update, VecDeque::from(vec![input]));
    computer.run()?;
    return Ok(result)
}

fn part1(program: Vec<i32>) -> Result<i32> {
    run_with_input(program, 1)
}

fn part2(program: Vec<i32>) -> Result<i32> {
    run_with_input(program, 5)
}

fn main() -> Result<()> {
    let program = read_input("input/day05.txt")?;

    let result1 = part1(program.clone());
    println!("part 1: {}", result1?);

    let result2 = part2(program.clone());
    println!("part 2: {}", result2?);

    Ok(())
}

#[test]
fn test() {
    let program = read_input("input/day05.txt").unwrap();

    let result1 = part1(program.clone());
    assert_eq!(result1.unwrap(), 6069343);

    let result2 = part2(program.clone());
    assert_eq!(result2.unwrap(), 3188550);
}
