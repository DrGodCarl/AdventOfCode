use gcd::Gcd;
use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[display("{0},{1}")]
struct Point(isize, isize);

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{start} -> {end}")]
struct Line {
    start: Point,
    end: Point,
}

fn lattice_points(line: &Line) -> HashSet<Point> {
    if line.start == line.end {
        // The line is really a point.
        let mut result = HashSet::new();
        result.insert(line.start);
        return result;
    }
    // Figure out the slope as a reduced fraction (rise/run)
    let run = line.end.0 - line.start.0;
    let rise = line.end.1 - line.start.1;
    let factor = (rise.abs() as usize).gcd(run.abs() as usize) as isize;
    let run = run / factor;
    let rise = rise / factor;
    // Either it's going up or it's going down - chaining together will catch the right one.
    let run_range = (line.start.0..=line.end.0).chain((line.end.0..=line.start.0).rev());
    let rise_range = (line.start.1..=line.end.1).chain((line.end.1..=line.start.1).rev());
    if run == 0 {
        // If the x coord is unchanging, just hit every int between y start and end
        rise_range
            .step_by(rise.abs() as usize)
            .map(|y| Point(line.start.0, y))
            .collect()
    } else if rise == 0 {
        // If the y coord is unchanging, just hit every int between z start and end
        run_range
            .step_by(run.abs() as usize)
            .map(|x| Point(x, line.start.1))
            .collect()
    } else {
        // Otherwise, in the general case, zip up the x range stepped by the run rate
        // and the y range stepped by the rise rate, and those are the lattice points.
        run_range
            .step_by(run.abs() as usize)
            .zip(rise_range.step_by(rise.abs() as usize))
            .map(|(x, y)| Point(x, y))
            .collect()
    }
}

fn count_overlapping_points(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(lattice_points)
        .counts()
        .values()
        .filter(|&&c| c > 1)
        .count()
}

fn part1(lines: &[Line]) -> usize {
    let lines: Vec<Line> = lines
        .iter()
        .filter(|l| l.start.0 == l.end.0 || l.start.1 == l.end.1)
        .copied()
        .collect();

    count_overlapping_points(&lines)
}

fn part2(lines: &[Line]) -> usize {
    count_overlapping_points(lines)
}

fn main() -> Result<()> {
    let numbers = read_lines("input/day05.txt")?;
    let result = part1(&numbers);
    println!("part 1: {}", result);
    let result = part2(&numbers);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() -> Result<()> {
        let numbers = read_lines("input/test/day05.txt")?;
        let result = part1(&numbers);
        assert_eq!(result, 5);

        let result = part2(&numbers);
        assert_eq!(result, 12);

        Ok(())
    }

    #[test]
    fn test_lattice_points() -> Result<()> {
        let line = "0,0 -> 1,1".parse()?;
        let result = lattice_points(&line);
        assert!(result.contains(&Point(0, 0)));
        assert!(result.contains(&Point(1, 1)));
        assert_eq!(result.len(), 2);

        let line = "1,1 -> 3,3".parse()?;
        let result = lattice_points(&line);
        assert!(result.contains(&Point(1, 1)));
        assert!(result.contains(&Point(2, 2)));
        assert!(result.contains(&Point(3, 3)));
        assert_eq!(result.len(), 3);

        let line = "0,0 -> 2,4".parse()?;
        let result = lattice_points(&line);
        assert!(result.contains(&Point(0, 0)));
        assert!(result.contains(&Point(1, 2)));
        assert!(result.contains(&Point(2, 4)));
        assert_eq!(result.len(), 3);

        let line = "2,4 -> 0,0".parse()?;
        let result = lattice_points(&line);
        assert!(result.contains(&Point(0, 0)));
        assert!(result.contains(&Point(1, 2)));
        assert!(result.contains(&Point(2, 4)));
        assert_eq!(result.len(), 3);

        let line = "1,1 -> 1,4".parse()?;
        let result = lattice_points(&line);
        assert!(result.contains(&Point(1, 1)));
        assert!(result.contains(&Point(1, 2)));
        assert!(result.contains(&Point(1, 3)));
        assert!(result.contains(&Point(1, 4)));
        assert_eq!(result.len(), 4);

        let line = "1,1 -> 4,1".parse()?;
        let result = lattice_points(&line);
        assert!(result.contains(&Point(1, 1)));
        assert!(result.contains(&Point(2, 1)));
        assert!(result.contains(&Point(3, 1)));
        assert!(result.contains(&Point(4, 1)));
        assert_eq!(result.len(), 4);

        let line = "1,1 -> 1,1".parse()?;
        let result = lattice_points(&line);
        assert!(result.contains(&Point(1, 1)));
        assert_eq!(result.len(), 1);
        Ok(())
    }
}
