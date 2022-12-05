use std::collections::HashSet;

pub fn left_right_sets_from_line(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    let (left, right) = line.split_once(',').expect("split section assignment");
    let (left_start, left_end): (u32, u32) = left
        .split_once('-')
        .map(|(left, right)| {
            (
                left.parse().expect("parse min"),
                right.parse().expect("parse max"),
            )
        })
        .expect("split section range");
    let (right_start, right_end): (u32, u32) = right
        .split_once('-')
        .map(|(left, right)| {
            (
                left.parse().expect("parse min"),
                right.parse().expect("parse max"),
            )
        })
        .expect("split section range");

    let left: HashSet<u32> = HashSet::from_iter(left_start..=left_end);
    let right: HashSet<u32> = HashSet::from_iter(right_start..=right_end);

    (left, right)
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = left_right_sets_from_line(line);

            usize::from(left.is_superset(&right) || left.is_subset(&right))
        })
        .sum()
}
