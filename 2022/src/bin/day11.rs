use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use anyhow::Result;
use itertools::Itertools;
use parse_display::FromStr;
use utils::{read_chunks, InputParseError};

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq)]
enum OperationType {
    #[display("*")]
    Multiply,
    #[display("+")]
    Add,
}

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq)]
enum OperationInput {
    #[display("old")]
    Old,
    #[from_str(regex = "(?P<0>[0-9]+)")]
    Value(u64),
}

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq)]
#[display("Operation: new = {left} {operation} {right}")]
struct Operation {
    left: OperationInput,
    operation: OperationType,
    right: OperationInput,
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        let left = match self.left {
            OperationInput::Old => old,
            OperationInput::Value(v) => v,
        };
        let right = match self.right {
            OperationInput::Old => old,
            OperationInput::Value(v) => v,
        };
        match self.operation {
            OperationType::Multiply => left * right,
            OperationType::Add => left + right,
        }
    }
}

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq)]
#[display("Test: divisible by {divisor}")]
struct Test {
    divisor: u64,
}

#[derive(FromStr, Debug, Clone, Copy, PartialEq, Eq)]
#[display("If {value}: throw to monkey {target_monkey}")]
struct Condition {
    value: bool,
    target_monkey: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkey {
    id: u8,
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    true_condition: Condition,
    false_condition: Condition,
    inspection_count: u64,
}

impl Monkey {
    fn test(&self, item: u64) -> u8 {
        if item % self.test.divisor == 0 {
            self.true_condition.target_monkey
        } else {
            self.false_condition.target_monkey
        }
    }

    fn inspect(&mut self) -> Option<u64> {
        let result = self.items.pop_front();
        if result.is_some() {
            self.inspection_count += 1;
        }
        result
    }
}

impl FromStr for Monkey {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').map(|n| n.trim()).collect();
        let (monkey, items, operation, test, true_condition, false_condition) =
            lines.iter().collect_tuple().ok_or(InputParseError)?;
        let id = monkey
            .split(' ')
            .last()
            .and_then(|s| s.strip_suffix(':')?.parse().ok())
            .ok_or(InputParseError)?;
        let items = items
            .split(": ")
            .skip(1)
            .flat_map(|s| s.split(", "))
            .map(|s| s.parse::<u64>())
            .collect::<Result<_, _>>()
            .map_err(|_| InputParseError)?;
        let operation = operation.parse().map_err(|_| InputParseError)?;
        let test = test.parse().map_err(|_| InputParseError)?;
        let true_condition = true_condition.parse().map_err(|_| InputParseError)?;
        let false_condition = false_condition.parse().map_err(|_| InputParseError)?;
        Ok(Monkey {
            id,
            items,
            operation,
            test,
            true_condition,
            false_condition,
            inspection_count: 0,
        })
    }
}

fn run(monkeys: &mut [Monkey], iterations: u16, reducer: &dyn Fn(u64) -> u64) -> u64 {
    let mut to_push = monkeys
        .iter()
        .map(|m| (m.id, VecDeque::<u64>::new()))
        .collect::<HashMap<_, _>>();
    for _ in 0..iterations {
        for monkey in monkeys.iter_mut() {
            monkey.items.append(to_push.get_mut(&monkey.id).unwrap());
            while let Some(item) = monkey.inspect() {
                let new_worry_level = reducer(monkey.operation.apply(item));
                let target_monkey = monkey.test(new_worry_level);
                to_push
                    .get_mut(&target_monkey)
                    .unwrap()
                    .push_back(new_worry_level);
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn part1(monkeys: &mut [Monkey]) -> u64 {
    run(monkeys, 20, &|x| x / 3)
}

fn part2(monkeys: &mut [Monkey]) -> u64 {
    let divisor_products = monkeys.iter().map(|m| m.test.divisor).product::<u64>();
    run(monkeys, 10000, &|x| x % divisor_products)
}

fn main() -> Result<()> {
    let monkeys = read_chunks("input/day11.txt")?;
    let result = part1(&mut monkeys.clone());
    println!("part 1: {}", result);
    let result = part2(&mut monkeys.clone());
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let monkeys = read_chunks("input/test/day11.txt")?;
    let result = part1(&mut monkeys.clone());
    assert_eq!(result, 10605);

    let result = part2(&mut monkeys.clone());
    assert_eq!(result, 2713310158);

    Ok(())
}
