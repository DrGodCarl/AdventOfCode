use std::{convert::TryFrom, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    self, bits,
    bits::complete::{tag, take},
    branch::alt,
    combinator::{map, map_res},
    multi::{length_count, many_till},
    IResult,
};
use utils::{read_file, InputParseError};

#[derive(PartialEq, Debug)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operation {
        version: u8,
        operator: Operator,
        packets: Vec<Packet>,
    },
}

impl FromStr for Packet {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_from_str(s).map_err(|_| InputParseError)
    }
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Operation {
                version: _,
                operator,
                packets,
            } => operator.exec(packets),
        }
    }
}

type TheBits<'a> = (&'a [u8], usize);

#[derive(PartialEq, Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn exec(&self, packets: &[Packet]) -> u64 {
        match self {
            Operator::Sum => packets.iter().map(|p| p.value()).sum(),
            Operator::Product => packets.iter().map(|p| p.value()).product(),
            Operator::Minimum => packets.iter().map(|p| p.value()).min().unwrap(),
            Operator::Maximum => packets.iter().map(|p| p.value()).max().unwrap(),
            Operator::GreaterThan => (packets[0].value() > packets[1].value()) as u64,
            Operator::LessThan => (packets[0].value() < packets[1].value()) as u64,
            Operator::EqualTo => (packets[0].value() == packets[1].value()) as u64,
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = InputParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Operator::Sum),
            1 => Ok(Operator::Product),
            2 => Ok(Operator::Minimum),
            3 => Ok(Operator::Maximum),
            5 => Ok(Operator::GreaterThan),
            6 => Ok(Operator::LessThan),
            7 => Ok(Operator::EqualTo),
            _ => Err(InputParseError),
        }
    }
}

fn bits_from_str(input: &str) -> Result<Vec<u8>> {
    input
        .chars()
        .chunks(2)
        .into_iter()
        .map(|two_chars| two_chars.into_iter().join(""))
        .map(|cs| u8::from_str_radix(&cs, 16))
        .map(|r| r.map_err(|_| anyhow!("Couldn't parse hex")))
        .collect::<Result<Vec<u8>>>()
}

fn parse_from_str(input: &str) -> Result<Packet> {
    let input_bits = bits_from_str(input)?;
    parse_packet(&input_bits)
}

fn version(i: TheBits) -> IResult<TheBits, u8> {
    take(3usize)(i)
}

fn operator(i: TheBits) -> IResult<TheBits, Operator> {
    #[allow(clippy::redundant_closure)] // using for type hint - map_res is a generic beast
    map_res(take(3usize), |o: u8| Operator::try_from(o))(i)
}

fn leading_literal_chunk(i: TheBits) -> IResult<TheBits, u8> {
    let (i, _) = tag(1usize, 1usize)(i)?;
    take(4usize)(i)
}

fn ending_literal_chunk(i: TheBits) -> IResult<TheBits, u8> {
    let (i, _) = tag(0usize, 1usize)(i)?;
    take(4usize)(i)
}

fn literal_chunk(i: TheBits) -> IResult<TheBits, Vec<u8>> {
    let (i, (mut leading, ending)) = many_till(leading_literal_chunk, ending_literal_chunk)(i)?;
    leading.push(ending);
    Ok((i, leading))
}

fn literal_value(i: TheBits) -> IResult<TheBits, u64> {
    let (i, chunks) = literal_chunk(i)?;
    let mut out = 0u64;
    for c in chunks {
        out <<= 4;
        out |= c as u64;
    }
    Ok((i, out))
}

fn literal_packet(i: TheBits) -> IResult<TheBits, Packet> {
    let (i, version) = version(i)?;
    let (i, _) = tag(4, 3usize)(i)?;
    let (i, value) = literal_value(i)?;
    Ok((i, Packet::Literal { version, value }))
}

fn sub_packets_by_length(i: TheBits) -> IResult<TheBits, Vec<Packet>> {
    let (i, _) = tag(0, 1usize)(i)?;

    // Ideally the following would work, but the bits don't jive with length_value
    //
    // length_value(
    //     take(15usize),
    //     fold_many1(packet, Vec::new, |mut acc: Vec<_>, item| {
    //         acc.push(item);
    //         acc
    //     }),
    // )(i)

    // So instead we'll do this...
    let (i, bit_count): (_, u16) = take(15usize)(i)?;

    let mut current_length = 0u16;
    let mut packets = vec![];
    let mut rem_i = i;

    while current_length < bit_count {
        // println!("read {:?} of {:?}", current_length, bit_count);
        let (new_i, packet) = packet(rem_i)?;
        current_length += calculate_consumed(rem_i, new_i);
        rem_i = new_i;
        packets.push(packet);
    }

    Ok((rem_i, packets))
}

fn sub_packets_by_count(i: TheBits) -> IResult<TheBits, Vec<Packet>> {
    let (i, _) = tag(1, 1usize)(i)?;
    length_count(map(take(11usize), |n: u16| n), packet)(i)
}

fn operation_sub_packets(i: TheBits) -> IResult<TheBits, Vec<Packet>> {
    alt((sub_packets_by_length, sub_packets_by_count))(i)
}

fn operation_packet(i: TheBits) -> IResult<TheBits, Packet> {
    let (i, version) = version(i)?;
    let (i, type_id) = operator(i)?;
    let (i, packets) = operation_sub_packets(i)?;
    Ok((
        i,
        Packet::Operation {
            version,
            operator: type_id,
            packets,
        },
    ))
}

fn packet(i: TheBits) -> IResult<TheBits, Packet> {
    alt((literal_packet, operation_packet))(i)
}

fn parse_packet(input: &[u8]) -> Result<Packet> {
    let packet: IResult<&[u8], Packet> = bits(packet)(input);

    match packet {
        Ok((_, p)) => Ok(p),
        Err(_) => Err(anyhow!("No good. Packet couldn't be made.")),
    }
}

fn calculate_consumed(old: TheBits, new: TheBits) -> u16 {
    let (i_old, idx_old) = old;
    let (i_new, idx_new) = new;

    ((i_old.len() * 8 - idx_old) - (i_new.len() * 8 - idx_new)) as u16
}

fn sum_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, value: _ } => *version as u64,
        Packet::Operation {
            version,
            operator: _,
            packets,
        } => *version as u64 + packets.iter().map(|p| sum_versions(p)).sum::<u64>(),
    }
}

fn part1(packet: &Packet) -> u64 {
    sum_versions(packet)
}

fn part2(packet: &Packet) -> u64 {
    packet.value()
}

fn main() -> Result<()> {
    let input = read_file("input/day16.txt")?;
    let result = part1(&input);
    println!("part 1: {}", result);
    let result = part2(&input);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = "8A004A801A8002F478".parse()?;
        let result = part1(&input);
        assert_eq!(result, 16);

        let input = "620080001611562C8802118E34".parse()?;
        let result = part1(&input);
        assert_eq!(result, 12);

        let input = "C0015000016115A2E0802F182340".parse()?;
        let result = part1(&input);
        assert_eq!(result, 23);

        let input = "A0016C880162017C3686B18A3D4780".parse()?;
        let result = part1(&input);
        assert_eq!(result, 31);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = "C200B40A82".parse()?;
        let result = part2(&input);
        assert_eq!(result, 3);

        let input = "04005AC33890".parse()?;
        let result = part2(&input);
        assert_eq!(result, 54);

        let input = "880086C3E88112".parse()?;
        let result = part2(&input);
        assert_eq!(result, 7);

        let input = "CE00C43D881120".parse()?;
        let result = part2(&input);
        assert_eq!(result, 9);

        let input = "D8005AC2A8F0".parse()?;
        let result = part2(&input);
        assert_eq!(result, 1);

        let input = "F600BC2D8F".parse()?;
        let result = part2(&input);
        assert_eq!(result, 0);

        let input = "9C005AC2F8F0".parse()?;
        let result = part2(&input);
        assert_eq!(result, 0);

        let input = "9C0141080250320F1802104A08".parse()?;
        let result = part2(&input);
        assert_eq!(result, 1);

        Ok(())
    }

    #[test]
    fn test_parse_literal() -> Result<()> {
        let input = "D2FE28";
        let packet = parse_from_str(input)?;
        assert_eq!(
            packet,
            Packet::Literal {
                version: 6,
                value: 2021
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_length_operation() -> Result<()> {
        let input = "38006F45291200";
        let packet = parse_from_str(input)?;
        assert_eq!(
            packet,
            Packet::Operation {
                version: 1,
                operator: Operator::LessThan,
                packets: vec![
                    Packet::Literal {
                        version: 6,
                        value: 10
                    },
                    Packet::Literal {
                        version: 2,
                        value: 20
                    },
                ]
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_count_operation() -> Result<()> {
        let input = "EE00D40C823060";
        let packet = parse_from_str(input)?;
        assert_eq!(
            packet,
            Packet::Operation {
                version: 7,
                operator: Operator::Maximum,
                packets: vec![
                    Packet::Literal {
                        version: 2,
                        value: 1
                    },
                    Packet::Literal {
                        version: 4,
                        value: 2
                    },
                    Packet::Literal {
                        version: 1,
                        value: 3
                    },
                ]
            }
        );
        Ok(())
    }
}
