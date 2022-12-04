use anyhow::Result;
use parse_display::FromStr;
use utils::read_lines;

#[derive(FromStr, PartialEq, Debug, Clone, Copy)]
enum Throw {
    #[from_str(regex = "[AX]")]
    Rock,
    #[from_str(regex = "[BY]")]
    Paper,
    #[from_str(regex = "[CZ]")]
    Scissors,
}

#[derive(FromStr, PartialEq, Debug, Clone, Copy)]
enum Outcome {
    #[display("X")]
    Lose,
    #[display("Y")]
    Draw,
    #[display("Z")]
    Win,
}

#[derive(FromStr, PartialEq, Debug)]
#[display("{opponent} {me}")]
struct Round {
    opponent: Throw,
    me: Throw,
}

#[derive(FromStr, PartialEq, Debug)]
#[display("{0} {1}")]
struct RoundStrategy(Throw, Outcome);

impl Throw {
    fn score(&self) -> u32 {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }
}

impl Round {
    fn score(&self) -> u32 {
        self.me.score()
            + match (self.opponent, self.me) {
                (Throw::Rock, Throw::Paper) => 6,
                (Throw::Rock, Throw::Scissors) => 0,
                (Throw::Paper, Throw::Rock) => 0,
                (Throw::Paper, Throw::Scissors) => 6,
                (Throw::Scissors, Throw::Rock) => 6,
                (Throw::Scissors, Throw::Paper) => 0,
                _ => 3,
            }
    }
}

impl RoundStrategy {
    fn to_round(&self) -> Round {
        let to_throw = match self {
            RoundStrategy(_, Outcome::Draw) => self.0,
            RoundStrategy(Throw::Rock, Outcome::Lose) => Throw::Scissors,
            RoundStrategy(Throw::Rock, Outcome::Win) => Throw::Paper,
            RoundStrategy(Throw::Paper, Outcome::Lose) => Throw::Rock,
            RoundStrategy(Throw::Paper, Outcome::Win) => Throw::Scissors,
            RoundStrategy(Throw::Scissors, Outcome::Lose) => Throw::Paper,
            RoundStrategy(Throw::Scissors, Outcome::Win) => Throw::Rock,
        };
        Round {
            opponent: self.0,
            me: to_throw,
        }
    }
}

fn part1(rounds: &[Round]) -> u32 {
    rounds.iter().map(|r| r.score()).sum()
}

fn part2(round_strats: &[RoundStrategy]) -> u32 {
    round_strats.iter().map(|r| r.to_round().score()).sum()
}

fn main() -> Result<()> {
    let rounds = read_lines("input/day02.txt")?;
    let result = part1(&rounds);
    println!("part 1: {}", result);
    let rounds_strats = read_lines("input/day02.txt")?;
    let result = part2(&rounds_strats);
    println!("part 2: {}", result);
    Ok(())
}

#[test]
fn test() -> Result<()> {
    let rounds = read_lines("input/test/day02.txt")?;
    let result = part1(&rounds);
    assert_eq!(result, 15);

    let rounds_strats = read_lines("input/test/day02.txt")?;
    let result = part2(&rounds_strats);
    assert_eq!(result, 12);

    Ok(())
}
