use anyhow::Result;
use std::fmt;
use std::fs;
use std::fs::File;
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
