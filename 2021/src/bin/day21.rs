#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::convert::TryInto;

use anyhow::Result;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use utils::read_lines;

#[derive(parse_display::FromStr, PartialEq, Debug, Clone, Copy)]
#[display("Player {number} starting position: {position}")]
struct Player {
    number: u8,
    position: u32,
}

fn part1(player1: &Player, player2: &Player) -> u32 {
    let max_score: u32 = 1000;
    let mut positions = (player1.position, player2.position);
    let (count, (p1, p2)) = (0..)
        .map(|n| n % 100 + 1)
        .chunks(3)
        .into_iter()
        .map(|c| c.sum())
        .chunks(2)
        .into_iter()
        .fold_while((0, (0, 0)), |(mut roll_count, scores), mut rolls| {
            if scores.0 >= max_score || scores.1 >= max_score {
                Done((roll_count, scores))
            } else {
                let p1_position = (positions.0 + rolls.next().unwrap_or(0) - 1) % 10 + 1;
                let p1_score: u32 = scores.0 + p1_position;
                let p2_position = (positions.1 + rolls.next().unwrap_or(0) - 1) % 10 + 1;
                positions = (p1_position, p2_position);
                roll_count += 3;
                let p2_score: u32 = if p1_score < 1000 {
                    roll_count += 3;
                    scores.1 + p2_position
                } else {
                    scores.1
                };
                Continue((roll_count, (p1_score, p2_score)))
            }
        })
        .into_inner();
    p1.min(p2) * count
}

fn part2(player1: &Player, player2: &Player) -> usize {
    fn count_ways(target: u8, position: u8) -> HashMap<u8, usize> {
        lazy_static! {
            // number of ways to roll a particular number
            static ref MULTI_MAP: HashMap<u8, usize> = (0..3)
                .map(|_| 1..=3u8)
                .multi_cartesian_product()
                .map(|v| v.iter().sum::<u8>())
                .counts();
        }
        if target == 0 {
            // Exit condition - one way to get 0 points: you're done!
            return HashMap::from([(0, 1)]);
        }
        let mut result = HashMap::new();
        for (&roll, ways) in MULTI_MAP.iter() {
            let next_pos = (position + roll - 1) % 10 + 1;
            let next_ways = count_ways(target.saturating_sub(next_pos), next_pos);
            next_ways.iter().for_each(|(way_cnt, paths)| {
                // Add a turn to every count received, and however many paths there were,
                // the number of ways to roll the current number is the new number of paths.
                *result.entry(way_cnt + 1).or_insert(0) += paths * ways;
            })
        }
        result
    }
    let paths_p1 = count_ways(21, player1.position.try_into().unwrap());
    let paths_p2 = count_ways(21, player2.position.try_into().unwrap());

    let mut p1_count = 0;
    let mut p2_count = 0;
    let number_of_ways: usize = 27;

    for &t in paths_p1.keys() {
        // How many paths to get to turn t for player 1?
        let p1_paths_to_t = paths_p1[&t];
        // How many ways for player 2 to get to the same point?
        let number_of_p2_possible_rolls = number_of_ways.pow((t - 1) as u32);
        // But how many of those ways did player 2 already win?
        let number_of_rolls_p2_already_won: usize = paths_p2
            .iter()
            .filter(|&(turn, _)| turn < &t)
            // For rolls that ended a few turns ago, we need to scale them up to kill all future universes
            // e.g. if it's turn 5 and we're looking at rolls player 2 won with on turn 3,
            // every possible universe after that needs to get killed, which 27 universes per path.
            .map(|(turn, rolls)| rolls * number_of_ways.pow(((t - 1) - turn) as u32))
            .sum();
        let p1_won = p1_paths_to_t * (number_of_p2_possible_rolls - number_of_rolls_p2_already_won);

        // Repeat the above for player 2, but player 1 always rolls more dice first, so use t instead of t-1.
        let p2_paths_to_t = paths_p2[&t];
        let number_of_p1_possible_rolls = number_of_ways.pow(t as u32);
        let number_of_rolls_p1_already_won: usize = paths_p1
            .iter()
            .filter(|&(turn, _)| turn <= &t)
            .map(|(turn, rolls)| rolls * number_of_ways.pow((t - turn) as u32))
            .sum();
        let p2_won = p2_paths_to_t * (number_of_p1_possible_rolls - number_of_rolls_p1_already_won);

        p1_count += p1_won;
        p2_count += p2_won;
    }

    p1_count.max(p2_count)
}

fn main() -> Result<()> {
    let players = read_lines("input/day21.txt")?;
    let result = part1(&players[0], &players[1]);
    println!("part 1: {}", result);
    let result = part2(&players[0], &players[1]);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let players = read_lines("input/test/day21.txt")?;
    let result = part1(&players[0], &players[1]);
    assert_eq!(result, 739785);

    let result = part2(&players[0], &players[1]);
    assert_eq!(result, 444356092776315);

    Ok(())
}
