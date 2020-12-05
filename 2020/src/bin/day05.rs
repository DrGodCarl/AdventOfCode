use anyhow::{bail, Context, Result};
use itertools::{Itertools, MinMaxResult};
use utils::read_lines;

fn to_seat_id(partition_instr: &str) -> usize {
    let bin_str = partition_instr
        .replace(&['F', 'L'][..], "0")
        .replace(&['B', 'R'][..], "1");
    usize::from_str_radix(bin_str.as_str(), 2).unwrap()
}

fn part1(partition_instrs: &Vec<String>) -> usize {
    partition_instrs
        .iter()
        .map(|s| to_seat_id(s.as_str()))
        .max()
        .unwrap_or(0)
}

fn part2(partition_instrs: &Vec<String>) -> Result<usize> {
    enum FoldState {
        Init,
        Looking(usize),
        Done(usize),
    }
    let search_result = partition_instrs
        .iter()
        .map(|s| to_seat_id(s.as_str()))
        .sorted()
        .fold(FoldState::Init, |state, id| match state {
            FoldState::Init => FoldState::Looking(id),
            FoldState::Looking(prev) if prev == id - 1 => FoldState::Looking(id),
            FoldState::Looking(_) => FoldState::Done(id - 1),
            res => res,
        });
    match search_result {
        FoldState::Done(res) => Ok(res),
        _ => bail!("something went wrong"),
    }
}

fn part2_alt(partition_instrs: &Vec<String>) -> Result<usize> {
    let seat_ids: Vec<usize> = partition_instrs
        .iter()
        .map(|s| to_seat_id(s.as_str()))
        .collect();
    let sum_of_seats: usize = seat_ids.iter().sum();
    let (min, max) = match seat_ids.iter().minmax() {
        MinMaxResult::MinMax(a, b) => (a, b),
        _ => bail!("not enough elements"),
    };
    Ok(max * (max + 1) / 2 - (min - 1) * min / 2 - sum_of_seats)
}

fn main() -> Result<()> {
    let partition_instructions = read_lines("input/day05.txt")?;
    let result = part1(&partition_instructions);
    println!("part 1: {}", result);
    let result = part2(&partition_instructions).unwrap();
    println!("part 2: {}", result);
    let result = part2_alt(&partition_instructions).unwrap();
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_seat_id() -> Result<()> {
        let seat = "FBFBBFFRLR";
        let seat_id = to_seat_id(seat);
        assert_eq!(seat_id, 357);
        let seat = "FFFFFFFLLL";
        let seat_id = to_seat_id(seat);
        assert_eq!(seat_id, 0);
        let seat = "BBBBBBBRRR";
        let seat_id = to_seat_id(seat);
        assert_eq!(seat_id, 1023);
        Ok(())
    }
}
