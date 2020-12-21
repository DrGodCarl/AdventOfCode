use std::{collections::HashMap, iter, str::FromStr, string::ParseError};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use utils::{read_file, InputParseError};

type RuleSet = HashMap<usize, Rule>;

#[derive(Debug)]
enum Rule {
    Char(char),
    Matches(Vec<usize>),
    MatchesOneOf(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('"') {
            return s
                .trim()
                .chars()
                .nth(1)
                .ok_or(InputParseError)
                .map(Rule::Char);
        }
        if s.contains('|') {
            return s
                .split('|')
                .map(|opt| {
                    opt.split_whitespace()
                        .map(|s| s.parse::<usize>())
                        .collect::<Result<Vec<usize>, _>>()
                })
                .collect::<Result<Vec<Vec<usize>>, _>>()
                .map_err(|_| InputParseError)
                .map(Rule::MatchesOneOf);
        }
        s.split_whitespace()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| InputParseError)
            .map(Rule::Matches)
    }
}

#[derive(Debug)]
struct RulesAndMessages(RuleSet, Vec<String>);

fn index_and_rule(s: &str) -> Result<(usize, Rule), InputParseError> {
    let index_and_rule: Vec<_> = s.split(':').collect();
    if index_and_rule.len() != 2 {
        return Err(InputParseError);
    }
    index_and_rule[0]
        .trim()
        .parse()
        .ok()
        .zip(index_and_rule[1].trim().parse().ok())
        .ok_or(InputParseError)
}

impl FromStr for RulesAndMessages {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs: Vec<&str> = s.split("\n\n").collect();
        if inputs.len() != 2 {
            return Err(InputParseError);
        }
        let rules = inputs[0]
            .split('\n')
            .map(index_and_rule)
            .collect::<Result<HashMap<usize, Rule>, _>>()?;
        let messages = inputs[1].split_whitespace().map(str::to_string).collect();
        Ok(RulesAndMessages(rules, messages))
    }
}

fn create_regex(rules: &RuleSet, start: usize) -> Regex {
    Regex::new(format!("^{}$", create_regex_string(rules, &start)).as_str()).unwrap()
}

fn create_regex_string(rules: &RuleSet, start: &usize) -> String {
    let rule = &rules[start];

    match rule {
        Rule::Char(c) => c.to_string(),
        Rule::Matches(indices) => indices
            .iter()
            .map(|i| create_regex_string(rules, i))
            .join(""),
        Rule::MatchesOneOf(inner_rules) => {
            let ors = inner_rules
                .iter()
                .map(|i| i.iter().map(|i| create_regex_string(rules, i)).join(""))
                .join("|");
            format!("({})", ors)
        }
    }
}

fn part1(rules: &RuleSet, messages: &[String]) -> usize {
    let regex = create_regex(rules, 0);
    println!("{:?}", regex);
    messages.iter().filter(|m| regex.is_match(m)).count()
}

fn part2(rules: &mut RuleSet, messages: &[String]) -> usize {
    rules.insert(8, Rule::MatchesOneOf(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::MatchesOneOf(vec![vec![42, 31], vec![42, 11, 31]]));
    part1(&rules, &messages)
}

fn main() -> Result<()> {
    let input = read_file("input/day19.txt")?;
    println!("{:?}", input);
    let RulesAndMessages(mut rules, messages) = input;
    let result = part1(&rules, &messages);
    println!("part 1: {}", result);
    let result = part2(&mut rules, &messages);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = read_file("input/test/day19_1.txt")?;
        let RulesAndMessages(rules, messages) = input;
        let result = part1(&rules, &messages);
        assert_eq!(result, 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = read_file("input/test/day19_2.txt")?;
        let RulesAndMessages(mut rules, messages) = input;
        let result = part2(&mut rules, &messages);
        assert_eq!(result, 12);
        Ok(())
    }
}
