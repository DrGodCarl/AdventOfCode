use std::collections::HashSet;

use anyhow::{bail, Result};
use itertools::Itertools;
use utils::read_file;

enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl Direction {
    fn from_coord(&self, (q, r): &(isize, isize)) -> (isize, isize) {
        match self {
            Direction::East => (q + 1, *r),
            Direction::Southeast => (*q, r + 1),
            Direction::Southwest => (q - 1, r + 1),
            Direction::West => (q - 1, *r),
            Direction::Northwest => (*q, r - 1),
            Direction::Northeast => (q + 1, r - 1),
        }
    }

    fn values() -> Vec<Self> {
        vec![
            Direction::East,
            Direction::Southeast,
            Direction::Southwest,
            Direction::West,
            Direction::Northwest,
            Direction::Northeast,
        ]
    }
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
    dirs.iter()
        .fold((0, 0), |coord, dir| dir.from_coord(&coord))
}

fn make_initial_state(directions: &[Vec<Direction>]) -> HashSet<(isize, isize)> {
    directions.iter().fold(HashSet::new(), |mut acc, dirs| {
        let coord = calculate_coords(dirs);
        if !acc.insert(coord) {
            acc.remove(&coord);
        }
        acc
    })
}

fn find_neighbors(point: &(isize, isize)) -> Vec<(isize, isize)> {
    Direction::values()
        .iter()
        .map(|d| d.from_coord(point))
        .collect()
}

fn part1(directions: &[Vec<Direction>]) -> usize {
    make_initial_state(directions).len()
}

fn part2(directions: &[Vec<Direction>]) -> usize {
    let mut state = make_initial_state(directions);
    for _ in 0..100 {
        let to_check: Vec<_> = state.iter().flat_map(find_neighbors).unique().collect();
        state = to_check
            .iter()
            .filter(|p| {
                let is_black = state.contains(p);
                let neighbors = find_neighbors(p);
                let black_neighbors = neighbors.iter().filter(|n| state.contains(n)).count();
                (is_black && black_neighbors == 1) || black_neighbors == 2
            })
            .copied()
            .collect();
    }
    state.len()
}

fn main() -> Result<()> {
    let input: String = read_file("input/day24.txt")?;
    let dirs = input
        .split_whitespace()
        .map(parse)
        .collect::<Result<Vec<_>, _>>()?;

    let result = part1(&dirs);
    println!("part 1: {}", result);

    let result = part2(&dirs);
    println!("part 2: {}", result);
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

    #[test]
    fn test_part2() -> Result<()> {
        let input: String = read_file("input/test/day24.txt")?;
        let dirs = input
            .split_whitespace()
            .map(parse)
            .collect::<Result<Vec<_>, _>>()?;
        let result = part2(&dirs);
        assert_eq!(result, 2208);
        Ok(())
    }
}
