use std::iter;

use anyhow::Result;
use itertools::Itertools;
use utils::read_lines;

// This is a super special case. Each non-3 difference in the input
// is 1, and each chunk-length of 1-differences (e.g. [10, 11, 12]
// is a chunk) has a known number of ways of being arranged. This
// function takes in the length of the number of jumps (e.g. 2 for
// the above) and kicks back how many ways there are to configure
// those. If this needed to be done arbitrarily (rather than just
// to 4), the general form of the Tribonacci numbers could be used.
fn number_of_paths(size: usize) -> usize {
    match size {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        _ => 0, // at least we'll know it's broken?
    }
}

fn part1(adapters: &[usize]) -> usize {
    iter::once(&0)
        .chain(iter::once(&(adapters.iter().max().unwrap() + 3)))
        .chain(adapters.iter())
        .sorted()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .sorted()
        .group_by(|&d| d)
        .into_iter()
        .map(|(_, group)| group.count())
        .product()
}

fn part2(adapters: &[usize]) -> u64 {
    iter::once(&0)
        .chain(iter::once(&(adapters.iter().max().unwrap() + 3)))
        .chain(adapters.iter())
        .sorted()
        .tuple_windows()
        .map(|(a, b)| b - a)
        // Jumps of 3 have exactly one way to be configured
        .group_by(|&d| d < 3)
        .into_iter()
        // so we can split those up
        .filter(|(k, _)| *k)
        // figure out the ways those chunks can be configured
        .map(|(_, g)| g.count())
        .map(|c| number_of_paths(c) as u64)
        // and multiply them
        .product()
}

fn main() -> Result<()> {
    let adapters = read_lines("input/day10.txt")?;
    let result = part1(&adapters);
    println!("part 1: {}", result);

    let result = part2(&adapters);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() -> Result<()> {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = part1(&adapters);
        assert_eq!(result, 7 * 5);

        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result = part1(&adapters);
        assert_eq!(result, 22 * 10);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = part2(&adapters);
        assert_eq!(result, 8);

        let adapters = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result = part2(&adapters);
        assert_eq!(result, 19208);

        Ok(())
    }
}
