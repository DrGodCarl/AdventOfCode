#![feature(iter_array_chunks)]
use std::str::FromStr;

use anyhow::Result;
use parse_display::FromStr;
use utils::{read_file, InputParseError};

#[derive(Debug, FromStr, PartialEq, Eq, PartialOrd, Ord)]
#[display("{destination_start} {source_start} {length}")]
struct IdMapping {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

impl IdMapping {
    fn get_destination(&self, source: i64) -> Option<i64> {
        if source < self.source_start || source >= self.source_start + self.length {
            return None;
        }
        return Some(self.destination_start - self.source_start + source);
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Mappings {
    SeedToSoil(Vec<IdMapping>),
    SoilToFertilizer(Vec<IdMapping>),
    FertilizerToWater(Vec<IdMapping>),
    WaterToLight(Vec<IdMapping>),
    LightToTemperature(Vec<IdMapping>),
    TemperatureToHumidity(Vec<IdMapping>),
    HumidityToLocation(Vec<IdMapping>),
}

struct Configuration {
    seeds: Vec<i64>,
    mappings: Vec<Mappings>,
}

impl FromStr for Configuration {
    type Err = InputParseError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut chunks = s.split("\n\n");
        let seeds = chunks
            .next()
            .ok_or(InputParseError)?
            .strip_prefix("seeds: ")
            .ok_or(InputParseError)?
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map_err(|_| InputParseError)?;
        let mut mappings = chunks
            .map(|s| s.parse::<Mappings>())
            .collect::<Result<Vec<Mappings>, _>>()
            .map_err(|_| InputParseError)?;
        mappings.sort();
        Ok(Configuration { seeds, mappings })
    }
}

impl Mappings {
    fn get_id_mapping(&self) -> &Vec<IdMapping> {
        match self {
            Mappings::SeedToSoil(mappings)
            | Mappings::SoilToFertilizer(mappings)
            | Mappings::FertilizerToWater(mappings)
            | Mappings::WaterToLight(mappings)
            | Mappings::LightToTemperature(mappings)
            | Mappings::TemperatureToHumidity(mappings)
            | Mappings::HumidityToLocation(mappings) => mappings,
        }
    }
}

impl FromStr for Mappings {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mappings = Vec::new();
        let mut lines = s.lines();
        let header = lines
            .next()
            .ok_or(InputParseError)?
            .strip_suffix(" map:")
            .ok_or(InputParseError)?;
        for line in lines {
            let mapping = line.parse::<IdMapping>().map_err(|_| InputParseError)?;
            mappings.push(mapping);
        }
        mappings.sort();
        Ok(match header {
            "seed-to-soil" => Mappings::SeedToSoil(mappings),
            "soil-to-fertilizer" => Mappings::SoilToFertilizer(mappings),
            "fertilizer-to-water" => Mappings::FertilizerToWater(mappings),
            "water-to-light" => Mappings::WaterToLight(mappings),
            "light-to-temperature" => Mappings::LightToTemperature(mappings),
            "temperature-to-humidity" => Mappings::TemperatureToHumidity(mappings),
            "humidity-to-location" => Mappings::HumidityToLocation(mappings),
            _ => panic!("unknown mapping"),
        })
    }
}

// mappings must be sorted by the enum order
fn seed_to_location(mappings: &[Mappings], seed: i64) -> i64 {
    mappings.iter().fold(seed, |acc, m| {
        m.get_id_mapping()
            .iter()
            .filter_map(|m| m.get_destination(acc))
            .next()
            .unwrap_or(acc)
    })
}

fn part1(config: &Configuration) -> i64 {
    config
        .seeds
        .iter()
        .map(|seed| seed_to_location(&config.mappings, *seed))
        .min()
        .unwrap()
}

fn part2(config: &Configuration) -> i64 {
    config
        .seeds
        .iter()
        .array_chunks()
        .flat_map(|[&start, &len]| (start..=start + len))
        .map(|seed| seed_to_location(&config.mappings, seed))
        .min()
        .unwrap()
}

fn main() -> Result<()> {
    let config = read_file("input/day05.txt")?;
    let result = part1(&config);
    println!("part 1: {}", result);
    let result = part2(&config);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let config: Configuration = read_file("input/test/day05.txt")?;
    let result = part1(&config);
    assert_eq!(result, 35);
    let result = part2(&config);
    assert_eq!(result, 46);
    Ok(())
}
