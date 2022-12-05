use std::collections::VecDeque;

use crate::day5pt1::{crate_top_string, read_crates, read_moves};

pub fn perform_move(
    crates: Vec<VecDeque<char>>,
    move_instruction: (usize, usize, usize),
) -> Vec<VecDeque<char>> {
    let (count, from, to) = move_instruction;
    let mut from_crate = crates[from - 1].clone();
    let mut to_crate = crates[to - 1].clone();

    let mut to_move: Vec<char> = vec![];

    for _ in 0..count {
        let value = from_crate.pop_front().expect("crate contents");
        to_move.push(value);
    }

    to_move.reverse();

    for item in to_move {
        to_crate.push_front(item)
    }

    let mut new = vec![];

    for (index, orig_crate) in crates.into_iter().enumerate() {
        if index != (from - 1) && index != (to - 1) {
            new.push(orig_crate);
        } else if index == (from - 1) {
            new.push(from_crate.clone())
        } else if index == (to - 1) {
            new.push(to_crate.clone())
        }
    }

    new
}

pub fn solve(input: &str) -> String {
    let mut crates = read_crates(input);
    let move_instructions = read_moves(input);

    for move_instruction in move_instructions {
        crates = perform_move(crates, move_instruction);
    }

    crate_top_string(crates)
}
