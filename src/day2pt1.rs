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

pub fn try_solve(input: &str) -> Option<u64> {
    let matches = input
        .lines()
        .map(|line| {
            line.split_once(' ').and_then(|(a_str, b_str)| {
                let a: Kind = a_str.to_string().parse().ok()?;
                let b: Kind = b_str.to_string().parse().ok()?;
                Some((a, b))
            })
        })
        .collect::<Option<Vec<(Kind, Kind)>>>()?;

    let scores: Vec<u64> = matches.into_iter().map(|(a, b)| score(b, a)).collect();

    Some(scores.iter().sum())
}
