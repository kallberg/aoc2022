use std::collections::HashSet;

fn split_rucksack_line(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn find_error(left: &str, right: &str) -> Option<char> {
    let l_set: HashSet<char> = HashSet::from_iter(left.chars());
    let r_set: HashSet<char> = HashSet::from_iter(right.chars());

    let err = l_set.intersection(&r_set).into_iter().next();

    err.cloned()
}

pub fn priority(char: &char) -> Option<u32> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .find(|(_, reference)| char == reference)
        .map(|(i, _)| i as u32 + 1)
}

pub fn try_solve(input: &str) -> Option<u32> {
    let errors: Vec<char> = input
        .lines()
        .map(|line| {
            let (l, r) = split_rucksack_line(line);
            find_error(l, r)
        })
        .collect::<Option<Vec<char>>>()?;

    let priorities: Vec<u32> = errors
        .into_iter()
        .map(|error| priority(&error))
        .collect::<Option<Vec<u32>>>()?;

    Some(priorities.iter().sum())
}
