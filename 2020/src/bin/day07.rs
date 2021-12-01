use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    string::ParseError,
};

use anyhow::Result;
use regex::Regex;
use utils::read_file;

struct BagGraph {
    map: HashMap<String, Vec<(usize, String)>>,
}

impl FromStr for BagGraph {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule_re = Regex::new(r"(\w+ \w+) bags contain (.*)\.\n?").unwrap();
        let bag_count_re = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

        let mut graph = HashMap::new();

        for cap in rule_re.captures_iter(s) {
            let container = String::from(cap.get(1).unwrap().as_str());
            let children_str = cap.get(2).unwrap().as_str();
            let entry = graph.entry(container).or_insert_with(Vec::new);
            for child_cap in bag_count_re.captures_iter(children_str) {
                let child = String::from(child_cap.get(2).unwrap().as_str());
                let count = child_cap.get(1).unwrap().as_str().parse().unwrap();
                entry.push((count, child));
            }
        }

        Ok(BagGraph { map: graph })
    }
}

impl BagGraph {
    fn inverted(&self) -> BagGraph {
        let mut result: HashMap<String, Vec<(usize, String)>> = HashMap::new();

        for (bag, contents) in self.map.clone() {
            for (num, inner_bag) in contents {
                let entry = result.entry(inner_bag).or_insert_with(Vec::new);
                entry.push((num, bag.clone()));
            }
        }

        BagGraph { map: result }
    }
}

fn _count_all_containing_bags(
    bag: &str,
    bag_graph: &BagGraph,
    visited: &mut HashSet<String>,
) -> usize {
    if visited.contains(bag) {
        0
    } else {
        visited.insert(bag.to_string());
        bag_graph
            .map
            .get(bag)
            .iter()
            .flat_map(|&containers| containers.iter())
            .map(|(_, inner)| _count_all_containing_bags(inner, bag_graph, visited))
            .sum::<usize>()
            + 1
    }
}

fn count_all_containing_bags(bag: &str, bag_graph: &BagGraph) -> usize {
    let mut visited = HashSet::new();
    // -1 because the recursive function counts the bag you're currently on and we don't want to count this one.
    _count_all_containing_bags(bag, bag_graph, &mut visited) - 1
}

fn count_all_internal_bags(bag: &str, bag_graph: &BagGraph) -> usize {
    bag_graph
        .map
        .get(bag)
        .map(|l| {
            l.iter()
                .map(|(c, b)| c * count_all_internal_bags(b, bag_graph) + c)
                .sum()
        })
        .unwrap_or(0)
}

fn part1(bag_graph: &BagGraph) -> usize {
    let shiny_gold = "shiny gold";
    count_all_containing_bags(shiny_gold, bag_graph)
}

fn part2(bag_graph: &BagGraph) -> usize {
    let shiny_gold = "shiny gold";
    count_all_internal_bags(shiny_gold, bag_graph)
}

fn main() -> Result<()> {
    let graph: BagGraph = read_file("input/day07.txt")?;
    let inverted_graph = graph.inverted();
    let result = part1(&inverted_graph);
    println!("part 1: {}", result);
    let result = part2(&graph);
    println!("part 2: {}", result);
    Ok(())
}
