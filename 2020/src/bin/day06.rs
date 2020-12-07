use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;
use utils::read_file;

fn break_up_answers(answer_text: &String) -> Vec<Vec<&str>> {
    answer_text
        .split("\n\n")
        .map(|ans_block| ans_block.split_whitespace().collect())
        .collect()
}

fn count_unique_answers(answers: &Vec<&str>) -> usize {
    answers.iter().flat_map(|&l| l.chars()).unique().count()
}

fn count_common_answers(answers: &Vec<&str>) -> usize {
    answers
        .iter()
        .map(|&l| l.chars().collect::<HashSet<_>>())
        .fold(
            (b'a'..=b'z').map(char::from).collect::<HashSet<_>>(),
            |acc, ans| acc.intersection(&ans).copied().collect(),
        )
        .len()
}

fn part1(all_answers: &Vec<Vec<&str>>) -> usize {
    all_answers.iter().map(|a| count_unique_answers(a)).sum()
}

fn part2(all_answers: &Vec<Vec<&str>>) -> usize {
    all_answers.iter().map(|a| count_common_answers(a)).sum()
}

fn main() -> Result<()> {
    let answer_text: String = read_file("input/day06.txt")?;
    let answers = break_up_answers(&answer_text);
    let result = part1(&answers);
    println!("part 1: {}", result);
    let result = part2(&answers);
    println!("part 2: {}", result);
    Ok(())
}