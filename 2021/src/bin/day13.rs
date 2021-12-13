use std::collections::HashSet;

use anyhow::Result;
use utils::read_file;

#[derive(parse_display::FromStr, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[display("{0},{1}")]
struct Point(u16, u16);

#[derive(parse_display::FromStr, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Instruction {
    #[display("fold along x={0}")]
    FoldX(u16),
    #[display("fold along y={0}")]
    FoldY(u16),
}

enum DoInstructions {
    Some(usize),
    All,
}

struct FoldingPaper {
    marked_points: HashSet<Point>,
    instructions: Vec<Instruction>,
    max_bounds: Point,
}

fn calculate_max_bounds(points: &HashSet<Point>) -> Point {
    points
        .iter()
        .fold(Point(0, 0), |mb, p| Point(p.0.max(mb.0), p.1.max(mb.1)))
}

impl FoldingPaper {
    fn new(marked_points: HashSet<Point>, instructions: Vec<Instruction>) -> Self {
        let max_bounds = calculate_max_bounds(&marked_points);
        Self {
            marked_points,
            instructions,
            max_bounds,
        }
    }

    fn count_marks(&self) -> usize {
        self.marked_points.len()
    }

    fn do_instructions(&mut self, how_many: &DoInstructions) {
        let number = match how_many {
            DoInstructions::Some(n) => *n,
            DoInstructions::All => self.instructions.len(),
        };
        for i in 0..number {
            let instr = self.instructions[i];
            self.do_instruction(&instr)
        }
    }

    fn do_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::FoldX(x) => self.fold_x(*x),
            Instruction::FoldY(y) => self.fold_y(*y),
        }
    }

    fn fold_x(&mut self, x_axis: u16) {
        let top = self.marked_points.iter().filter(|p| p.0 < x_axis).copied();
        let reflection = |x: u16| 2 * x_axis - x;
        let bottom = self
            .marked_points
            .iter()
            .filter(|p| p.0 > x_axis)
            .map(|Point(x, y)| Point(reflection(*x), *y));
        self.marked_points = top.chain(bottom).collect();
        self.max_bounds = calculate_max_bounds(&self.marked_points);
    }

    fn fold_y(&mut self, y_axis: u16) {
        let top = self.marked_points.iter().filter(|p| p.1 < y_axis).copied();
        let reflection = |y: u16| 2 * y_axis - y;
        let bottom = self
            .marked_points
            .iter()
            .filter(|p| p.1 > y_axis)
            .map(|Point(x, y)| Point(*x, reflection(*y)));
        self.marked_points = top.chain(bottom).collect();
        self.max_bounds = calculate_max_bounds(&self.marked_points);
    }
}

fn part1(paper: &mut FoldingPaper) -> usize {
    paper.do_instructions(&DoInstructions::Some(1));
    paper.count_marks()
}

fn part2(paper: &mut FoldingPaper) {
    paper.do_instructions(&DoInstructions::All);
    for y in 0..=paper.max_bounds.1 {
        for x in 0..=paper.max_bounds.0 {
            if paper.marked_points.contains(&Point(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn read_input(path: &str) -> Result<FoldingPaper> {
    let text: String = read_file(path)?;
    let parts: Vec<_> = text.split("\n\n").collect();
    let marked_points = parts[0]
        .split('\n')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let instructions = parts[1]
        .split('\n')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    Ok(FoldingPaper::new(marked_points, instructions))
}

fn main() -> Result<()> {
    let mut paper = read_input("input/day13.txt")?;
    let result = part1(&mut paper);
    println!("part 1: {}", result);
    let mut paper = read_input("input/day13.txt")?;
    part2(&mut paper);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let mut paper = read_input("input/test/day13.txt")?;
    let result = part1(&mut paper);
    assert_eq!(result, 17);

    Ok(())
}
