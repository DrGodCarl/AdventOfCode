use anyhow::Result;
use parse_display::FromStr;
use utils::read_lines;

#[derive(FromStr, PartialEq, Debug)]
#[display("{0}-{1}")]
struct Range(u32, u32);

impl Range {
    fn contains(&self, value: &Range) -> bool {
        self.0 <= value.0 && value.1 <= self.1
    }

    fn overlaps(&self, value: &Range) -> bool {
        self.0 <= value.0 && value.0 <= self.1
            || self.0 <= value.1 && value.1 <= self.1
            || value.0 <= self.0 && self.0 <= value.1
            || value.0 <= self.1 && self.1 <= value.1
    }
}

#[derive(FromStr, PartialEq, Debug)]
#[display("{0},{1}")]
struct Assignment(Range, Range);

impl Assignment {
    fn one_range_contains_the_other(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn ranges_overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

fn part1(assignments: &[Assignment]) -> usize {
    assignments
        .iter()
        .filter(|a| a.one_range_contains_the_other())
        .count()
}

fn part2(assignments: &[Assignment]) -> usize {
    assignments.iter().filter(|a| a.ranges_overlap()).count()
}

fn main() -> Result<()> {
    let assignments = read_lines("input/day04.txt")?;
    let result = part1(&assignments);
    println!("part 1: {}", result);
    let result = part2(&assignments);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let assignments = read_lines("input/test/day04.txt")?;
    let result = part1(&assignments);
    assert_eq!(result, 2);

    let result = part2(&assignments);
    assert_eq!(result, 4);

    Ok(())
}
