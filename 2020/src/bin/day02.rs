use anyhow::Result;
use parse_display::{Display, FromStr};
use utils::read_lines;

#[derive(Debug, FromStr, Display)]
#[display("{min}-{max} {required}: {password}")]
struct PasswordRow {
    min: usize,
    max: usize,
    required: char,
    password: String,
}

fn part1(passwords: &[PasswordRow]) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let char_count = p.password.matches(p.required).count();
            p.min <= char_count && p.max >= char_count
        })
        .count()
}

fn part2(passwords: &[PasswordRow]) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let min_char = p.password.as_bytes()[p.min - 1] as char;
            let max_char = p.password.as_bytes()[p.max - 1] as char;
            (min_char == p.required || max_char == p.required) && max_char != min_char
        })
        .count()
}

fn main() -> Result<()> {
    let passwords = read_lines("input/day02.txt")?;
    let result = part1(&passwords);
    println!("part 1: {}", result);
    let result = part2(&passwords);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let row_strings = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
    let password_rows = row_strings
        .iter()
        .map(|s| Ok(s.parse::<PasswordRow>()?))
        .collect::<Result<Vec<PasswordRow>>>()?;
    let result = part1(&password_rows);
    assert_eq!(result, 2);

    let result = part2(&password_rows);
    assert_eq!(result, 1);

    Ok(())
}
