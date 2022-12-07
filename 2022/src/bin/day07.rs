use anyhow::Result;
use std::{collections::HashMap, str::FromStr};
use utils::{read_chunks_delimited, InputParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    size: u32,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    CdRoot,
    CdUp,
    Cd(String),
    Ls(Vec<File>),
}

impl FromStr for Command {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The first one keeps its $ prefix.
        let s = s.strip_prefix("$ ").unwrap_or(s);
        if s == "cd .." {
            return Ok(Command::CdUp);
        }
        if s == "cd /" {
            return Ok(Command::CdRoot);
        }
        if s.starts_with("cd ") {
            let dir = s.split(" ").nth(1).unwrap();
            return Ok(Command::Cd(dir.to_string()));
        }
        if s.starts_with("ls") {
            let internals = s
                .split('\n')
                .skip(1)
                .map(|l| l.split_whitespace().collect::<Vec<_>>())
                .filter_map(|parts| {
                    // Ignoring dirs for now because they, at the moment, aren't useful.
                    parts[0]
                        .parse()
                        .map(|size| File {
                            size,
                            name: parts[1].to_string(),
                        })
                        .ok()
                })
                .collect();
            return Ok(Command::Ls(internals));
        }
        println!("Failed to parse: {}", s);
        Err(InputParseError)
    }
}

fn calculate_fs_size(commands: &[Command]) -> HashMap<String, u32> {
    let mut file_system_size: HashMap<String, u32> = HashMap::new();
    let mut current_dir = vec!["/"];

    for command in commands {
        match command {
            Command::CdRoot => current_dir = vec!["/"],
            Command::CdUp => {
                current_dir.pop();
            }
            Command::Cd(dir) => current_dir.push(dir),
            Command::Ls(internals) => {
                let sum = internals.iter().map(|f| f.size).sum::<u32>();
                let mut key_dir = "".to_string();
                // For the current dir and its parent, add the sum of the files.
                for &dir in current_dir.iter() {
                    key_dir.push_str(dir);
                    *file_system_size.entry(key_dir.clone()).or_insert(0) += sum;
                }
            }
        }
    }
    file_system_size
}

fn part1(commands: &[Command]) -> u32 {
    calculate_fs_size(commands)
        .values()
        .filter(|&&s| s <= 100000)
        .sum()
}

fn part2(commands: &[Command]) -> Option<u32> {
    let total_size = 70000000;
    let needed_size = 30000000;
    let fs_size = calculate_fs_size(commands);
    let used_size = fs_size.values().max().unwrap();
    let free_size = total_size - used_size;
    let to_free = needed_size - free_size;
    fs_size.values().filter(|&&s| s >= to_free).min().copied()
}

fn main() -> Result<()> {
    let commands = read_chunks_delimited("input/day07.txt", "\n$ ")?;
    let result = part1(&commands);
    println!("part 1: {}", result);
    let result = part2(&commands);
    println!("part 2: {:?}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let commands: Vec<Command> = read_chunks_delimited("input/test/day07.txt", "\n$ ")?;
    let result = part1(&commands);
    assert_eq!(result, 95437);

    let result = part2(&commands);
    assert_eq!(result, Some(24933642));

    Ok(())
}
