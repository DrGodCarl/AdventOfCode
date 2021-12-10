use anyhow::Result;
use itertools::Itertools;
use utils::read_lines;

enum SyntaxError {
    Incomplete(Vec<char>),
    Corrupt(char),
}

fn expected_closing_brace(opening_brace: &char) -> Option<char> {
    match opening_brace {
        '{' => Some('}'),
        '(' => Some(')'),
        '<' => Some('>'),
        '[' => Some(']'),
        _ => None,
    }
}

fn score_corrupt_character(brace: &char) -> usize {
    match brace {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_autocomplete_character(brace: &char) -> usize {
    match brace {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn score_autocompletion(completion_stack: &[char]) -> usize {
    completion_stack
        .iter()
        .rev()
        .map(score_autocomplete_character)
        .fold(0, |score, s| score * 5 + s)
}

fn parse_program(line: &str) -> SyntaxError {
    let mut expected = vec![];
    for c in line.chars() {
        if let Some(closing) = expected_closing_brace(&c) {
            expected.push(closing);
        } else if let Some(closing) = expected.pop() {
            if closing != c {
                return SyntaxError::Corrupt(c);
            }
        } else {
            return SyntaxError::Corrupt(c);
        }
    }
    SyntaxError::Incomplete(expected)
}

fn part1(program: &[String]) -> usize {
    program
        .iter()
        .map(|l| parse_program(l))
        .filter_map(|e| match e {
            SyntaxError::Corrupt(c) => Some(c),
            _ => None,
        })
        .map(|c| score_corrupt_character(&c))
        .sum()
}

fn part2(program: &[String]) -> usize {
    let result_set: Vec<usize> = program
        .iter()
        .map(|l| parse_program(l))
        .filter_map(|e| match e {
            SyntaxError::Incomplete(cs) => Some(cs),
            _ => None,
        })
        .map(|cs| score_autocompletion(&cs))
        .sorted()
        .collect();
    result_set[result_set.len() / 2]
}

fn main() -> Result<()> {
    let program = read_lines("input/day10.txt")?;
    let result = part1(&program);
    println!("part 1: {}", result);
    let result = part2(&program);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let program = read_lines("input/test/day10.txt")?;
    let result = part1(&program);
    assert_eq!(result, 26397);

    let result = part2(&program);
    assert_eq!(result, 288957);

    Ok(())
}
