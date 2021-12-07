use anyhow::Result;
use utils::read_comma_separated;

fn part1(numbers: &mut [isize]) -> isize {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    let target = numbers[mid] as isize;
    numbers.iter().map(|&n| (n - target).abs()).sum()
}

fn part2(numbers: &[isize]) -> isize {
    let target = numbers.iter().sum::<isize>() / numbers.len() as isize;
    // try either side of the mean - it's not /perfectly/ the mean, but it's close.
    vec![target, target + 1]
        .iter()
        .map(|t| {
            numbers
                .iter()
                .map(|&n| {
                    let d = (n - t).abs();
                    d * (d + 1) / 2
                })
                .sum::<isize>()
        })
        .min()
        .unwrap()
}

fn main() -> Result<()> {
    let mut numbers = read_comma_separated("input/day07.txt")?;
    let result = part1(&mut numbers);
    println!("part 1: {}", result);
    let result = part2(&numbers);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let mut numbers = read_comma_separated("input/test/day07.txt")?;
    let result = part1(&mut numbers);
    assert_eq!(result, 37);

    let result = part2(&numbers);
    assert_eq!(result, 168);

    Ok(())
}
