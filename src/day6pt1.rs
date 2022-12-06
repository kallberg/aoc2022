use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let found = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .map(|chars| {
            let mut set = HashSet::<char>::new();

            for char in chars {
                set.insert(*char);
            }

            println!("{}", set.len());

            set
        })
        .enumerate()
        .find(|(_, set)| set.len() == 4)
        .expect("solution");

    found.0 + 4
}
