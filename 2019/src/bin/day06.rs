use std::fs;

use anyhow::{Result, Context};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
struct Orbit {
    name: String,
    orbiting: Vec<Orbit>,
}

impl Orbit {
    fn checksum(&self) -> i32 {
        self._checksum(1)
    }

    fn _checksum(&self, prev_count: i32) -> i32 {
        self.orbiting.iter().map(|o| o._checksum(prev_count + 1) + prev_count).sum()
    }

    fn shortest_distance_between(&self, name1: String, name2: String) -> Option<usize> {
        let stops1 = self.stops_on_the_way(&name1)?;
        let stops2 = self.stops_on_the_way(&name2)?;
        Some(stops1.symmetric_difference(&stops2).cloned().unique().count())
    }

    fn stops_on_the_way(&self, name: &str) -> Option<HashSet<String>> {
        if name.to_owned() == self.name {
            let mut names = HashSet::new();
            names.insert(self.name.clone());
            Some(names)
        } else {
            self.orbiting.iter()
                .map(|o| o.stops_on_the_way(name))
                .filter(|c| c.is_some())
                .map(|opt| opt.unwrap())
                .next()
                .map(|mut c| {
                    c.insert(self.name.clone());
                    c
                })
        }
    }
}

fn read_input() -> Result<Orbit> {
    let contents = fs::read_to_string("input/day06.txt")?;
    parse(&contents)
}

fn parse(input: &str) -> Result<Orbit> {
    let orbital_pairs: Result<Vec<(String, String)>> = input.lines()
        .map(|line| line.trim().split(')').map(|s| s.to_owned()).collect())
        .map(|split: Vec<String>| {
            Ok((
                split.first().context("invalid input")?.to_owned(),
                split.last().context("invalid input")?.to_owned(),
            ))
        }).collect();
    let orbits: HashMap<String, Vec<String>> = orbital_pairs.map(|v: Vec<(String, String)>| v.into_iter().into_group_map())?;
    Ok(build("COM", &orbits))
}

fn build(center_of_gravity: &str, planets: &HashMap<String, Vec<String>>) -> Orbit {
    let empty = vec![];
    let orbiting = planets.get(center_of_gravity).unwrap_or(&empty);
    Orbit {
        name: center_of_gravity.to_owned(),
        orbiting: orbiting.iter().map(|o| build(o, planets)).collect(),
    }
}

fn part1(orbit: &Orbit) -> i32 {
    orbit.checksum()
}

fn part2(orbit: &Orbit) -> Result<usize> {
    Ok(orbit.shortest_distance_between("YOU".to_owned(), "SAN".to_owned())
        .context("No path between them.")? - 2)
}

fn main() -> Result<()> {
    let orbit = read_input()?;

    let result1 = part1(&orbit);
    println!("part 1: {}", result1);

    let result2 = part2(&orbit);
    println!("part 2: {:?}", result2?);

    Ok(())
}

#[test]
fn test1() {
    let input = "COM)B
                 B)C
                 C)D
                 D)E
                 E)F
                 B)G
                 G)H
                 D)I
                 E)J
                 J)K
                 K)L";
    /*
            G - H       J - K - L
           /           /
    COM - B - C - D - E - F
                   \
                    I
    */
    let parsed = parse(input).unwrap();
    assert_eq!(parsed.checksum(), 42);
}


#[test]
fn test2() {
    let input = "COM)B
                 B)C
                 C)D
                 D)E
                 E)F
                 B)G
                 G)H
                 D)I
                 E)J
                 J)K
                 K)L
                 K)YOU
                 I)SAN";
    let parsed = parse(input).unwrap();
    assert_eq!(part2(&parsed).unwrap(), 4);
}
