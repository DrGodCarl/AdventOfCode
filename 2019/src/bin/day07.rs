use anyhow::Result;
use common::read_input;
use intcode::Computer;

fn part1(memory: &Vec<i32>) -> u32 {
    0
}

fn main() -> Result<()> {
    let memory = read_input("input/day07.txt")?;

    let result1 = part1(&memory);
    println!("part 1: {}", result1);
//
//    let result2 = part2(&orbit);
//    println!("part 2: {:?}", result2?);

    Ok(())
}
