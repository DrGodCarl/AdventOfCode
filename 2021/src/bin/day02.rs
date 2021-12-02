use anyhow::Result;
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
enum Instr {
    #[display("forward {0}")]
    Forward(isize),
    #[display("up {0}")]
    Up(isize),
    #[display("down {0}")]
    Down(isize),
}

fn part1(numbers: &[Instr]) -> isize {
    let point = numbers.iter().fold((0, 0), |acc, instr| match instr {
        Instr::Forward(x) => (acc.0 + x, acc.1),
        Instr::Up(y) => (acc.0, acc.1 - y),
        Instr::Down(y) => (acc.0, acc.1 + y),
    });
    point.0 * point.1
}

fn part2(numbers: &[Instr]) -> isize {
    let point = numbers.iter().fold((0, 0, 0), |acc, instr| match instr {
        Instr::Forward(x) => (acc.0 + x, acc.1 + acc.2 * x, acc.2),
        Instr::Up(y) => (acc.0, acc.1, acc.2 - y),
        Instr::Down(y) => (acc.0, acc.1, acc.2 + y),
    });
    point.0 * point.1
}

fn main() -> Result<()> {
    let instructions = read_lines("input/day02.txt")?;
    let result = part1(&instructions);
    println!("part 1: {}", result);
    let result = part2(&instructions);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let instructions = read_lines("input/test/day02.txt")?;
    let result = part1(&instructions);
    assert_eq!(result, 150);

    let result = part2(&instructions);
    assert_eq!(result, 900);

    Ok(())
}
