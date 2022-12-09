use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;
use parse_display::FromStr;
use utils::read_lines;

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[display("{direction} {distance}")]
struct Instruction {
    direction: Direction,
    distance: u8,
}

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

fn run(instructions: &[Instruction], rope_size: u8) -> u32 {
    let rope = (0..rope_size - 1).map(|_| (0, 0)).collect_vec();
    let mut initial_tail_pos = HashSet::new();
    initial_tail_pos.insert((0, 0));
    instructions
        .iter()
        .flat_map(|i| (0..i.distance).map(|_| i.direction))
        .fold(
            (initial_tail_pos, rope, (0, 0)),
            |(mut tail_pos, cur_rope, h_pos), d| {
                let new_h_pos = match d {
                    Direction::Up => (h_pos.0, h_pos.1 + 1),
                    Direction::Down => (h_pos.0, h_pos.1 - 1),
                    Direction::Left => (h_pos.0 - 1, h_pos.1),
                    Direction::Right => (h_pos.0 + 1, h_pos.1),
                };
                let (_, rest_of_rope) = cur_rope.iter().fold(
                    (new_h_pos, Vec::new()),
                    |(effective_h_pos, mut new_rope), t_pos| {
                        let x_dist = effective_h_pos.0 - t_pos.0 as i32;
                        let y_dist = effective_h_pos.1 - t_pos.1 as i32;
                        let knot_moves = x_dist.abs() == 2 || y_dist.abs() == 2;

                        let new_t_pos = (
                            t_pos.0 + x_dist.signum() * knot_moves as i32,
                            t_pos.1 + y_dist.signum() * knot_moves as i32,
                        );

                        new_rope.push(new_t_pos);
                        (new_t_pos, new_rope)
                    },
                );
                tail_pos.insert(rest_of_rope.last().unwrap().clone());
                (tail_pos, rest_of_rope, new_h_pos)
            },
        )
        .0
        .len() as u32
}

fn part1(instructions: &[Instruction]) -> u32 {
    run(instructions, 2)
}

fn part2(instructions: &[Instruction]) -> u32 {
    run(instructions, 10)
}

fn main() -> Result<()> {
    let instructions = read_lines("input/day09.txt")?;
    let result = part1(&instructions);
    println!("part 1: {}", result);
    let result = part2(&instructions);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let instructions = read_lines("input/test/day09.txt")?;
    let result = part1(&instructions);
    assert_eq!(result, 13);

    let instructions = read_lines("input/test/day09.txt")?;
    let result = part2(&instructions);
    assert_eq!(result, 1);

    let instructions = read_lines("input/test/day09_2.txt")?;
    let result = part2(&instructions);
    assert_eq!(result, 36);

    Ok(())
}
