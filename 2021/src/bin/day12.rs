use std::collections::{HashMap, HashSet};

use anyhow::Result;
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Eq, Hash, Debug, Clone)]
enum Cave {
    #[display("start")]
    Start,
    #[display("end")]
    End,
    #[from_str(regex = "(?P<0>[A-Z]+)")]
    Big(#[display("[A-Z]+")] String),
    #[from_str(regex = "(?P<0>[a-z]+)")]
    Small(#[display("[a-z]+")] String),
}

#[derive(parse_display::FromStr, PartialEq, Debug, Clone)]
#[display("{0}-{1}")]
struct Connection(Cave, Cave);

type CaveGraph = HashMap<Cave, HashSet<Cave>>;

fn make_cave_graph(caves: &[Connection]) -> CaveGraph {
    let mut map = HashMap::new();
    for Connection(cave1, cave2) in caves.iter() {
        map.entry(cave1.to_owned())
            .or_insert_with(HashSet::new)
            .insert(cave2.to_owned());
        map.entry(cave2.to_owned())
            .or_insert_with(HashSet::new)
            .insert(cave1.to_owned());
    }
    map
}

fn count_exit_paths<F>(
    caves: &CaveGraph,
    current_cave: &Cave,
    previously_visited: &mut HashMap<Cave, u8>,
    can_visit_next: &F,
) -> usize
where
    F: Fn(&Cave, &HashMap<Cave, u8>) -> bool,
{
    if current_cave == &Cave::End {
        return 1;
    }
    let mut count = 0;
    *previously_visited
        .entry(current_cave.to_owned())
        .or_insert_with(|| 0) += 1;
    for adjoining_cave in caves[current_cave].iter() {
        if !can_visit_next(adjoining_cave, previously_visited) {
            continue;
        }
        count += count_exit_paths(caves, adjoining_cave, previously_visited, can_visit_next);
    }
    *previously_visited
        .entry(current_cave.to_owned())
        .or_insert_with(|| 0) -= 1;
    count
}

fn part1(caves: &CaveGraph) -> usize {
    count_exit_paths(
        caves,
        &Cave::Start,
        &mut HashMap::new(),
        &|cave, previously_visited| {
            // If it's a big cave
            matches!(cave, Cave::Big(_)) 
                // Or we haven't been there before
                || previously_visited.get(cave).unwrap_or(&0) == &0
        },
    )
}

fn part2(caves: &CaveGraph) -> usize {
    count_exit_paths(
        caves,
        &Cave::Start,
        &mut HashMap::new(),
        &|cave, previously_visited| {
            // If it's a big cave
            matches!(cave, Cave::Big(_))
                // or we haven't been there before
                || previously_visited.get(cave).unwrap_or(&0) == &0
                    // or it's a small cave
                || (matches!(cave, Cave::Small(_))
                    // that we've only visted once
                    && previously_visited.get(cave).unwrap_or(&0) == &1
                    // and we haven't visted any other small cave twice already
                    && !previously_visited
                        .iter()
                        .filter(|&(c, _)| matches!(c, Cave::Small(_)))
                        .any(|(_, &cnt)| cnt == 2))
        },
    )
}

fn main() -> Result<()> {
    let connections: Vec<Connection> = read_lines("input/day12.txt")?;
    let cave_graph = make_cave_graph(&connections);
    let result = part1(&cave_graph);
    println!("part 1: {}", result);
    let result = part2(&cave_graph);
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        let connections: Vec<Connection> = read_lines("input/test/day12.txt")?;
        let cave_graph = make_cave_graph(&connections);
        let result = part1(&cave_graph);
        assert_eq!(result, 226);
        let result = part2(&cave_graph);
        assert_eq!(result, 3509);

        Ok(())
    }
}
