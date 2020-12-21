use std::{collections::HashMap, str::FromStr};

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
    None,
}

impl Rule {
    fn by_removing_references(&self, references: &[usize]) -> Self {
        match self {
            Rule::Char(c) => Rule::Char(*c),
            Rule::Matches(indices) => {
                let contains = references.iter().any(|r| indices.contains(r));
                if contains {
                    Rule::None
                } else {
                    Rule::Matches(indices.clone())
                }
            }
            Rule::MatchesOneOf(or_indices) => {
                let new_or_indices = or_indices
                    .iter()
                    .filter(|inds| !references.iter().any(|r| inds.contains(r)))
                    .cloned()
                    .collect::<Vec<Vec<usize>>>();
                if new_or_indices.is_empty() {
                    Rule::None
                } else {
                    Rule::MatchesOneOf(new_or_indices)
                }
            }
            Rule::None => Rule::None,
        }
    }
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

struct RegexGenerator {
    max_depth: usize,
    incestuous_depth_counter: HashMap<usize, usize>,
}

impl RegexGenerator {
    fn new(incestuous_rules: Vec<usize>, max_depth: usize) -> Self {
        RegexGenerator {
            max_depth,
            incestuous_depth_counter: incestuous_rules.iter().map(|i| (*i, 0usize)).collect(),
        }
    }

    fn default() -> Self {
        RegexGenerator {
            max_depth: 0,
            incestuous_depth_counter: HashMap::new(),
        }
    }

    fn create_regex(&mut self, rules: &RuleSet, start: usize) -> Regex {
        Regex::new(format!("^{}$", self.create_regex_string(rules, &start)).as_str()).unwrap()
    }

    fn create_regex_string(&mut self, rules: &RuleSet, start: &usize) -> String {
        let rule = &rules[start];

        if let Some(c) = self.incestuous_depth_counter.get(start) {
            if c > &self.max_depth {
                self.incestuous_depth_counter.insert(*start, 0);
                let references = self
                    .incestuous_depth_counter
                    .keys()
                    .cloned()
                    .collect::<Vec<_>>();
                let adjusted_rule = rule.by_removing_references(&references);
                return self.create_regex_string_for_rule(rules, &adjusted_rule);
            }
            *self.incestuous_depth_counter.entry(*start).or_insert(0) += 1;
        }

        self.create_regex_string_for_rule(rules, rule)
    }

    fn create_regex_string_for_rule(&mut self, rules: &RuleSet, rule: &Rule) -> String {
        match rule {
            Rule::Char(c) => c.to_string(),
            Rule::Matches(indices) => indices
                .iter()
                .map(|i| self.create_regex_string(rules, i))
                .join(""),
            Rule::MatchesOneOf(or_indices) => {
                let ors = or_indices
                    .iter()
                    .map(|i| {
                        i.iter()
                            .map(|i| self.create_regex_string(rules, i))
                            .join("")
                    })
                    .join("|");
                format!("({})", ors)
            }
            Rule::None => "".to_string(),
        }
    }
}

fn part1(rules: &RuleSet, messages: &[String]) -> usize {
    let mut generator = RegexGenerator::default();
    let regex = generator.create_regex(rules, 0);
    messages.iter().filter(|m| regex.is_match(m)).count()
}

fn part2(rules: &mut RuleSet, messages: &[String]) -> usize {
    let mut generator = RegexGenerator::new(vec![8, 11], 3);
    rules.insert(8, Rule::MatchesOneOf(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::MatchesOneOf(vec![vec![42, 31], vec![42, 11, 31]]));
    let regex = generator.create_regex(rules, 0);
    messages.iter().filter(|m| regex.is_match(m)).count()
}

fn main() -> Result<()> {
    let input = read_file("input/day19.txt")?;
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
