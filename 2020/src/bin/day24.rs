use std::collections::HashMap;

use anyhow::{bail, Result};
use utils::read_file;

enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

fn parse(s: &str) -> Result<Vec<Direction>> {
    let mut res = Vec::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        let d = match c {
            'e' => Direction::East,
            'w' => Direction::West,
            'n' => match chars.next() {
                Some('e') => Direction::Northeast,
                Some('w') => Direction::Northwest,
                _ => bail!("Started going north and then something went way wrong."),
            },
            's' => match chars.next() {
                Some('e') => Direction::Southeast,
                Some('w') => Direction::Southwest,
                _ => bail!("Started going south and then something went way wrong."),
            },
            _ => bail!("Something got really fucked up."),
        };
        res.push(d);
    }
    Ok(res)
}

fn calculate_coords(dirs: &[Direction]) -> (isize, isize) {
    dirs.iter().fold((0, 0), |(q, r), dir| match dir {
        Direction::East => (q + 1, r),
        Direction::Southeast => (q, r + 1),
        Direction::Southwest => (q - 1, r + 1),
        Direction::West => (q - 1, r),
        Direction::Northwest => (q, r - 1),
        Direction::Northeast => (q + 1, r - 1),
    })
}

fn make_initial_state(directions: &[Vec<Direction>]) -> HashMap<(isize, isize), usize> {
    directions.iter().fold(HashMap::new(), |mut acc, dirs| {
        let coord = calculate_coords(dirs);
        *acc.entry(coord).or_insert(0) += 1;
        acc
    })
}

fn part1(directions: &[Vec<Direction>]) -> usize {
    make_initial_state(directions)
        .iter()
        .filter(|(_, &f)| f % 2 == 1)
        .count()
}

fn main() -> Result<()> {
    let input: String = read_file("input/day24.txt")?;
    let dirs = input
        .split_whitespace()
        .map(parse)
        .collect::<Result<Vec<_>, _>>()?;
    let result = part1(&dirs);
    println!("part 1: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input: String = read_file("input/test/day24.txt")?;
        let dirs = input
            .split_whitespace()
            .map(parse)
            .collect::<Result<Vec<_>, _>>()?;
        let result = part1(&dirs);
        assert_eq!(result, 10);
        Ok(())
    }
}
