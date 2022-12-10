use anyhow::Result;
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
    fn cycle_length(&self) -> i32 {
        match self {
            Instr::AddX(_) => 2,
            Instr::Noop => 1,
        }
    }

    fn apply(&self, x: i32) -> i32 {
        match self {
            Instr::AddX(a) => a + x,
            Instr::Noop => x,
        }
    }
}

fn part1(instructions: &[Instr]) -> i32 {
    instructions
        .iter()
        .fold((0, 0, 1), |(cycle, sum, reg_x), instr| {
            let new_cycle = cycle + instr.cycle_length();
            let new_cycle_offset = (new_cycle + 20) % 40;
            let should_measure = new_cycle_offset < (cycle + 20) % 40;
            (
                new_cycle,
                sum + should_measure as i32 * (new_cycle - new_cycle_offset) * reg_x,
                instr.apply(reg_x),
            )
        })
        .1
}

fn part2(instructions: &[Instr]) -> String {
    instructions
        .iter()
        .fold(
            (0, 1, "".to_string()),
            |(mut cycle, reg_x, mut display), instr| {
                let to_draw = (0..instr.cycle_length())
                    .map(|_| {
                        let pos_to_check = cycle % 40;
                        let should_draw_pixel = (reg_x - 1..=reg_x + 1).any(|x| x == pos_to_check);
                        let maybe_newline = if pos_to_check == 39 { "\n" } else { "" };
                        let to_draw = if should_draw_pixel { "#" } else { "." };
                        cycle += 1;

                        to_draw.to_string() + maybe_newline
                    })
                    .collect::<String>();
                display += to_draw.as_str();
                (cycle, instr.apply(reg_x), display)
            },
        )
        .2
        .trim()
        .to_string()
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
