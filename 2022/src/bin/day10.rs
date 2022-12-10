use std::iter;

use anyhow::Result;
use itertools::Itertools;
use parse_display::FromStr;
use utils::read_lines;

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq)]
enum Instr {
    #[display("addx {0}")]
    AddX(i32),
    #[display("noop")]
    Noop,
}

impl Instr {
    fn cycle_values(&self) -> Vec<i32> {
        match self {
            Instr::AddX(a) => vec![0, *a],
            Instr::Noop => vec![0],
        }
    }
}

fn part1(instructions: &[Instr]) -> i32 {
    instructions
        .iter()
        .flat_map(|i| i.cycle_values())
        .scan(1, |reg_x, val| {
            *reg_x = *reg_x + val;
            Some(*reg_x)
        })
        .enumerate()
        // cycles start at 1.
        .map(|(cycle, val)| (cycle as i32 + 1, val))
        // we want to measure from the end of the cycle before the desired one (e.g. what's the value at 19 for 20)
        .filter(|(cycle, _)| cycle % 40 == 19)
        // we're measuring at 19, but the cycle number we're looking for starts at 20.
        .map(|(cycle, val)| (cycle + 1) * val)
        .sum()
}

fn part2(instructions: &[Instr]) -> String {
    // Rust's scan is not consistent with my understanding of scan.
    // In Haskell, Kotlin, etc. scan will return the initial value
    // but Rust's scan will not. So we have to add it manually.
    iter::once(1)
        .chain(
            instructions
                .iter()
                .flat_map(|i| i.cycle_values())
                .scan(1, |reg_x, val| {
                    *reg_x = *reg_x + val;
                    Some(*reg_x)
                }),
        )
        .take(240)
        .enumerate()
        .map(|(cycle, val)| ((cycle % 40) as i32, val))
        .map(|(pixel, val)| {
            if (val - 1..=val + 1).contains(&pixel) {
                "#"
            } else {
                "."
            }
        })
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n")
}

fn main() -> Result<()> {
    let instructions = read_lines("input/day10.txt")?;
    let result = part1(&instructions);
    println!("part 1: {}", result);
    let result = part2(&instructions);
    println!("part 2: \n{}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let instructions = read_lines("input/test/day10.txt")?;
    let result = part1(&instructions);
    assert_eq!(result, 13140);

    let result = part2(&instructions);
    println!("{}", result);
    assert_eq!(
        result,
        // I swear there's some funky formatting thing I can do here
        // but I can't remember what.
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );

    Ok(())
}
