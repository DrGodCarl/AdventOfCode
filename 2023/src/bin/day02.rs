use std::str::FromStr;

use anyhow::Result;
use parse_display::FromStr;
use utils::{read_lines, InputParseError};

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let id = parts
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .map_err(|_| InputParseError)?;
        let rounds = parts
            .next()
            .unwrap()
            .trim()
            .split(';')
            .map(|s| s.trim().parse::<Round>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| InputParseError)?;
        Ok(Game { id, rounds })
    }
}

struct Round {
    color_counts: Vec<CubeCount>,
}

impl FromStr for Round {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color_counts = s
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<CubeCount>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| InputParseError)?;
        Ok(Round { color_counts })
    }
}

#[derive(FromStr, PartialEq, Eq, Hash, Debug)]
#[display(style = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(FromStr)]
#[display("{count} {color}")]
struct CubeCount {
    count: u32,
    color: Color,
}

impl CubeCount {
    fn red() -> Self {
        Self::make(Color::Red)
    }

    fn green() -> Self {
        Self::make(Color::Green)
    }

    fn blue() -> Self {
        Self::make(Color::Blue)
    }

    fn make(color: Color) -> Self {
        CubeCount { count: 0, color }
    }
}

struct CubeConfiguration {
    cubes: Vec<CubeCount>,
}

impl CubeConfiguration {
    fn is_possible(&self, cube_count: &CubeCount) -> bool {
        self.cubes
            .iter()
            .any(|c| c.color == cube_count.color && c.count >= cube_count.count)
    }
}

fn part1(games: &[Game]) -> u32 {
    let config = CubeConfiguration {
        cubes: vec![
            CubeCount {
                count: 12,
                color: Color::Red,
            },
            CubeCount {
                count: 13,
                color: Color::Green,
            },
            CubeCount {
                count: 14,
                color: Color::Blue,
            },
        ],
    };
    games
        .iter()
        .filter(|g| {
            g.rounds
                .iter()
                .flat_map(|r| &r.color_counts)
                .all(|c| config.is_possible(c))
        })
        .map(|g| g.id)
        .sum()
}

fn calculate_minimum_cubes(game: &Game) -> Vec<CubeCount> {
    let res = game.rounds.iter().flat_map(|r| &r.color_counts).fold(
        (CubeCount::red(), CubeCount::green(), CubeCount::blue()),
        |(r, g, b), cube_count| match cube_count.color {
            Color::Red => (
                CubeCount {
                    count: r.count.max(cube_count.count),
                    color: Color::Red,
                },
                g,
                b,
            ),
            Color::Green => (
                r,
                CubeCount {
                    count: g.count.max(cube_count.count),
                    color: Color::Green,
                },
                b,
            ),
            Color::Blue => (
                r,
                g,
                CubeCount {
                    count: b.count.max(cube_count.count),
                    color: Color::Blue,
                },
            ),
        },
    );
    vec![res.0, res.1, res.2]
}

fn power_value(cube_counts: &[CubeCount]) -> u32 {
    cube_counts.iter().map(|c| c.count).product()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| calculate_minimum_cubes(g))
        .map(|mc| power_value(&mc))
        .sum()
}

fn main() -> Result<()> {
    let games = read_lines("input/day02.txt")?;
    let result = part1(&games);
    println!("part 1: {}", result);
    let result = part2(&games);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let games: Vec<Game> = read_lines("input/test/day02.txt")?;
    let result = part1(&games);
    assert_eq!(result, 8);
    let result = part2(&games);
    assert_eq!(result, 2286);
    Ok(())
}
