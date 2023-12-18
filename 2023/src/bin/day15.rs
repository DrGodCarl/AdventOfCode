use anyhow::Result;
use parse_display::FromStr;
use utils::read_comma_separated;

#[derive(Debug, PartialEq, Eq, Hash, Clone, FromStr)]
enum Instruction {
    #[display("{0}={1}")]
    SetFocalLength(String, u8),
    #[display("{0}-")]
    RemoveLens(String),
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize)
        .fold(0, |acc, c| (acc + c) * 17 % 256)
}

impl Instruction {
    fn full_hash(&self) -> usize {
        match self {
            Instruction::SetFocalLength(s, n) => hash(&format!("{}={}", s, n)),
            Instruction::RemoveLens(s) => hash(&format!("{}-", s)),
        }
    }

    fn box_number(&self) -> usize {
        match self {
            Instruction::SetFocalLength(s, _) | Instruction::RemoveLens(s) => hash(s),
        }
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    instructions.iter().map(|i| i.full_hash()).sum()
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut boxes: [Vec<(&str, u8)>; 256] = std::array::from_fn(|_| Vec::new());

    instructions.iter().for_each(|i| {
        let box_number = i.box_number();
        match i {
            Instruction::SetFocalLength(lens_identifier, focal_length) => {
                let mut replaced = false;
                boxes[box_number] = boxes[box_number]
                    .iter()
                    .map(|(s, n)| {
                        if s == &lens_identifier {
                            replaced = true;
                            (*s, *focal_length)
                        } else {
                            (*s, *n)
                        }
                    })
                    .collect::<Vec<_>>();
                if !replaced {
                    boxes[box_number].push((lens_identifier, *focal_length));
                }
            }
            Instruction::RemoveLens(s) => {
                let box_number = hash(s);
                let lens_identifier = s.as_str();
                boxes[box_number].retain(|(s, _)| s != &lens_identifier);
            }
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1, b))
        .map(|(i, b)| {
            b.iter()
                .filter(|(s, _)| !s.is_empty())
                .enumerate()
                .map(|(j, e)| (j + 1, e))
                .map(|(l_pos, &(_, focal_length))| l_pos * focal_length as usize)
                .sum::<usize>()
                * i
        })
        .sum()
}

fn main() -> Result<()> {
    let instructions = read_comma_separated("input/day15.txt")?;
    let result = part1(&instructions);
    println!("part 1: {}", result);
    let result = part2(&instructions);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let instructions = read_comma_separated("input/test/day15.txt")?;
    let result = part1(&instructions);
    assert_eq!(result, 1320);
    let result = part2(&instructions);
    assert_eq!(result, 145);
    Ok(())
}
