use std::fs;
use anyhow::Result;

pub fn read_input(path: &str) -> Result<Vec<i32>> {
    let contents = fs::read_to_string(path)?;
    contents
        .split(',')
        .map(|s| Ok(s.parse::<i32>()?))
        .collect()
}
