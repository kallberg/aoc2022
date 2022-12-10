use std::collections::HashSet;

fn split_rucksack_line(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_error(left: &str, right: &str) -> char {
    let l_set: HashSet<char> = HashSet::from_iter(left.chars());
    let r_set: HashSet<char> = HashSet::from_iter(right.chars());

    let err = l_set.intersection(&r_set).into_iter().next();

    err.cloned().expect("find backpack error")
}

pub fn priority(char: &char) -> u32 {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .find(|(_, reference)| char == reference)
        .map(|(i, _)| i as u32 + 1)
        .expect("find item priority")
}

pub fn solve(input: &str) -> String {
    let errors = input.lines().map(|line| {
        let (l, r) = split_rucksack_line(line);
        find_error(l, r)
    });

    let priorities = errors.map(|error| priority(&error));

    priorities.sum::<u32>().to_string()
}
