use crate::day9pt1::{parse_directions, RopeSimulation};

pub fn solve(input: &str) -> String {
    RopeSimulation::new(10)
        .perform_moves(parse_directions(input))
        .to_string()
}
