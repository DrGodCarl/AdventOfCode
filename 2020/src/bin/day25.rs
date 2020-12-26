fn perform(value: usize, subject: usize, divisor: usize) -> usize {
    (value * subject) % divisor
}

fn perform_times(subject: usize, divisor: usize, times: usize) -> usize {
    (0..times).fold(1, |acc, _| perform(acc, subject, divisor))
}

fn determine_loop_size(
    subject: usize,
    target: usize,
    mut value: usize,
    divisor: usize,
    mut iteration: usize,
) -> usize {
    while value != target {
        iteration += 1;
        value = perform(value, subject, divisor);
    }
    iteration
}

fn part1(card: usize, door: usize) -> usize {
    let divisor = 20201227;
    let card_loop_size = determine_loop_size(7, card, 1, divisor, 0);
    println!("card loop: {}", card_loop_size);
    let door_loop_size = determine_loop_size(7, door, 1, divisor, 0);
    println!("door loop: {}", door_loop_size);
    let door_public_key = perform_times(7, divisor, door_loop_size);
    perform_times(door_public_key, 20201227, card_loop_size)
}

fn main() {
    let (card, door) = (5099500, 7648211);
    let result = part1(door, card);
    println!("part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (card, door) = (5764801, 17807724);
        let result = part1(card, door);
        assert_eq!(result, 14897079);
    }
}
