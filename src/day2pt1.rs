use std::str::FromStr;

#[derive(Clone)]
pub enum Kind {
    Rock,
    Paper,
    Scissiors,
}

impl FromStr for Kind {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" => Ok(Kind::Rock),
            "B" => Ok(Kind::Paper),
            "C" => Ok(Kind::Scissiors),
            "X" => Ok(Kind::Rock),
            "Y" => Ok(Kind::Paper),
            "Z" => Ok(Kind::Scissiors),
            _ => Err("bad instruction"),
        }
    }
}

pub fn score(a: Kind, b: Kind) -> u64 {
    match (a, b) {
        (Kind::Rock, Kind::Rock) => 3 + 1,
        (Kind::Rock, Kind::Paper) => 1,
        (Kind::Rock, Kind::Scissiors) => 6 + 1,
        (Kind::Paper, Kind::Rock) => 6 + 2,
        (Kind::Paper, Kind::Paper) => 3 + 2,
        (Kind::Paper, Kind::Scissiors) => 2,
        (Kind::Scissiors, Kind::Rock) => 3,
        (Kind::Scissiors, Kind::Paper) => 6 + 3,
        (Kind::Scissiors, Kind::Scissiors) => 3 + 3,
    }
}

pub fn solve(input: &str) -> u64 {
    let scores: Vec<u64> = input
        .lines()
        .map(|line| {
            let parsed = line.split_once(' ').and_then(|(a_str, b_str)| {
                let a: Kind = a_str.to_string().parse().ok()?;
                let b: Kind = b_str.to_string().parse().ok()?;
                Some((a, b))
            });

            parsed.expect("parse line")
        })
        .map(|(a, b)| score(b, a))
        .collect();

    scores.iter().sum()
}
