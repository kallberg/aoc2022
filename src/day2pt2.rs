use crate::day2pt1::{Kind, Match};

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

pub fn solve(input: &str) -> String {
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
