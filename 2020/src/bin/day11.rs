use std::{cmp::min, fmt::Display, str::FromStr};

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
    changed: bool,
}

enum Mode {
    Neighbor,
    Vision,
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

    // vision is, in order, the cells seen when looking in a particular direction.
    // we're looking for a chair, taken or open.
    fn look<I>(&self, vision: I) -> Option<State>
    where
        I: Iterator<Item = (usize, usize)>,
    {
        vision
            .map(|(i, j)| self.get_state(i, j))
            .find(|s| s == &State::TakenChair || s == &State::OpenChair)
    }

    fn vision_neighbor_count(&self, x: usize, y: usize) -> usize {
        // Each of these represents (inter)cardinal directions as coordinates.
        // For instance, the first one is coordinates you look at when you look
        // up, cell by cell in the seating area
        let up = self.look((0..y).rev().map(|j| (x, j)));
        let up_right = self.look((x + 1..self.width).zip((0..y).rev()));
        let right = self.look((x + 1..self.width).map(|i| (i, y)));
        let down_right = self.look((x + 1..self.width).zip(y + 1..self.height));
        let down = self.look((y + 1..self.height).map(|j| (x, j)));
        let down_left = self.look((0..x).rev().zip(y + 1..self.height));
        let left = self.look((0..x).rev().map(|i| (i, y)));
        let up_left = self.look((0..x).rev().zip((0..y).rev()));
        [
            up, up_right, right, down_right, down, down_left, left, up_left,
        ]
        .iter()
        .filter_map(|&a| a)
        .filter(|s| s == &State::TakenChair)
        .count()
    }

    fn tick(&mut self, mode: &Mode) {
        let counter = match mode {
            Mode::Neighbor => Self::neighbor_count,
            Mode::Vision => Self::vision_neighbor_count,
        };
        let mut changed = false;
        let new_state = (0..self.width)
            .cartesian_product(0..self.height)
            .sorted_by_key(|(x, _)| *x)
            .sorted_by_key(|(_, y)| *y)
            .map(|(x, y)| {
                let old_state = self.get_state(x, y);
                let new_state = match (counter(&self, x, y), old_state, mode) {
                    (0, State::OpenChair, _) => State::TakenChair,
                    (5..=8, State::TakenChair, _) => State::OpenChair,
                    (4, State::TakenChair, Mode::Neighbor) => State::OpenChair,
                    (_, state, _) => state,
                };
                changed = changed || new_state != old_state;
                new_state
            })
            .collect();
        self.changed = changed;
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
            changed: false,
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

fn run(mut seating_area: SeatingArea, mode: &Mode, visualize: bool) -> usize {
    loop {
        seating_area.tick(mode);
        if !seating_area.changed {
            break;
        }
        if visualize {
            print!("{}[2J", 27 as char);
            println!("{}", seating_area);
        }
    }
    seating_area.count_sitters()
}

fn part1(seating_area: SeatingArea) -> usize {
    run(seating_area, &Mode::Neighbor, false)
}

fn part2(seating_area: SeatingArea) -> usize {
    run(seating_area, &Mode::Vision, false)
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
    fn test_neighbor_tick() -> Result<()> {
        let mut seating: SeatingArea = read_file("input/test/day11_1.1.txt")?;
        seating.tick(&Mode::Neighbor);
        let expected: SeatingArea = read_file("input/test/day11_1.2.txt")?;
        assert_eq!(seating.state, expected.state);
        seating.tick(&Mode::Neighbor);
        let expected: SeatingArea = read_file("input/test/day11_1.3.txt")?;
        assert_eq!(seating.state, expected.state);

        Ok(())
    }

    #[test]
    fn test_vision_tick() -> Result<()> {
        let mut seating: SeatingArea = read_file("input/test/day11_2.1.txt")?;
        seating.tick(&Mode::Vision);
        let expected: SeatingArea = read_file("input/test/day11_2.2.txt")?;
        assert_eq!(seating.state, expected.state);
        seating.tick(&Mode::Vision);
        let expected: SeatingArea = read_file("input/test/day11_2.3.txt")?;
        assert_eq!(seating.state, expected.state);

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
