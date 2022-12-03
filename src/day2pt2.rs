use std::str::FromStr;

use crate::day2pt1::{score, Kind};

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("unexpected instruction"),
        }
    }
}

fn outcome_move(oponent: Kind, outcome: Outcome) -> Kind {
    match (oponent, outcome) {
        (Kind::Rock, Outcome::Win) => Kind::Paper,
        (Kind::Rock, Outcome::Loss) => Kind::Scissiors,
        (Kind::Paper, Outcome::Win) => Kind::Scissiors,
        (Kind::Paper, Outcome::Loss) => Kind::Rock,
        (Kind::Scissiors, Outcome::Win) => Kind::Rock,
        (Kind::Scissiors, Outcome::Loss) => Kind::Paper,
        (x, Outcome::Draw) => x,
    }
}

pub fn solve(input: &str) -> u64 {
    let moves = input
        .lines()
        .map(|line| {
            let parsed = line.split_once(' ').and_then(|(a_str, b_str)| {
                let a: Kind = a_str.to_string().parse().ok()?;
                let b: Outcome = b_str.to_string().parse().ok()?;
                Some((a, b))
            });

            parsed.expect("parse line")
        })
        .map(|(a, b)| (a.clone(), outcome_move(a, b)));

    moves.map(|(a, b)| score(b, a)).sum()
}
