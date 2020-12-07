use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    string::ParseError,
};

use anyhow::Result;
use regex::Regex;
use utils::read_file;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Bag {
    color: String,
}

impl Bag {
    fn new(color: &str) -> Self {
        Bag {
            color: String::from(color),
        }
    }
}

struct InvertedBagGraph {
    map: HashMap<Bag, HashSet<Bag>>,
}

impl FromStr for InvertedBagGraph {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule_re = Regex::new(r"(\w+ \w+) bags contain (.*)\.\n?").unwrap();
        let bag_count_re = Regex::new(r"\d+ (\w+ \w+) bags?").unwrap();

        let mut graph = HashMap::new();

        for cap in rule_re.captures_iter(s) {
            let container = Bag::new(cap.get(1).unwrap().as_str());
            let children_str = cap.get(2).unwrap().as_str();
            for child_cap in bag_count_re.captures_iter(children_str) {
                let child = Bag::new(child_cap.get(1).unwrap().as_str());
                let entry = graph.entry(child).or_insert_with(|| HashSet::new());
                entry.insert(container.clone());
            }
        }

        Ok(InvertedBagGraph { map: graph })
    }
}

fn count_all_containing_bags(bag: &Bag, bag_graph: &InvertedBagGraph) -> usize {
    let mut to_process: HashSet<&Bag> = HashSet::new();
    let mut processed: HashSet<&Bag> = HashSet::new();
    let default_set = HashSet::new();
    to_process.extend(bag_graph.map.get(bag).unwrap_or(&default_set));
    while to_process.len() > 0 {
        let bag = to_process.iter().take(1).map(|&i| i).collect::<Vec<_>>()[0];
        processed.insert(bag);
        to_process.extend(bag_graph.map.get(bag).unwrap_or(&default_set));
        to_process.take(bag);
    }
    processed.len()
}

fn part1(bag_graph: &InvertedBagGraph) -> usize {
    let shiny_gold = Bag::new("shiny gold");
    count_all_containing_bags(&shiny_gold, bag_graph)
}

fn part2(bag_graph: &InvertedBagGraph) -> usize {
    0
}

fn main() -> Result<()> {
    let graph: InvertedBagGraph = read_file("input/day07.txt")?;
    let result = part1(&graph);
    println!("part 1: {}", result);
    let result = part2(&graph);
    println!("part 2: {}", result);
    Ok(())
}
