#[macro_use]
extern crate lazy_static;
use std::{cmp::min, collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use parse_display::{self, ParseError};
use regex::Regex;
use utils::{read_chunks, InputParseError};

struct MapChunk {
    id: u64,
    edges: Vec<String>,
}

impl FromStr for MapChunk {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }
        let lines: Vec<&str> = s.split('\n').collect();
        let id_line = lines.get(0).ok_or(InputParseError)?;
        let id = ID_RE
            .captures(id_line)
            .and_then(|c| c.get(1))
            .map(|i| i.as_str().parse().map_err(|_| InputParseError))
            .unwrap_or(Err(InputParseError))?;
        let chunk = &lines[1..];
        let top = chunk[0].to_string();
        let bottom = chunk[chunk.len() - 1].to_string();
        let left = chunk
            .iter()
            .map(|l| l.chars().next())
            .collect::<Option<Vec<char>>>()
            .map(|l| l.iter().join(""))
            .ok_or(InputParseError)?;
        let right = chunk
            .iter()
            .map(|l| l.chars().last())
            .collect::<Option<Vec<char>>>()
            .map(|l| l.iter().join(""))
            .ok_or(InputParseError)?;
        Ok(MapChunk {
            id,
            edges: vec![top, bottom, left, right],
        })
    }
}

trait Reversable {
    fn reverse(&self) -> Self;
}

impl Reversable for String {
    fn reverse(&self) -> Self {
        self.chars().rev().collect()
    }
}

fn count_edges_with_matches(chunk: &MapChunk, all_edges: &[String]) -> usize {
    chunk
        .edges
        .iter()
        .filter(|&e| {
            all_edges
                .iter()
                .filter(|&ae| ae == e || &ae.reverse() == e)
                .count()
                == 2
        })
        .count()
}

fn part1(map_chunks: &[MapChunk]) -> u64 {
    let all_edges: Vec<String> = map_chunks.iter().flat_map(|c| c.edges.clone()).collect();
    map_chunks
        .iter()
        .filter(|&c| count_edges_with_matches(c, &all_edges) == 2)
        .map(|c| c.id)
        .product()
}

fn part2(map_chunks: &[MapChunk]) -> usize {
    0
}

fn main() -> Result<()> {
    let map_chunks = read_chunks("input/day20.txt")?;
    let result = part1(&map_chunks);
    println!("part 1: {}", result);

    let result = part2(&map_chunks);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let map_chunks = read_chunks("input/test/day20.txt")?;
        let result = part1(&map_chunks);
        assert_eq!(result, 20899048083289);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let map_chunks = read_chunks("input/test/day20.txt")?;
        let result = part2(&map_chunks);
        assert_eq!(result, 273);
        Ok(())
    }
}
