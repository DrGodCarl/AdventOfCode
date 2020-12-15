use std::collections::HashMap;

use anyhow::Result;
use utils::read_file;

enum History {
    First(u32),
    PreviousTwo(u32, u32),
}

fn speak(previous: u32, history: &HashMap<u32, History>) -> u32 {
    let num_hist = history.get(&previous);
    match num_hist.unwrap_or(&History::First(0)) {
        History::First(_) => 0,
        History::PreviousTwo(recent, older) => recent - older,
    }
}

fn run(numbers: &[u32], until_turn: u32) -> u32 {
    let history: HashMap<u32, History> = numbers
        .iter()
        .enumerate()
        .map(|(turn, num)| (*num, History::First((turn + 1) as u32)))
        .collect();
    let previous = numbers.last().unwrap();
    ((numbers.len() + 1) as u32..=until_turn)
        .fold((*previous, history), |(previous, mut history), turn| {
            let spoken = speak(previous, &history);
            let num_hist = match history.get(&spoken) {
                Some(History::First(prev)) => History::PreviousTwo(turn, *prev),
                Some(History::PreviousTwo(recent, _)) => History::PreviousTwo(turn, *recent),
                None => History::First(turn),
            };
            history.insert(spoken, num_hist);
            (spoken, history)
        })
        .0
}

fn part1(numbers: &[u32]) -> u32 {
    run(numbers, 2020)
}

// This is very slow unless run in release mode.
fn part2(numbers: &[u32]) -> u32 {
    run(numbers, 30_000_000)
}

fn main() -> Result<()> {
    let numbers = read_file::<String>("input/day15.txt")?
        .split(',')
        .map(|i| i.parse())
        .collect::<Result<Vec<_>, _>>()?;
    let result = part1(&numbers);
    println!("part 1: {}", result);

    let result = part2(&numbers);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() -> Result<()> {
        let numbers = vec![1, 3, 2];
        let result = part1(&numbers);
        assert_eq!(result, 1);

        let numbers = vec![2, 1, 3];
        let result = part1(&numbers);
        assert_eq!(result, 10);

        let numbers = vec![1, 2, 3];
        let result = part1(&numbers);
        assert_eq!(result, 27);

        let numbers = vec![3, 1, 2];
        let result = part1(&numbers);
        assert_eq!(result, 1836);

        Ok(())
    }
}
