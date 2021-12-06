use anyhow::Result;
use utils::read_comma_separated;

fn number_of_fish_after_days(initial_pop: &[usize], days: usize) -> usize {
    let mut population = [0_usize; 9];
    initial_pop.iter().for_each(|&n| population[n] += 1);

    (0..days).for_each(|_| {
        population.rotate_left(1);
        population[6] += population[8];
    });

    population.iter().sum()
}

fn part1(numbers: &[usize]) -> usize {
    number_of_fish_after_days(numbers, 80)
}

fn part2(numbers: &[usize]) -> usize {
    number_of_fish_after_days(numbers, 256)
}

fn main() -> Result<()> {
    let numbers = read_comma_separated("input/day06.txt")?;
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
    fn test() -> Result<()> {
        let numbers = read_comma_separated("input/test/day06.txt")?;
        let result = number_of_fish_after_days(&numbers, 18);
        assert_eq!(result, 26);
        let result = part1(&numbers);
        assert_eq!(result, 5934);
        let result = part2(&numbers);
        assert_eq!(result, 26984457539);

        Ok(())
    }
}
