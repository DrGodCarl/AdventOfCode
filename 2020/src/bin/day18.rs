use anyhow::Result;
use nom::{
    branch::alt, bytes::complete::take_while, character::complete::char, combinator::map,
    combinator::map_res, sequence::delimited, sequence::pair, IResult,
};
use utils::read_lines;

type Number = u64;

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Number(Number),
}

fn is_char_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_number(input: &str) -> IResult<&str, Number> {
    map_res(take_while(is_char_digit), str::parse)(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), parse_expr, char(')'))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((map(parse_number, Expr::Number), parse_parens))(input)
}

fn parse_mul(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('*'), parse_expr);
    let (input, (lhs, (_, rhs))) = pair(parse_factor, sub)(input)?;
    Ok((input, Expr::Mul(Box::new(lhs), Box::new(rhs))))
}

fn parse_add(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('+'), parse_expr);
    let (input, (lhs, (_, rhs))) = pair(parse_factor, sub)(input)?;
    Ok((input, Expr::Add(Box::new(lhs), Box::new(rhs))))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_add, parse_mul, parse_factor))(input)
}

fn parse(input: &str) -> Result<Expr, String> {
    let i = input.trim().replace(" ", "");
    parse_expr(&i)
        .map_err(|e| format!("{}", e))
        .and_then(|(remaining, parsed)| {
            if !remaining.is_empty() {
                return Err(format!("Parsed: {:?}, Remaining, {}", parsed, remaining));
            }
            Ok(parsed)
        })
}

fn parse_parens_adv(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), parse_expr_adv, char(')'))(input)
}

fn parse_factor_adv(input: &str) -> IResult<&str, Expr> {
    alt((map(parse_number, Expr::Number), parse_parens_adv))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    alt((parse_add_adv, parse_factor_adv))(input)
}

fn parse_mul_adv(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('*'), parse_expr_adv);
    let (input, (lhs, (_, rhs))) = pair(parse_term, sub)(input)?;
    Ok((input, Expr::Mul(Box::new(lhs), Box::new(rhs))))
}

fn parse_add_adv(input: &str) -> IResult<&str, Expr> {
    let sub = pair(char('+'), parse_term);
    let (input, (lhs, (_, rhs))) = pair(parse_factor_adv, sub)(input)?;
    Ok((input, Expr::Add(Box::new(lhs), Box::new(rhs))))
}

fn parse_expr_adv(input: &str) -> IResult<&str, Expr> {
    alt((parse_mul_adv, parse_term))(input)
}

fn parse_adv(input: &str) -> Result<Expr, String> {
    let i = input.trim().replace(" ", "");
    parse_expr_adv(&i)
        .map_err(|e| format!("{}", e))
        .and_then(|(remaining, parsed)| {
            if !remaining.is_empty() {
                return Err(format!("Parsed: {:?}, Remaining, {}", parsed, remaining));
            }
            Ok(parsed)
        })
}

fn walk(ast: &Expr) -> Number {
    match ast {
        Expr::Add(lhs, rhs) => {
            let lhs = walk(lhs);
            let rhs = walk(rhs);
            lhs + rhs
        }
        Expr::Mul(lhs, rhs) => {
            let lhs = walk(lhs);
            let rhs = walk(rhs);
            lhs * rhs
        }
        Expr::Number(num) => *num,
    }
}

fn calculate(equation: &str) -> Result<Number, String> {
    // So I helped write a little calculator - https://github.com/brian-dawn/trusty-calculator
    // and decided to use chunks of it to make my life easier.
    // It wasn't working, but I realized it was evaluating right-to-left
    // instead of left-to-right. I tried for a bit to get it to go the
    // other way but it was much easier to flip the input.
    let equation: String = equation
        .chars()
        .rev()
        .map(|c| match c {
            '(' => ')',
            ')' => '(',
            a => a,
        })
        .collect();
    parse(&equation).map(|expr| walk(&expr))
}

fn calculate_adv(equation: &str) -> Result<Number, String> {
    parse_adv(equation).map(|expr| walk(&expr))
}

fn part1(equations: &[String]) -> Number {
    equations.iter().filter_map(|s| calculate(s).ok()).sum()
}

fn part2(equations: &[String]) -> Number {
    equations.iter().filter_map(|s| calculate_adv(s).ok()).sum()
}

fn main() -> Result<()> {
    let equations = read_lines("input/day18.txt")?;
    let result = part1(&equations);
    println!("part 1: {}", result);

    let equations = read_lines("input/day18.txt")?;
    let result = part2(&equations);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() -> Result<(), String> {
        let result = calculate("(2 * 3) * (1 + 1)")?;
        assert_eq!(result, 12);
        let result = calculate("2 * 3 + (4 * 5)")?;
        assert_eq!(result, 26);
        let result = calculate("5 + (8 * 3 + 9 + 3 * 4 * 3)")?;
        assert_eq!(result, 437);
        Ok(())
    }

    #[test]
    fn test_calculate_adv() -> Result<(), String> {
        let result = calculate_adv("1 + (2 * 3) + (4 * (5 + 6))")?;
        assert_eq!(result, 51);
        let result = calculate_adv("2 * 3 + (4 * 5)")?;
        assert_eq!(result, 46);
        let result = calculate_adv("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")?;
        assert_eq!(result, 669060);
        let result = calculate_adv("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")?;
        assert_eq!(result, 23340);
        Ok(())
    }
}
