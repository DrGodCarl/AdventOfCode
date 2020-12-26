type Cup = usize;

fn parse(s: &str) -> Vec<Cup> {
    s.chars().map(|c| c.to_digit(10).unwrap() as Cup).collect()
}

fn play(cups: &[Cup], times: usize) -> Vec<Cup> {
    let max = *cups.iter().max().unwrap();
    let length = cups.len();
    let mut cup_to_neighbor = vec![0; max + 1];
    (0..length).for_each(|i| cup_to_neighbor[cups[i]] = cups[(i + 1) % length]);

    let minus_one = |c: Cup| -> Cup {
        match c - 1 {
            0 => max,
            a => a,
        }
    };
    let mut current_cup = cups[0];
    for _ in 0..times {
        let take1 = cup_to_neighbor[current_cup];
        let take2 = cup_to_neighbor[take1];
        let take3 = cup_to_neighbor[take2];
        cup_to_neighbor[current_cup] = cup_to_neighbor[take3];

        let mut target = minus_one(current_cup);
        while [take1, take2, take3].contains(&target) {
            target = minus_one(target);
        }

        let temp = cup_to_neighbor[target];
        cup_to_neighbor[target] = take1;
        cup_to_neighbor[take3] = temp;

        current_cup = cup_to_neighbor[current_cup];
    }
    cup_to_neighbor
}

fn part1(cups: &[Cup]) -> String {
    let cup_to_neighbor = play(cups, 100);
    let mut res = String::new();
    let mut cup = 1;
    while cup_to_neighbor[cup] != 1 {
        let next_cup = cup_to_neighbor[cup];
        res.push_str(&next_cup.to_string());
        cup = next_cup;
    }
    res
}

fn part2(initial_cups: &[Cup]) -> usize {
    let cups = initial_cups
        .iter()
        .copied()
        .chain(10usize..=1_000_000)
        .collect::<Vec<_>>();
    let cup_to_neighbor = play(&cups, 10_000_000);
    let after_one = cup_to_neighbor[1];
    let after_that = cup_to_neighbor[after_one];
    after_one * after_that
}

fn main() {
    let cups = parse("318946572");
    let result = part1(&cups);
    println!("part 1: {}", result);

    let result = part2(&cups);
    println!("part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let cups = parse("389125467");
        let result = part1(&cups);
        assert_eq!(result, "67384529");
    }

    #[test]
    fn test_part2() {
        let cups = parse("389125467");
        let result = part2(&cups);
        assert_eq!(result, 149245887792);
    }
}
