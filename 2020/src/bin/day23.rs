#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use utils::{read_file, InputParseError};

struct Cups {
    cups: Vec<u8>,
    current_cup: u8,
    picked_cups: Vec<u8>,
    max: u8,
}

impl FromStr for Cups {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| c.to_string().parse())
            .collect::<Result<_, _>>()
            .map_err(|_| InputParseError)
            .map(Cups::new)
    }
}

impl Cups {
    fn new(cups: Vec<u8>) -> Self {
        let current_cup = cups[0];
        let max = *cups.iter().max().unwrap();
        Cups {
            cups,
            current_cup,
            picked_cups: Vec::new(),
            max,
        }
    }

    fn index_of_cup(&self, cup: u8) -> usize {
        let (index, _) = self
            .cups
            .iter()
            .enumerate()
            .find(|(_, &c)| c == cup)
            .unwrap();
        index
    }

    fn index_of_current_cup(&self) -> usize {
        self.index_of_cup(self.current_cup)
    }

    fn pick_up(&mut self) {
        let index = self.index_of_current_cup();
        let indices_to_pick = (index + 1..=index + 3)
            .map(|i| i % self.cups.len())
            .collect::<Vec<_>>();
        let (cups, picked) = (index..index + self.cups.len())
            .map(|i| i % self.cups.len())
            .map(|i| (i, self.cups[i]))
            .fold((Vec::new(), Vec::new()), |(mut leave, mut pick), (i, c)| {
                if indices_to_pick.contains(&i) {
                    pick.push(c);
                } else {
                    leave.push(c);
                }
                (leave, pick)
            });
        self.cups = cups;
        self.picked_cups = picked;
    }

    fn choose_destination(&self) -> u8 {
        let minus_one = |c: u8| -> u8 {
            match c - 1 {
                0 => self.max,
                a => a,
            }
        };
        let mut target = minus_one(self.current_cup);
        while !self.cups.contains(&target) {
            target = minus_one(target);
        }
        target
    }

    fn put_down(&mut self) {
        let destination = self.choose_destination();
        let index = self.index_of_cup(destination) + 1;
        let picked = self.picked_cups.clone();
        self.picked_cups = Vec::new();
        self.cups.splice(index..index, picked);
    }

    fn adjust_current_cup(&mut self) {
        let index = (self.index_of_current_cup() + 1) % self.cups.len();
        self.current_cup = self.cups[index];
    }

    fn cups_from_cup(&self, cup: u8) -> Vec<u8> {
        let index = self.index_of_cup(cup);
        (index..index + self.cups.len())
            .map(|i| i % self.cups.len())
            .map(|i| self.cups[i])
            .collect()
    }

    fn tick(&mut self) {
        self.pick_up();
        self.put_down();
        self.adjust_current_cup();
    }
}

fn part1(mut cups: Cups) -> String {
    for _ in 0..100 {
        cups.tick();
    }
    cups.cups_from_cup(1)
        .iter()
        .skip(1)
        .map(|&i| i.to_string())
        .join("")
}

fn part2(mut cups: Cups) -> String {
    String::default()
}

fn main() -> Result<()> {
    let cups = "318946572".parse()?;
    let result = part1(cups);
    println!("part 1: {}", result);

    let cups = "318946572".parse()?;
    let result = part2(cups);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let cups = "389125467".parse()?;
        let result = part1(cups);
        assert_eq!(result, "67384529");
        Ok(())
    }

    // #[test]
    // fn test_part2() -> Result<()> {
    // let cups = "389125467".parse()?;
    // let result = part1(cups);
    // assert_eq!(result, "67384529");
    // Ok(())
    // }
}
