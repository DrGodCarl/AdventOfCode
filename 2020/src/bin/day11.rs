use std::{cmp::min, fmt::Display, iter, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use parse_display::{self, ParseError};
use utils::read_file;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
enum State {
    #[display("L")]
    OpenChair,

    #[display("#")]
    TakenChair,

    #[display(".")]
    Floor,
}

#[derive(PartialEq, Debug, Clone)]
struct SeatingArea {
    state: Vec<State>,
    width: usize,
    height: usize,
}

impl SeatingArea {
    fn get_state(&self, x: usize, y: usize) -> State {
        self.state[x + y * self.width]
    }

    fn count_sitters(&self) -> usize {
        self.state
            .iter()
            .filter(|&s| s == &State::TakenChair)
            .count()
    }

    fn neighbor_count(&self, x: usize, y: usize) -> usize {
        let x_min = x.checked_sub(1).unwrap_or(0);
        let x_max = min(x + 1, self.width - 1);
        let y_min = y.checked_sub(1).unwrap_or(0);
        let y_max = min(y + 1, self.height - 1);
        (x_min..=x_max)
            .cartesian_product(y_min..=y_max)
            .filter(|coords| coords != &(x, y))
            .map(|(i, j)| self.get_state(i, j))
            .filter(|s| s == &State::TakenChair)
            .count()
    }

    fn tick(&mut self) {
        let new_state = (0..self.width)
            .cartesian_product(0..self.height)
            .sorted_by_key(|(x, _)| *x)
            .sorted_by_key(|(_, y)| *y)
            .map(
                |(x, y)| match (self.neighbor_count(x, y), self.get_state(x, y)) {
                    (0, State::OpenChair) => State::TakenChair,
                    (4..=8, State::TakenChair) => State::OpenChair,
                    (_, state) => state,
                },
            )
            .collect();
        self.state = new_state;
    }

    fn tick2(&mut self) {
        let new_state = (0..self.width)
            .cartesian_product(0..self.height)
            .sorted_by_key(|(x, _)| *x)
            .sorted_by_key(|(_, y)| *y)
            .map(
                |(x, y)| match (self.neighbor_count(x, y), self.get_state(x, y)) {
                    (0, State::OpenChair) => State::TakenChair,
                    (4..=8, State::TakenChair) => State::OpenChair,
                    (_, state) => state,
                },
            )
            .collect();
        self.state = new_state;
    }
}

impl FromStr for SeatingArea {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split_whitespace().collect();
        let height = lines.len();
        let width = lines[0].len();
        let state = lines
            .iter()
            .flat_map(|&l| l.chars())
            .map(|c| c.to_string())
            .map(|c| c.parse::<State>().unwrap())
            .collect();
        Ok(SeatingArea {
            state,
            width,
            height,
        })
    }
}

impl Display for SeatingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .state
            .chunks(self.width)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|s| match s {
                        State::OpenChair => "L",
                        State::TakenChair => "#",
                        State::Floor => ".",
                    })
                    .join("")
            })
            .join("\n");
        write!(f, "{}", out)
    }
}

fn part1(mut seating_area: SeatingArea) -> usize {
    let mut prev_state = seating_area.state.clone();
    loop {
        seating_area.tick();
        if seating_area.state == prev_state {
            break;
        }
        prev_state = seating_area.state.clone();
    }
    seating_area.count_sitters()
}

fn part2(mut seating_area: SeatingArea) -> usize {
    let mut prev_state = seating_area.state.clone();
    loop {
        seating_area.tick2();
        if seating_area.state == prev_state {
            break;
        }
        prev_state = seating_area.state.clone();
    }
    seating_area.count_sitters()
}

fn main() -> Result<()> {
    let seating_area = read_file("input/day11.txt")?;
    let result = part1(seating_area);
    println!("part 1: {}", result);

    let seating_area = read_file("input/day11.txt")?;
    let result = part2(seating_area);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick() -> Result<()> {
        let mut seating: SeatingArea = read_file("input/test/day11_1.1.txt")?;
        seating.tick();
        let expected: SeatingArea = read_file("input/test/day11_1.2.txt")?;
        assert_eq!(seating, expected);
        seating.tick();
        let expected: SeatingArea = read_file("input/test/day11_1.3.txt")?;
        assert_eq!(seating, expected);

        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let seating: SeatingArea = read_file("input/test/day11_1.1.txt")?;
        let result = part1(seating);
        assert_eq!(result, 37);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let seating: SeatingArea = read_file("input/test/day11_1.1.txt")?;
        let result = part2(seating);
        assert_eq!(result, 26);
        Ok(())
    }
}
