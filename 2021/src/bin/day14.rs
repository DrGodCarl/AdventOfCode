use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use parse_display::FromStr;
use utils::read_file;

type Polymer = Vec<char>;
type PolymerizationRules = HashMap<(char, char), char>;

#[derive(FromStr)]
#[display("{0} -> {1}")]
struct PolymerizationRule(String, char);

struct Input {
    polymer: Polymer,
    rules: PolymerizationRules,
}

fn run_polymerization(steps: i16, input: &Input) -> usize {
    let mut pair_counts = input
        .polymer
        .iter()
        .tuple_windows()
        .map(|(a, b)| (*a, *b))
        .counts();
    for _ in 0..steps {
        let mut new_counts = HashMap::new();
        for (&k, &v) in input.rules.iter() {
            let current_count = *pair_counts.get(&k).unwrap_or(&0);
            if current_count == 0 {
                continue;
            }
            let (f, s) = k;
            *new_counts.entry((f, v)).or_insert(0) += current_count;
            *new_counts.entry((v, s)).or_insert(0) += current_count;
        }
        pair_counts = new_counts;
    }
    let mut char_counts =
        pair_counts
            .iter()
            .fold(HashMap::new(), |mut counts, ((c1, c2), count)| {
                *counts.entry(c1).or_insert(0) += count;
                *counts.entry(c2).or_insert(0) += count;
                counts
            });
    // Everything but the first and last characters have been double counted.
    let first = input.polymer.first().unwrap();
    let last = input.polymer.last().unwrap();
    // So count those each once more
    *char_counts.entry(first).or_insert(0) += 1;
    *char_counts.entry(last).or_insert(0) += 1;
    // and halve them.
    let minmax = char_counts.values().map(|v| v / 2).minmax();
    match minmax {
        itertools::MinMaxResult::MinMax(min, max) => max - min,
        _ => 0,
    }
}

fn part1(input: &Input) -> usize {
    run_polymerization(10, input)
}

fn part2(input: &Input) -> usize {
    run_polymerization(40, input)
}

fn read_input(path: &str) -> Result<Input> {
    let text: String = read_file(path)?;
    let parts: Vec<_> = text.split("\n\n").collect();
    let polymer = parts[0].chars().collect();
    let rules = parts[1]
        .split('\n')
        .map(|s| {
            s.parse::<PolymerizationRule>().map(|r| {
                let mut chars = r.0.chars();
                ((chars.next().unwrap(), chars.next().unwrap()), r.1)
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(Input { polymer, rules })
}

fn main() -> Result<()> {
    let input = read_input("input/day14.txt")?;
    let result = part1(&input);
    println!("part 1: {}", result);
    let result = part2(&input);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let input = read_input("input/test/day14.txt")?;
    let result = part1(&input);
    assert_eq!(result, 1588);
    let result = part2(&input);
    assert_eq!(result, 2188189693529);

    Ok(())
}
