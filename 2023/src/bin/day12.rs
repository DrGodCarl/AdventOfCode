use std::{iter, marker::PhantomData, str::FromStr, vec};

use anyhow::Result;
use parse_display::FromStr;
use utils::{read_lines, InputParseError};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, FromStr)]
enum Status {
    #[display("#")]
    Damaged,
    #[display(".")]
    Operational,
    #[display("?")]
    Unknown,
}

impl Status {
    fn matches(&self, other: Status) -> bool {
        match (self, other) {
            (Status::Damaged, Status::Damaged) => true,
            (Status::Operational, Status::Operational) => true,
            (Status::Unknown, _) => true,
            (_, Status::Unknown) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Folded;
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Unfolded;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SpringRecord<T: Clone> {
    state: Vec<Status>,
    groups: Vec<u64>,
    _marker: std::marker::PhantomData<T>,
}

impl FromStr for SpringRecord<Folded> {
    type Err = InputParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let state = parts[0]
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let groups = parts[1]
            .split(',')
            .map(|g| g.parse().unwrap())
            .collect::<Vec<u64>>();
        Ok(SpringRecord::<Folded> {
            state,
            groups,
            _marker: PhantomData,
        })
    }
}

impl FromStr for SpringRecord<Unfolded> {
    type Err = InputParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let state = format!(
            "{}{}{}{}{}{}{}{}{}",
            parts[0], "?", parts[0], "?", parts[0], "?", parts[0], "?", parts[0]
        )
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
        let groups = format!(
            "{}{}{}{}{}{}{}{}{}",
            parts[1], ",", parts[1], ",", parts[1], ",", parts[1], ",", parts[1]
        )
        .split(',')
        .map(|g| g.parse().unwrap())
        .collect::<Vec<u64>>();
        Ok(SpringRecord::<Unfolded> {
            state,
            groups,
            _marker: PhantomData,
        })
    }
}

// the sum of the gaps must be less than max
fn increment_gaps(gaps: &mut Vec<u64>, max: u64) -> bool {
    // println!("incrementing gaps: {:?}, {:?}", gaps, max);
    let mut new_gaps = vec![];
    while let Some(gap) = gaps.pop() {
        if gaps.iter().sum::<u64>() + new_gaps.iter().sum::<u64>() + gap < max {
            new_gaps.push(gap + 1);
            new_gaps.extend(gaps.iter().rev());
            break;
        } else {
            new_gaps.push(1);
            if gaps.is_empty() {
                return false;
            }
        }
        // println!("new_gaps: {:?}", new_gaps);
    }
    new_gaps.reverse();
    *gaps = new_gaps;
    true
}

fn generate_state(groups: &[u64], gaps: &[u64], length: u64) -> Vec<Status> {
    let mut state = Vec::new();
    let mut current = 0;
    for (gap, group) in gaps.iter().zip(groups.iter()) {
        state.extend(iter::repeat(Status::Operational).take(*gap as usize));
        state.extend(iter::repeat(Status::Damaged).take(*group as usize));
        current += *gap + *group;
    }
    // println!("current: {}", current);
    // println!("length: {}", length);
    state.extend(iter::repeat(Status::Operational).take((length - current) as usize));
    state
}

fn generate_possible_states<T: Clone>(record: &SpringRecord<T>) -> Vec<Vec<Status>> {
    // println!("generating possible states for {:?}", record);
    let gap_count = record.groups.len() - 1;
    let mut gaps = (0..gap_count)
        .map(|_| 1)
        .chain(iter::once(0)) // The "gap" between nothing and the first group
        .rev()
        .collect::<Vec<u64>>();
    let length = record.state.len() as u64;
    let groups_sum = record.groups.iter().sum::<u64>();
    let max_gap_total = length - groups_sum as u64;
    iter::once((true, gaps.clone()))
        .chain(iter::repeat_with(|| {
            let keep_going = increment_gaps(&mut gaps, max_gap_total);
            (keep_going, gaps.clone())
        }))
        .take_while(|&(keep_going, _)| keep_going)
        .map(|(_, gaps)| gaps)
        .map(|gaps| generate_state(&record.groups, &gaps, length))
        .filter(|state| {
            state
                .iter()
                .zip(record.state.iter())
                .all(|(&a, &b)| a.matches(b))
        })
        .collect()
}

fn part1(records: &[SpringRecord<Folded>]) -> usize {
    records
        .iter()
        .map(generate_possible_states)
        .map(|s| s.len())
        .sum()
}

fn part2(records: &[SpringRecord<Unfolded>]) -> usize {
    records
        .iter()
        .map(generate_possible_states)
        .map(|s| s.len())
        .sum()
}

fn main() -> Result<()> {
    let records = read_lines("input/day12.txt")?;
    let result = part1(&records);
    println!("part 1: {}", result);

    let records = read_lines("input/day12.txt")?;
    let result = part2(&records);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    // let record: SpringRecord = "???.### 1,1,3".parse()?;
    // let result = generate_possible_states(&record);
    // assert_eq!(result.len(), 1);

    // let record: SpringRecord = ".??..??...?##. 1,1,3".parse()?;
    // let result = generate_possible_states(&record);
    // assert_eq!(result.len(), 4);

    // let record: SpringRecord = "?#?#?#?#?#?#?#? 1,3,1,6".parse()?;
    // let result = generate_possible_states(&record);
    // assert_eq!(result.len(), 1);

    // let record: SpringRecord = "????.#...#... 4,1,1".parse()?;
    // let result = generate_possible_states(&record);
    // assert_eq!(result.len(), 1);

    // let record: SpringRecord = "????.######..#####. 1,6,5".parse()?;
    // let result = generate_possible_states(&record);
    // assert_eq!(result.len(), 4);

    // let record: SpringRecord = "?###???????? 3,2,1".parse()?;
    // let result = generate_possible_states(&record);
    // assert_eq!(result.len(), 10);
    let records: Vec<SpringRecord<Folded>> = read_lines("input/test/day12.txt")?;
    let result = part1(&records);
    assert_eq!(result, 21);

    let records: Vec<SpringRecord<Unfolded>> = read_lines("input/test/day12.txt")?;
    let result = part2(&records);
    assert_eq!(result, 525152);
    Ok(())
}
