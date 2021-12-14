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
    let mut polymer = input.polymer.clone();
    for _ in 0..steps {
        let size = polymer.len();
        polymer = polymer
            .iter()
            .tuple_windows()
            .enumerate()
            .flat_map(|(c, (a, b))| {
                let middle = input.rules.get(&(*a, *b));
                let mut result = vec![*a];
                if let Option::Some(m) = middle {
                    result.push(*m);
                }
                if c == size - 2 {
                    result.push(*b);
                }
                result
            })
            .collect();
    }
    let counts = polymer.iter().counts();
    let minmax = counts.values().minmax();
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
