use std::{collections::HashMap, hash::Hash, str::FromStr};

use anyhow::Result;
use parse_display::FromStr;
use utils::read_lines;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, FromStr)]
enum Card {
    #[display("2")]
    Two,
    #[display("3")]
    Three,
    #[display("4")]
    Four,
    #[display("5")]
    Five,
    #[display("6")]
    Six,
    #[display("7")]
    Seven,
    #[display("8")]
    Eight,
    #[display("9")]
    Nine,
    #[display("T")]
    Ten,
    #[display("J")]
    Jack,
    #[display("Q")]
    Queen,
    #[display("K")]
    King,
    #[display("A")]
    Ace,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, FromStr)]
enum FunCard {
    #[display("J")]
    Joker,
    #[display("2")]
    Two,
    #[display("3")]
    Three,
    #[display("4")]
    Four,
    #[display("5")]
    Five,
    #[display("6")]
    Six,
    #[display("7")]
    Seven,
    #[display("8")]
    Eight,
    #[display("9")]
    Nine,
    #[display("T")]
    Ten,
    #[display("Q")]
    Queen,
    #[display("K")]
    King,
    #[display("A")]
    Ace,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, FromStr)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, FromStr)]
#[from_str(regex = "(?<one>.)(?<two>.)(?<three>.)(?<four>.)(?<five>.)")]
struct Hand<CT: Copy + FromStr + Ord = Card> {
    one: CT,
    two: CT,
    three: CT,
    four: CT,
    five: CT,
}

trait TypeableHand: Ord + Clone {
    fn hand_type(&self) -> HandType;
}

impl<CT: Copy + FromStr + Ord> Hand<CT> {
    fn cards(&self) -> Vec<CT> {
        vec![self.one, self.two, self.three, self.four, self.five]
    }
}

impl<T: Copy + FromStr + Ord + Hash> Hand<T> {
    fn get_card_counts(&self, exclude: Option<T>) -> Vec<usize>
    where
        T: Ord + Clone,
    {
        let mut card_counts = self
            .cards()
            .iter()
            .fold(HashMap::new(), |mut acc, &card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            })
            .iter()
            .filter(|(&card, _)| exclude.map(|e| e != card).unwrap_or(true))
            .map(|(_, &count)| count)
            .collect::<Vec<_>>();
        card_counts.sort();
        card_counts.reverse();
        card_counts.truncate(5);
        card_counts
    }
}

fn match_card_counts(card_counts: &[usize]) -> HandType {
    match card_counts {
        [5] => HandType::FiveOfAKind,
        [4, ..] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, ..] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

impl TypeableHand for Hand<Card> {
    fn hand_type(&self) -> HandType {
        let card_counts = self.get_card_counts(None);
        match_card_counts(&card_counts)
    }
}

impl TypeableHand for Hand<FunCard> {
    fn hand_type(&self) -> HandType {
        let mut card_counts = self.get_card_counts(Some(FunCard::Joker));
        let joker_count = 5 - card_counts.iter().sum::<usize>();
        let first_count = card_counts.get_mut(0);
        if let Some(first_count) = first_count {
            *first_count += joker_count;
        } else {
            card_counts.push(joker_count);
        }
        match_card_counts(&card_counts)
    }
}

#[test]
fn test_hand_types() {
    let hand = "AAAAA".parse::<Hand>().unwrap();
    assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
    let hand = "AA8AA".parse::<Hand>().unwrap();
    assert_eq!(hand.hand_type(), HandType::FourOfAKind);
    let hand = "23332".parse::<Hand>().unwrap();
    assert_eq!(hand.hand_type(), HandType::FullHouse);
    let hand = "TTT98".parse::<Hand>().unwrap();
    assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
    let hand = "23432".parse::<Hand>().unwrap();
    assert_eq!(hand.hand_type(), HandType::TwoPair);
    let hand = "A23A4".parse::<Hand>().unwrap();
    assert_eq!(hand.hand_type(), HandType::OnePair);
    let hand = "23456".parse::<Hand>().unwrap();
    assert_eq!(hand.hand_type(), HandType::HighCard);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, FromStr)]
#[display("{hand} {bet}")]
struct Bet<T: TypeableHand = Hand<Card>> {
    hand: T,
    bet: u32,
}

fn run<HT: TypeableHand>(bets: &[Bet<HT>]) -> u32 {
    let mut bets = bets.to_vec();
    bets.sort_by(|a, b| b.hand.cmp(&a.hand));
    bets.sort_by_key(|bet| bet.hand.hand_type());
    bets.iter()
        .rev()
        .enumerate()
        .map(|(i, Bet { bet, .. })| bet * (i as u32 + 1))
        .sum()
}

fn part1(bets: &[Bet]) -> u32 {
    run(bets)
}

fn part2(bets: &[Bet<Hand<FunCard>>]) -> u32 {
    run(bets)
}

fn main() -> Result<()> {
    let bets = read_lines("input/day07.txt")?;
    let result = part1(&bets);
    println!("part 1: {}", result);
    let bets: Vec<Bet<Hand<FunCard>>> = read_lines("input/day07.txt")?;
    let result = part2(&bets);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let bets: Vec<Bet> = read_lines("input/test/day07.txt")?;
    let result = part1(&bets);
    assert_eq!(result, 6440);
    let bets: Vec<Bet<Hand<FunCard>>> = read_lines("input/test/day07.txt")?;
    let result = part2(&bets);
    assert_eq!(result, 5905);
    Ok(())
}
