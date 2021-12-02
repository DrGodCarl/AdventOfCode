use anyhow::Result;
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{} {0}")]
#[display(style = "snake_case")]
enum Instr {
    Forward(isize),
    Up(isize),
    Down(isize),
}

fn part1(numbers: &[Instr]) -> isize {
    let point = numbers.iter().fold((0, 0), |(x, y), instr| match instr {
        Instr::Forward(n) => (x + n, y),
        Instr::Up(n) => (x, y - n),
        Instr::Down(n) => (x, y + n),
    });
    point.0 * point.1
}

fn part2(numbers: &[Instr]) -> isize {
    let point = numbers
        .iter()
        .fold((0, 0, 0), |(x, y, aim), instr| match instr {
            Instr::Forward(n) => (x + n, y + aim * n, aim),
            Instr::Up(n) => (x, y, aim - n),
            Instr::Down(n) => (x, y, aim + n),
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
