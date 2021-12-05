use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{self, bail};
use anyhow::{Context, Result};
use utils::{read_file, InputParseError};

struct Board {
    called: HashSet<usize>,
    layout: HashMap<(usize, usize), usize>,
}

enum Direction {
    Column,
    Row,
}

impl Board {
    fn play_number(&mut self, number: usize) {
        self.called.insert(number);
    }

    fn has_bingo_in(&self, direction: Direction) -> bool {
        let point_maker = match direction {
            Direction::Column => |a: usize, b| (a, b),
            Direction::Row => |a, b| (b, a),
        };
        (0..=4).any(|x| {
            (0..=4).all(|y| {
                self.layout
                    .get(&point_maker(x, y))
                    .map(|n| self.called.contains(n))
                    .unwrap_or(false)
            })
        })
    }

    fn has_bingo(&self) -> bool {
        self.has_bingo_in(Direction::Row) || self.has_bingo_in(Direction::Column)
    }

    fn unmarked_sum(&self) -> usize {
        self.layout
            .iter()
            .map(|(_, &n)| n)
            .filter(|n| !self.called.contains(n))
            .sum()
    }
}

impl FromStr for Board {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let layout_vec: Vec<Vec<usize>> = s
            .split('\n')
            .map(|s| {
                s.split(' ')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()
            .map_err(|_| InputParseError)?;
        let layout = layout_vec
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &num)| ((x, y), num)))
            .collect();
        Ok(Board {
            called: HashSet::new(),
            layout,
        })
    }
}

fn part1(numbers: &[usize], boards: &mut [Board]) -> Result<usize> {
    for &number in numbers {
        for board in boards.iter_mut() {
            board.play_number(number);
            if board.has_bingo() {
                return Ok(board.unmarked_sum() * number);
            }
        }
    }
    bail!("No winning game");
}

fn part2(numbers: &[usize], boards: &mut [Board]) -> Result<usize> {
    let length = boards.len();
    let mut done_boards = HashSet::new();
    for &called_number in numbers {
        for (board_num, board) in boards.iter_mut().enumerate() {
            board.play_number(called_number);
            if board.has_bingo() && !done_boards.contains(&board_num) {
                done_boards.insert(board_num);
                if done_boards.len() == length {
                    return Ok(board.unmarked_sum() * called_number);
                }
            }
        }
    }
    bail!("No winning game");
}

fn read_input(path: &str) -> Result<(Vec<usize>, Vec<Board>)> {
    let file_contents = read_file::<String>(path)?;
    let strings: Vec<_> = file_contents.split("\n\n").collect();
    let numbers: Vec<usize> = strings
        .get(0)
        .with_context(|| "No data found in file")?
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()?;
    let board = strings
        .iter()
        .skip(1)
        .map(|s| s.parse::<Board>())
        .collect::<Result<_, _>>()?;
    Ok((numbers, board))
}

fn main() -> Result<()> {
    let (numbers, mut boards) = read_input("input/day04.txt")?;
    let result = part1(&numbers, &mut boards)?;
    println!("part 1: {}", result);
    let result = part2(&numbers, &mut boards)?;
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let (numbers, mut boards) = read_input("input/test/day04.txt")?;
    let result = part1(&numbers, &mut boards)?;
    assert_eq!(result, 4512);

    // despite mutating boards, the previous part doesn't impact part 2
    let result = part2(&numbers, &mut boards)?;
    assert_eq!(result, 1924);

    Ok(())
}
