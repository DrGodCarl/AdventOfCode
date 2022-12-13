use std::cmp::Ordering;

use anyhow::Result;
use itertools::{EitherOrBoth, Itertools};
use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::complete::char,
    combinator::map_res,
    combinator::{map, opt},
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Ord, Eq)]
enum Packet {
    Number(u16),
    List(Box<Vec<Packet>>),
}

impl Packet {
    fn listed(&self) -> Packet {
        match self {
            Packet::Number(_) => Packet::List(Box::new(vec![self.clone()])),
            Packet::List(_) => self.clone(),
        }
    }
}

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => a
                .iter()
                .zip_longest(b.iter())
                .map(|zip| match zip {
                    EitherOrBoth::Both(a, b) => a.partial_cmp(b),
                    EitherOrBoth::Left(_) => Some(Ordering::Greater), // Right side ran out
                    EitherOrBoth::Right(_) => Some(Ordering::Less),   // Left side ran out
                })
                .find(|cmp| cmp != &Some(Ordering::Equal))
                .unwrap_or(Some(Ordering::Equal)),
            (Packet::Number(_), Packet::List(_)) => self.listed().partial_cmp(other),
            (Packet::List(_), Packet::Number(_)) => self.partial_cmp(&other.listed()),
        }
    }
}

fn parse_list(s: &str) -> IResult<&str, Vec<Packet>> {
    delimited(char('['), parse_many, char(']'))(s)
}

fn parse_list_packet(s: &str) -> IResult<&str, Packet> {
    map(parse_list, |packets| Packet::List(Box::new(packets)))(s)
}

fn parse_many(s: &str) -> IResult<&str, Vec<Packet>> {
    let (s, first) = opt(parse_packet)(s)?;
    let Some(first) = first else {
        return Ok((s, vec![]));
    };
    let (s, rest) = opt(delimited(char(','), parse_many, opt(char(','))))(s)?;
    let mut packets = vec![first];
    if let Some(mut rest) = rest {
        packets.append(&mut rest);
    }
    Ok((s, packets))
}

fn is_char_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_integer(input: &str) -> IResult<&str, u16> {
    map_res(take_while(is_char_digit), str::parse)(input)
}

fn parse_number_packet(s: &str) -> IResult<&str, Packet> {
    map(parse_integer, Packet::Number)(s)
}

fn parse_packet(s: &str) -> IResult<&str, Packet> {
    alt((parse_list_packet, parse_number_packet))(s)
}

impl std::str::FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_packet(s)
            .map_err(|e| {
                println!("Error parsing packet: {:?}", e);
                anyhow::anyhow!("Failed to parse packet")
            })
            .and_then(|(remaining, parsed)| {
                if !remaining.is_empty() {
                    Err(anyhow::anyhow!(
                        "There's some remaining text: {}",
                        remaining
                    ))
                } else {
                    Ok(parsed)
                }
            })
    }
}

fn read_packets(filename: &str) -> Result<Vec<Packet>> {
    let mut packets = Vec::new();
    for line in std::fs::read_to_string(filename)?.lines() {
        if line.is_empty() {
            continue;
        }
        packets.push(line.parse()?);
    }
    Ok(packets)
}

fn part1(packets: &[Packet]) -> u32 {
    packets
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| 1 + i as u32)
        .sum()
}

fn part2(packets: &[Packet]) -> u32 {
    let dividers: Vec<Packet> = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    packets
        .iter()
        .chain(dividers.iter())
        .sorted()
        .enumerate()
        .filter(|(_, p)| dividers.contains(p))
        .map(|(i, _)| 1 + i as u32)
        .product()
}

fn main() -> Result<()> {
    let packets = read_packets("input/day13.txt")?;
    let result = part1(&packets);
    println!("part 1: {}", result);
    let result = part2(&packets);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let packets = read_packets("input/test/day13.txt")?;
    let result = part1(&packets);
    assert_eq!(result, 13);

    let result = part2(&packets);
    assert_eq!(result, 140);

    Ok(())
}
