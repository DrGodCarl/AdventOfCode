#![feature(iterator_fold_self)]
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use parse_display::{Display, FromStr};
use utils::{parse_comma_separated, read_file, InputParseError};

type TicketInfo = Vec<usize>;

#[derive(Debug, FromStr, Display)]
#[display("{0}-{1}")]
struct NumberRange(usize, usize);

#[derive(Debug, FromStr, Display)]
#[display("{field_name}: {range_1} or {range_2}")]
struct Rule {
    field_name: String,
    range_1: NumberRange,
    range_2: NumberRange,
}

impl Rule {
    fn satisfies_rule(&self, n: &usize) -> bool {
        (n >= &self.range_1.0 && n <= &self.range_1.1)
            || (n >= &self.range_2.0 && n <= &self.range_2.1)
    }
}

struct Info {
    rules: Vec<Rule>,
    my_ticket: TicketInfo,
    nearby_tickets: Vec<TicketInfo>,
}

impl Info {
    fn satisfies_any_rule(&self, n: &usize) -> bool {
        self.rules.iter().any(|r| r.satisfies_rule(n))
    }
}

impl FromStr for Info {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("\n\n").collect();
        if parts.len() != 3 {
            return Err(InputParseError);
        }
        let rules = parts[0]
            .split('\n')
            .map(|s| s.parse::<Rule>())
            .collect::<Result<_, _>>()
            .map_err(|_| InputParseError)?;
        let my_ticket = parts[1]
            .split('\n')
            .nth(1)
            .and_then(|s| parse_comma_separated(s).ok())
            .ok_or(InputParseError)?;
        let nearby_tickets = parts[2]
            .split('\n')
            .skip(1)
            .map(parse_comma_separated)
            .collect::<Result<_, _>>()
            .map_err(|_| InputParseError)?;
        Ok(Info {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }
}

fn invalid_data(info: &Info, ticket_info: &[usize]) -> Vec<usize> {
    ticket_info
        .iter()
        .filter(|&n| !info.satisfies_any_rule(n))
        .copied()
        .collect()
}

fn part1(info: &Info) -> u32 {
    info.nearby_tickets
        .iter()
        .flat_map(|t| invalid_data(info, t))
        .filter(|n| !info.satisfies_any_rule(n))
        .map(|n| n as u32)
        .sum()
}

fn part2(info: &Info) -> u64 {
    let keys: Vec<_> = info.rules.iter().map(|r| r.field_name.clone()).collect();
    let good_tickets: Vec<&TicketInfo> = info
        .nearby_tickets
        .iter()
        .filter(|&t| invalid_data(info, t).is_empty())
        .collect();
    let mut possibilities: HashMap<String, HashSet<usize>> = info
        .rules
        .iter()
        .map(|r| {
            let possible = good_tickets
                .iter()
                .map(|&ticket| {
                    ticket
                        .iter()
                        .enumerate()
                        .filter(|(_, n)| r.satisfies_rule(n))
                        .map(|(idx, _)| idx)
                        .collect::<HashSet<_>>()
                })
                .fold_first(|acc, set| acc.intersection(&set).copied().collect());
            (r.field_name.clone(), possible)
        })
        .map(|(key, val)| (key, val.unwrap_or_default()))
        .collect();
    let mut finalized: HashMap<String, usize> = HashMap::new();
    let key_count = possibilities.len();
    while finalized.len() != key_count {
        let key: String;
        {
            if let Some((k, indices)) = possibilities.iter().find(|(_, v)| v.len() == 1) {
                key = k.clone();
                finalized.insert(key.clone(), *indices.iter().next().unwrap());
            } else {
                break;
            }
        }
        let old = possibilities.remove(&key).unwrap_or_default();
        for k in &keys {
            if let Some(indices) = possibilities.get(k) {
                let new = indices.difference(&old).copied().collect();
                possibilities.insert(k.to_string(), new);
            }
        }
    }
    finalized
        .iter()
        .filter(|(field, _)| field.starts_with("departure"))
        .map(|(_, &index)| info.my_ticket[index] as u64)
        .product()
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
        let input = read_file("input/test/day16.txt")?;
        let result = part1(&input);
        assert_eq!(result, 71);
        Ok(())
    }
}
