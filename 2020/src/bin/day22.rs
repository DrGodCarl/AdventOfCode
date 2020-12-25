use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use anyhow::{bail, Result};
use utils::{read_chunks, InputParseError};

#[derive(PartialEq, Eq, Hash)]
enum Player {
    One,
    Two,
}

struct GameOfCombat {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

struct Wrapper(VecDeque<usize>);

impl FromStr for Wrapper {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deck = s
            .split('\n')
            .skip(1)
            .map(|n| n.parse::<usize>())
            .collect::<Result<VecDeque<_>, _>>()
            .map_err(|_| InputParseError)
            .map(Wrapper)?;
        Ok(deck)
    }
}

impl GameOfCombat {
    fn new(players: Vec<VecDeque<usize>>) -> Result<Self> {
        if players.len() != 2 {
            bail!("Incorrect number of players")
        }
        Ok(GameOfCombat {
            player1: players[0].clone(),
            player2: players[1].clone(),
        })
    }

    fn play(&mut self) {
        while !self.player1.is_empty() && !self.player2.is_empty() {
            let play1 = self.player1.pop_front().unwrap();
            let play2 = self.player2.pop_front().unwrap();
            match play1 > play2 {
                true => {
                    self.player1.push_back(play1);
                    self.player1.push_back(play2);
                }
                false => {
                    self.player2.push_back(play2);
                    self.player2.push_back(play1);
                }
            }
        }
    }

    fn winner(&self) -> Option<&VecDeque<usize>> {
        if !self.player1.is_empty() && !self.player2.is_empty() {
            None
        } else if self.player1.is_empty() {
            Some(&self.player2)
        } else {
            Some(&self.player1)
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct PlayerState {
    player: Player,
    state: VecDeque<usize>,
}

struct GameOfRecursiveCombat {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
    previous_states: HashSet<PlayerState>,
}

impl GameOfRecursiveCombat {
    fn new(players: Vec<VecDeque<usize>>) -> Result<Self> {
        if players.len() != 2 {
            bail!("Incorrect number of players")
        }
        Ok(GameOfRecursiveCombat {
            player1: players[0].clone(),
            player2: players[1].clone(),
            previous_states: HashSet::new(),
        })
    }

    fn track_state(&mut self) -> bool {
        self.previous_states.insert(PlayerState {
            player: Player::One,
            state: self.player1.clone(),
        }) && self.previous_states.insert(PlayerState {
            player: Player::Two,
            state: self.player2.clone(),
        })
    }

    fn play(&mut self) -> Result<Player> {
        while !self.player1.is_empty() && !self.player2.is_empty() {
            if !self.track_state() {
                return Ok(Player::One);
            }
            let play1 = self.player1.pop_front().unwrap();
            let play2 = self.player2.pop_front().unwrap();
            let hand_winner: Player;
            if play1 <= self.player1.len() && play2 <= self.player2.len() {
                hand_winner = GameOfRecursiveCombat::new(vec![
                    self.player1.iter().take(play1).copied().collect(),
                    self.player2.iter().take(play2).copied().collect(),
                ])?
                .play()?;
            } else {
                hand_winner = match play1 > play2 {
                    true => Player::One,
                    false => Player::Two,
                };
            }
            match hand_winner {
                Player::One => {
                    self.player1.push_back(play1);
                    self.player1.push_back(play2);
                }
                Player::Two => {
                    self.player2.push_back(play2);
                    self.player2.push_back(play1);
                }
            }
        }

        Ok(if self.player1.is_empty() {
            Player::Two
        } else {
            Player::One
        })
    }

    fn get_deck_for_player(&self, player: &Player) -> &VecDeque<usize> {
        match player {
            Player::One => &self.player1,
            Player::Two => &self.player2,
        }
    }
}

fn part1(mut game: GameOfCombat) -> Result<usize> {
    game.play();
    match game.winner() {
        Some(deck) => Ok(deck
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, &val)| (idx + 1) * val)
            .sum()),
        None => bail!("No one won? I guess?"),
    }
}

fn part2(mut game: GameOfRecursiveCombat) -> Result<usize> {
    let winner = game.play()?;
    Ok(game
        .get_deck_for_player(&winner)
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, &val)| (idx + 1) * val)
        .sum())
}

fn main() -> Result<()> {
    let players = read_chunks::<Wrapper>("input/day22.txt")?
        .iter()
        .map(|w| w.0.clone())
        .collect();
    let game = GameOfCombat::new(players)?;
    let result = part1(game)?;
    println!("part 1: {}", result);

    let players = read_chunks::<Wrapper>("input/day22.txt")?
        .iter()
        .map(|w| w.0.clone())
        .collect();
    let game = GameOfRecursiveCombat::new(players)?;
    let result = part2(game)?;
    println!("part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let players = read_chunks::<Wrapper>("input/test/day22.txt")?
            .iter()
            .map(|w| w.0.clone())
            .collect();
        let game = GameOfCombat::new(players)?;
        let result = part1(game)?;
        assert_eq!(result, 306);
        Ok(())
    }

    #[test]
    fn test_recursive_for_loops() -> Result<()> {
        let players = read_chunks::<Wrapper>("input/test/day22_inf.txt")?
            .iter()
            .map(|w| w.0.clone())
            .collect();
        let mut game = GameOfRecursiveCombat::new(players)?;
        game.play()?; // not a great test. it won't terminate if it failed.
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let players = read_chunks::<Wrapper>("input/test/day22.txt")?
            .iter()
            .map(|w| w.0.clone())
            .collect();
        let game = GameOfRecursiveCombat::new(players)?;
        let result = part2(game)?;
        assert_eq!(result, 291);
        Ok(())
    }
}
