use anyhow::Result;
use utils::{read_grid, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NumberLocation {
    number: u32,
    y: u16,
    x_start: u16,
    x_end: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SymbolLocation {
    symbol: char,
    point: (u16, u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Gear {
    symbol: SymbolLocation,
    numbers: (NumberLocation, NumberLocation),
}

impl NumberLocation {
    fn is_adjacent_to_point(&self, point: &(u16, u16)) -> bool {
        let top_left = (self.x_start.saturating_sub(1), self.y.saturating_sub(1));
        let bottom_right = (self.x_end + 1, self.y + 1);
        top_left.0 <= point.0
            && point.0 <= bottom_right.0
            && top_left.1 <= point.1
            && point.1 <= bottom_right.1
    }
}

fn locate_symbols(values: &Grid<u16, char>) -> Vec<SymbolLocation> {
    values
        .iter()
        .filter(|(_, &symbol)| symbol != '.' && !symbol.is_ascii_digit())
        .map(|(&point, &symbol)| SymbolLocation { symbol, point })
        .collect()
}

fn locate_numbers(grid: &Grid<u16, char>) -> Vec<NumberLocation> {
    let mut result = Vec::new();
    let mut point = (0, 0);
    let mut character = grid.get(&point);
    while character.is_some() {
        while let Some(curr) = character {
            let mut curr = curr;
            let mut number = 0;
            let y = point.1;
            let x_start = point.0;
            while let Some(digit) = curr.to_digit(10) {
                number *= 10;
                number += digit;
                point.0 += 1;
                curr = grid.get(&point).unwrap_or(&'.');
            }
            if number > 0 {
                result.push(NumberLocation {
                    number,
                    y,
                    x_start,
                    x_end: point.0 - 1,
                });
            }
            point.0 += 1;
            character = grid.get(&point);
        }
        point.0 = 0;
        point.1 += 1;
        character = grid.get(&point);
    }
    result
}

fn find_numbers_with_symbol_neighbors(
    numbers: &[NumberLocation],
    symbols: &[SymbolLocation],
) -> Vec<NumberLocation> {
    numbers
        .iter()
        .filter(|n| {
            symbols
                .iter()
                .any(|symbol| n.is_adjacent_to_point(&symbol.point))
        })
        .cloned()
        .collect()
}

fn find_gears_with_two_number_neighbors(
    numbers: &[NumberLocation],
    symbols: &[SymbolLocation],
) -> Vec<Gear> {
    symbols
        .iter()
        .filter(|s| s.symbol == '*')
        .filter_map(|g| {
            let adjacent_numbers: Vec<_> = numbers
                .iter()
                .filter(|n| n.is_adjacent_to_point(&g.point))
                .collect();
            if adjacent_numbers.len() == 2 {
                Some(Gear {
                    symbol: g.clone(),
                    numbers: (adjacent_numbers[0].clone(), adjacent_numbers[1].clone()),
                })
            } else {
                None
            }
        })
        .collect()
}

fn part1(values: &Grid<u16, char>) -> u32 {
    let symbols = locate_symbols(values);
    let numbers = locate_numbers(values);
    let numbers_with_symbol_neighbors = find_numbers_with_symbol_neighbors(&numbers, &symbols);
    numbers_with_symbol_neighbors.iter().map(|n| n.number).sum()
}

fn part2(values: &Grid<u16, char>) -> u32 {
    let symbols = locate_symbols(values);
    let numbers = locate_numbers(values);
    let gears = find_gears_with_two_number_neighbors(&numbers, &symbols);
    gears
        .iter()
        .map(|g| g.numbers.0.number * g.numbers.1.number)
        .sum()
}

fn main() -> Result<()> {
    let grid = read_grid("input/day03.txt")?;
    let result = part1(&grid);
    println!("part 1: {}", result);
    let result = part2(&grid);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let grid = read_grid("input/test/day03.txt")?;
    let result = part1(&grid);
    assert_eq!(result, 4361);
    let result = part2(&grid);
    assert_eq!(result, 467835);
    Ok(())
}
