use anyhow::Result;
use num::Integer;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct InputParseError;

impl fmt::Display for InputParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went horribly wrong.")
    }
}

impl std::error::Error for InputParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub fn parse_comma_separated<F: FromStr>(line: &str) -> Result<Vec<F>>
where
    <F as FromStr>::Err: std::error::Error,
    <F as FromStr>::Err: Send,
    <F as FromStr>::Err: Sync,
    <F as FromStr>::Err: 'static,
{
    line.split(',')
        .map(|i| Ok(i.parse::<F>()?))
        .collect::<Result<Vec<_>, _>>()
}

pub fn read_comma_separated<F: FromStr>(path: &str) -> Result<Vec<F>>
where
    <F as FromStr>::Err: std::error::Error,
    <F as FromStr>::Err: Send,
    <F as FromStr>::Err: Sync,
    <F as FromStr>::Err: 'static,
{
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .next()
        .map(|s| parse_comma_separated(s?.as_str()))
        .unwrap()
}

pub fn read_chunks<F: FromStr>(path: &str) -> Result<Vec<F>>
where
    <F as FromStr>::Err: std::error::Error,
    <F as FromStr>::Err: Send,
    <F as FromStr>::Err: Sync,
    <F as FromStr>::Err: 'static,
{
    let contents = fs::read_to_string(path)?;

    contents
        .split("\n\n")
        .map(|line| Ok(line.parse::<F>()?))
        .collect()
}

pub fn read_lines<F: FromStr>(path: &str) -> Result<Vec<F>>
where
    <F as FromStr>::Err: std::error::Error,
    <F as FromStr>::Err: Send,
    <F as FromStr>::Err: Sync,
    <F as FromStr>::Err: 'static,
{
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Ok(line?.parse::<F>()?))
        .collect()
}

pub fn read_file<F: FromStr>(path: &str) -> Result<F>
where
    <F as FromStr>::Err: std::error::Error,
    <F as FromStr>::Err: Send,
    <F as FromStr>::Err: Sync,
    <F as FromStr>::Err: 'static,
{
    let contents = fs::read_to_string(path)?;
    let result = contents.parse::<F>()?;
    Ok(result)
}

pub fn read_grid<PI: Integer, VI: Integer>(path: &str) -> Result<HashMap<(PI, PI), VI>>
where
    VI: FromStr,
    <VI as FromStr>::Err: std::error::Error,
    <VI as FromStr>::Err: Send,
    <VI as FromStr>::Err: Sync,
    <VI as FromStr>::Err: 'static,
    VI: Copy,
    PI: TryFrom<usize>,
    <PI as TryFrom<usize>>::Error: std::error::Error,
    <PI as TryFrom<usize>>::Error: Send,
    <PI as TryFrom<usize>>::Error: Sync,
    <PI as TryFrom<usize>>::Error: 'static,
    PI: Hash,
{
    let lines: Vec<String> = read_lines(path)?;
    let layout_vec = lines
        .iter()
        .map(|l| {
            l.chars()
                .filter(|&c| c != '\n')
                .map(|c| c.to_string())
                .map(|s| Ok(s.parse::<VI>()?))
                .collect::<Result<Vec<VI>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;
    let result = layout_vec
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &num)| Ok(((PI::try_from(x)?, PI::try_from(y)?), num)))
        })
        .collect::<Result<_>>()?;
    Ok(result)
}
