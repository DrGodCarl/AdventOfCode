use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Result;
use utils::{read_lines, InputParseError};

#[derive(Debug, Clone)]
struct Card {
    id: u16,
    winning_numbers: HashSet<u16>,
    revealed_numbers: Vec<u16>,
}

impl Card {
    fn score_card(&self) -> u32 {
        let count = self.count_winning_numbers() as u32;
        return if count == 0 { 0 } else { 2u32.pow(count - 1) };
    }

    fn count_winning_numbers(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|n| self.revealed_numbers.contains(n))
            .count()
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            id: 0,
            winning_numbers: HashSet::new(),
            revealed_numbers: Vec::new(),
        }
    }
}

impl FromStr for Card {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(':').flat_map(|s| s.split('|')).enumerate().fold(
            Ok(Card::default()),
            |mut acc, (i, f)| {
                match i {
                    0 => {
                        acc.as_mut().unwrap().id = f
                            .trim()
                            .split_whitespace()
                            .nth(1)
                            .ok_or(InputParseError)
                            .and_then(|s| s.parse::<u16>().map_err(|_| InputParseError))?;
                    }
                    1 => {
                        acc.as_mut().unwrap().winning_numbers = f
                            .trim()
                            .split_whitespace()
                            .map(|s| s.parse::<u16>())
                            .collect::<Result<HashSet<_>, _>>()
                            .map_err(|_| InputParseError)?;
                    }
                    _ => {
                        acc.as_mut().unwrap().revealed_numbers = f
                            .trim()
                            .split_whitespace()
                            .map(|s| s.parse::<u16>())
                            .collect::<Result<Vec<_>, _>>()
                            .map_err(|_| InputParseError)?;
                    }
                }
                acc
            },
        )
    }
}

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(|c| c.score_card()).sum()
}

// This is a brute force solution, that I think can be done with math
// but here we are.
// The math would probably look like:
// - the last card adds zero cards
// - the second to last card adds 1 or 0 cards (figure out which)
// - and so on, so when you get to the first card, you know how many cards it adds
// - and can multiply the number of those added cards by the number of cards each card adds.
fn part2(cards: &[Card]) -> u32 {
    let mut card_count = 0;
    let mut deck = (0..cards.len()).collect::<VecDeque<_>>();
    while let Some(index) = deck.pop_front() {
        card_count += 1;
        let winners = cards[index]
            .count_winning_numbers()
            .min(cards.len() - index - 1); // we don't want to overflow
        (index + 1..index + 1 + winners).for_each(|i| deck.push_back(i));
    }

    card_count
}

fn main() -> Result<()> {
    let cards = read_lines("input/day04.txt")?;
    let result = part1(&cards);
    println!("part 1: {}", result);
    let result = part2(&cards);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let cards: Vec<Card> = read_lines("input/test/day04.txt")?;
    let result = part1(&cards);
    assert_eq!(result, 13);
    let result = part2(&cards);
    assert_eq!(result, 30);
    Ok(())
}
