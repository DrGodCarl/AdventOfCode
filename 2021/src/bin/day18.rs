#![feature(let_else)]

use anyhow::Result;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::{complete::take, streaming::tag},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};
use utils::{read_lines, InputParseError};

#[derive(PartialEq, Debug, Clone)]
enum DumbNumber {
    Number(u8),
    Pair(Box<DumbNumber>, Box<DumbNumber>),
}

impl std::str::FromStr for DumbNumber {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_pair(s).map(|(_, d)| d).map_err(|_| InputParseError)
    }
}

impl DumbNumber {
    fn add(self, other: DumbNumber) -> DumbNumber {
        let mut result = DumbNumber::Pair(Box::new(self), Box::new(other));
        result.reduce();
        result
    }

    fn leftmost(&mut self) -> &mut u8 {
        match self {
            DumbNumber::Number(n) => n,
            DumbNumber::Pair(left, _) => left.leftmost(),
        }
    }

    fn rightmost(&mut self) -> &mut u8 {
        match self {
            DumbNumber::Number(n) => n,
            DumbNumber::Pair(_, right) => right.rightmost(),
        }
    }

    fn reduce(&mut self) {
        loop {
            if !self.do_explode(0, None, None) && !self.do_split() {
                break;
            }
        }
    }

    fn do_explode(
        &mut self,
        depth: usize,
        to_the_left: Option<&mut u8>,
        to_the_right: Option<&mut u8>,
    ) -> bool {
        if depth < 4 {
            let DumbNumber::Pair(left, right)= self else {
                return false;
            };
            return left.do_explode(depth + 1, to_the_left, Some(right.leftmost()))
                || right.do_explode(depth + 1, Some(left.rightmost()), to_the_right);
        } else {
            match self {
                DumbNumber::Number(_) => false,
                DumbNumber::Pair(left, right) => {
                    if let Some(l) = to_the_left {
                        *l += *left.leftmost()
                    }
                    if let Some(r) = to_the_right {
                        *r += *right.rightmost()
                    }
                    *self = DumbNumber::Number(0);
                    true
                }
            }
        }
    }

    fn do_split(&mut self) -> bool {
        match self {
            DumbNumber::Pair(left, right) => left.do_split() || right.do_split(),
            DumbNumber::Number(n) => match n {
                0..=9 => false,
                _ => {
                    *self = DumbNumber::Pair(
                        Box::from(DumbNumber::Number(*n / 2)),
                        Box::from(DumbNumber::Number((*n + 1) / 2)),
                    );
                    true
                }
            },
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            DumbNumber::Number(n) => *n as u64,
            DumbNumber::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

fn parse_actual_number(s: &str) -> IResult<&str, DumbNumber> {
    map_res(take(1usize), |i: &str| i.parse())(s).map(|(s, n)| (s, DumbNumber::Number(n)))
}

fn dumb_number_element(s: &str) -> IResult<&str, DumbNumber> {
    alt((parse_actual_number, parse_pair))(s)
}

fn parse_pair(s: &str) -> IResult<&str, DumbNumber> {
    let (s, (left, right)) = delimited(
        tag("["),
        separated_pair(dumb_number_element, tag(","), dumb_number_element),
        tag("]"),
    )(s)?;
    Ok((s, DumbNumber::Pair(Box::new(left), Box::new(right))))
}

fn part1(numbers: Vec<DumbNumber>) -> u64 {
    let mut current_num = None;
    for num in numbers.into_iter() {
        current_num = match current_num {
            None => Some(num),
            Some(n) => Some(n.add(num)),
        }
    }
    current_num.unwrap().magnitude()
}

fn part2(numbers: Vec<DumbNumber>) -> u64 {
    numbers
        .iter()
        .cartesian_product(numbers.iter())
        .map(|(n1, n2)| n1.clone().add(n2.clone()).magnitude())
        .max()
        .unwrap()
}

fn main() -> Result<()> {
    let numbers = read_lines("input/day18.txt")?;
    let result = part1(numbers);
    println!("part 1: {}", result);

    let numbers = read_lines("input/day18.txt")?;
    let result = part2(numbers);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let numbers = read_lines("input/test/day18.txt")?;
    let result = part1(numbers);
    assert_eq!(result, 4140);

    let numbers = read_lines("input/test/day18.txt")?;
    let result = part2(numbers);
    assert_eq!(result, 3993);

    Ok(())
}
