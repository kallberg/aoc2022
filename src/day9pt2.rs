use std::iter::repeat;

use crate::day9pt1::{
    display_area, parse_directions, BoundingBox, Direction, Position, RopeSimulation,
};

pub fn solve(input: &str) -> usize {
    let moves: Vec<Direction> = parse_directions(input);

    let mut sim = RopeSimulation::default();
    sim.parts
        .append(&mut repeat(Position::default()).take(9).collect());

    sim.mark_tail();

    for instruction in moves {
        sim.perform_move(instruction);
        println!();
    }

    sim.tail_markers.len()
}
