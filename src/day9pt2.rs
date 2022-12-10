use crate::day9pt1::{parse_directions, RopeSimulation};

pub fn solve(input: &str) -> usize {
    RopeSimulation::new(10).perform_moves(parse_directions(input))
}
