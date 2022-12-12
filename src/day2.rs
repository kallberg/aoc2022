#[derive(Clone)]
pub enum Kind {
    Rock,
    Paper,
    Scissiors,
}

#[derive(Clone)]
pub struct Match {
    pub player_one: Kind,
    pub player_two: Kind,
}

impl From<&str> for Kind {
    fn from(value: &str) -> Self {
        match value {
            "A" => Kind::Rock,
            "B" => Kind::Paper,
            "C" => Kind::Scissiors,
            "X" => Kind::Rock,
            "Y" => Kind::Paper,
            "Z" => Kind::Scissiors,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Match {
    fn from(line: &str) -> Self {
        let pair = line
            .split_once(' ')
            .map(|(a_str, b_str)| (Kind::from(a_str), Kind::from(b_str)))
            .expect("split line");

        Self {
            player_one: pair.1,
            player_two: pair.0,
        }
    }
}

impl Match {
    pub fn score(&self) -> u64 {
        match (&self.player_one, &self.player_two) {
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
}

pub fn solve_1(input: &str) -> String {
    input
        .lines()
        .map(Match::from)
        .map(|m| m.score())
        .sum::<u64>()
        .to_string()
}

pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl From<&Kind> for Outcome {
    fn from(value: &Kind) -> Self {
        match value {
            Kind::Rock => Outcome::Loss,
            Kind::Paper => Outcome::Draw,
            Kind::Scissiors => Outcome::Win,
        }
    }
}

impl Kind {
    pub fn move_with_outcome(&self, outcome: Outcome) -> Kind {
        match (self, outcome) {
            (Kind::Rock, Outcome::Win) => Kind::Paper,
            (Kind::Rock, Outcome::Loss) => Kind::Scissiors,
            (Kind::Paper, Outcome::Win) => Kind::Scissiors,
            (Kind::Paper, Outcome::Loss) => Kind::Rock,
            (Kind::Scissiors, Outcome::Win) => Kind::Rock,
            (Kind::Scissiors, Outcome::Loss) => Kind::Paper,
            (x, Outcome::Draw) => x.clone(),
        }
    }
}

pub fn solve_2(input: &str) -> String {
    input
        .lines()
        .map(Match::from)
        .map(|m| {
            let mut adjusted = m.clone();
            let instruction = Outcome::from(&m.player_one);
            let new_move = m.player_two.move_with_outcome(instruction);
            adjusted.player_one = new_move;
            adjusted.score()
        })
        .sum::<u64>()
        .to_string()
}
