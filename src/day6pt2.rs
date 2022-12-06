use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let found = input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .map(|chars| {
            let mut set = HashSet::<char>::new();

            for char in chars {
                set.insert(*char);
            }

            set
        })
        .enumerate()
        .find(|(_, set)| set.len() == 14)
        .expect("solution");

    found.0 + 14
}
