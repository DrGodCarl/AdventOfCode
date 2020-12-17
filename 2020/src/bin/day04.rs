#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use utils::read_file;

type PassportRecord<'a> = HashMap<&'a str, &'a str>;

fn parse_passport_record(s: &str) -> PassportRecord {
    s.split_whitespace()
        .flat_map(|rec| rec.split(':'))
        .tuples()
        .collect()
}

fn parse_passport_records(s: &str) -> Vec<PassportRecord<'_>> {
    s.split("\n\n").map(|r| parse_passport_record(r)).collect()
}

fn validate_fields(passport_record: &PassportRecord) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&k| passport_record.contains_key(k))
}

fn validate_content(passport_record: &PassportRecord) -> bool {
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref EYE_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    fn is_between(s: &str, start: usize, end: usize) -> bool {
        let i = s.parse().unwrap_or(0);
        i >= start && i <= end
    }
    passport_record.iter().all(|(&k, &v)| match k {
        "byr" => is_between(v, 1920, 2002),
        "iyr" => is_between(v, 2010, 2020),
        "eyr" => is_between(v, 2020, 2030),
        "hgt" => HEIGHT_RE
            .captures(v)
            .and_then(|c| {
                c.get(1)
                    .zip(c.get(2))
                    .map(|(c1, c2)| (c1.as_str(), c2.as_str()))
                    .map(|(num, unit)| match unit {
                        "in" => is_between(num, 59, 76),
                        _ => is_between(num, 150, 193),
                    })
            })
            .unwrap_or(false),
        "hcl" => HAIR_RE.is_match(v),
        "ecl" => EYE_RE.is_match(v),
        "pid" => PID_RE.is_match(v),
        _ => true,
    })
}

fn part1(passports: &[PassportRecord]) -> usize {
    passports.iter().filter(|&p| validate_fields(p)).count()
}

fn part2(passports: &[PassportRecord]) -> usize {
    passports
        .iter()
        .filter(|&p| validate_fields(p))
        .filter(|&p| validate_content(p))
        .count()
}

fn main() -> Result<()> {
    let passport_input: String = read_file("input/day04.txt")?;
    let passport_records: Vec<PassportRecord> = parse_passport_records(&passport_input);
    let result = part1(&passport_records);
    println!("part 1: {}", result);
    let result = part2(&passport_records);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one_record() {
        let _: PassportRecord = parse_passport_record(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
             byr:1937 iyr:2017 cid:147 hgt:183cm",
        );
        let _: PassportRecord = parse_passport_record(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
             hcl:#cfa07d byr:1929",
        );
    }

    #[test]
    fn test_part1() {
        let passport_input = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
             byr:1937 iyr:2017 cid:147 hgt:183cm\n\
             \n\
             iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
             hcl:#cfa07d byr:1929\n\
             \n\
             hcl:#ae17e1 iyr:2013\n\
             eyr:2024\n\
             ecl:brn pid:760753108 byr:1931\n\
             hgt:179cm\n\
             \n\
             hcl:#cfa07d eyr:2025 pid:166559648\n\
             iyr:2011 ecl:brn hgt:59in",
        );
        let passports = parse_passport_records(&passport_input);
        let result = part1(&passports);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let bad_passport_input = String::from(
            "eyr:1972 cid:100\n\
             hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
             \n\
             iyr:2019\n\
             hcl:#602927 eyr:1967 hgt:170cm\n\
             ecl:grn pid:012533040 byr:1946\n\
             \n\
             hcl:dab227 iyr:2012\n\
             ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
             \n\
             hgt:59cm ecl:zzz\n\
             eyr:2038 hcl:74454a iyr:2023\n\
             pid:3556412378 byr:2007",
        );
        let bad_passports = parse_passport_records(&bad_passport_input);

        let result = part2(&bad_passports);
        assert_eq!(result, 0);

        let good_passport_input = String::from(
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
             hcl:#623a2f\n\
             \n\
             eyr:2029 ecl:blu cid:129 byr:1989\n\
             iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
             \n\
             hcl:#888785\n\
             hgt:164cm byr:2001 iyr:2015 cid:88\n\
             pid:545766238 ecl:hzl\n\
             eyr:2022\n\
             \n\
             iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        );
        let good_passports = parse_passport_records(&good_passport_input);

        let result = part2(&good_passports);
        assert_eq!(result, 4);
    }
}
