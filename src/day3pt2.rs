use std::collections::HashSet;

use crate::day3pt1::priority;

pub fn solve(input: &str) -> String {
    let charsets: Vec<HashSet<char>> = input
        .lines()
        .map(|line| HashSet::from_iter(line.chars()))
        .collect();

    let groups = charsets.chunks_exact(3);

    groups
        .map(|groups| match groups {
            [one, two, three] => priority(
                one.iter()
                    .filter(|k| two.contains(k))
                    .find(|k| three.contains(k))
                    .expect("common group"),
            ),
            _ => unreachable!(),
        })
        .sum::<u32>()
        .to_string()
}
