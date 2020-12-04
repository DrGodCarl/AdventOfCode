#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, str::FromStr};

use anyhow::Result;
use regex::Regex;
use utils::{read_file, InputParseError};

#[derive(Debug)]
struct PassportRecord {
    fields: HashMap<String, String>,
}

impl PassportRecord {
    fn is_valid(&self, validators: &Vec<Validator>) -> bool {
        validators.iter().all(|v| v.validate(self))
    }
}

struct Wrapper(Vec<PassportRecord>);

impl FromStr for PassportRecord {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: HashMap<_, _> = s
            .split_whitespace()
            .map(|rec| {
                let mut pair = rec.split(':');
                Some((String::from(pair.next()?), String::from(pair.next()?)))
            })
            .filter_map(|a| a)
            .collect();
        Ok(PassportRecord { fields })
    }
}

impl FromStr for Wrapper {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let record_texts: Result<Vec<_>, _> = s
            .split("\n\n")
            .map(|r| r.parse::<PassportRecord>())
            .collect();
        record_texts.map(Wrapper)
    }
}

struct Height {
    number: u8,
    unit: HeightUnit,
}

enum HeightUnit {
    IN,
    CM,
}

impl FromStr for Height {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        }
        let captures = HEIGHT_RE.captures(s).ok_or(InputParseError)?;
        let number = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| InputParseError)?;
        let unit = match captures.get(2).unwrap().as_str() {
            "in" => HeightUnit::IN,
            _ => HeightUnit::CM,
        };
        Ok(Height { number, unit })
    }
}

struct Validator<'a> {
    key: &'a String,
    validation: fn(&PassportRecord, &String) -> bool,
}

impl<'a> Validator<'a> {
    fn validate(&self, passport: &PassportRecord) -> bool {
        (self.validation)(passport, self.key)
    }
}

fn existence_validator<'a>(key: &'a String) -> Validator<'a> {
    Validator {
        key,
        validation: |p, k| p.fields.contains_key(k),
    }
}

lazy_static! {
    static ref KEYS: Vec<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|&s| String::from(s))
        .collect();
}

fn part1(passports: &Vec<PassportRecord>) -> usize {
    let validators = KEYS.iter().map(|s| existence_validator(s)).collect();
    passports
        .iter()
        .filter(|&p| PassportRecord::is_valid(p, &validators))
        .count()
}

fn part2(passports: &Vec<PassportRecord>) -> usize {
    lazy_static! {
        static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref EYE_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    let mut validators: Vec<_> = KEYS.iter().map(|s| existence_validator(s)).collect();
    let content_validators = vec![
        Validator {
            key: &KEYS[0],
            validation: |p, k| {
                p.fields[k]
                    .parse::<usize>()
                    .map(|y| y >= 1920 && y <= 2002)
                    .unwrap_or(false)
            },
        },
        Validator {
            key: &KEYS[1],
            validation: |p, k| {
                p.fields[k]
                    .parse::<usize>()
                    .map(|y| y >= 2010 && y <= 2020)
                    .unwrap_or(false)
            },
        },
        Validator {
            key: &KEYS[2],
            validation: |p, k| {
                p.fields[k]
                    .parse::<usize>()
                    .map(|y| y >= 2020 && y <= 2030)
                    .unwrap_or(false)
            },
        },
        Validator {
            key: &KEYS[3],
            validation: |p, k| {
                p.fields[k]
                    .parse::<Height>()
                    .map(|h| match h.unit {
                        HeightUnit::IN => h.number >= 59 && h.number <= 76,
                        HeightUnit::CM => h.number >= 150 && h.number <= 193,
                    })
                    .unwrap_or(false)
            },
        },
        Validator {
            key: &KEYS[4],
            validation: |p, k| HAIR_RE.is_match(p.fields[k].as_str()),
        },
        Validator {
            key: &KEYS[5],
            validation: |p, k| EYE_RE.is_match(p.fields[k].as_str()),
        },
        Validator {
            key: &KEYS[6],
            validation: |p, k| PID_RE.is_match(p.fields[k].as_str()),
        },
    ];
    validators.extend(content_validators);
    passports
        .iter()
        .filter(|&p| PassportRecord::is_valid(p, &validators))
        .count()
}

fn main() -> Result<()> {
    let passports: Wrapper = read_file("input/day04.txt")?;
    let result = part1(&passports.0);
    println!("part 1: {}", result);
    let result = part2(&passports.0);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one_record() -> Result<()> {
        let _: PassportRecord = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm"
            .parse()?;
        let _: PassportRecord = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929"
            .parse()?;
        Ok(())
    }

    static PASSPORT_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_parse_many_records() -> Result<()> {
        let _: Wrapper = PASSPORT_INPUT.parse()?;
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let passports: Wrapper = PASSPORT_INPUT.parse()?;
        let result = part1(&passports.0);
        assert_eq!(result, 2);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let bad_passports: Wrapper = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            .parse()?;
        let result = part2(&bad_passports.0);
        assert_eq!(result, 0);

        let good_passports: Wrapper = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            .parse()?;
        let result = part2(&good_passports.0);
        assert_eq!(result, 4);
        Ok(())
    }
}
