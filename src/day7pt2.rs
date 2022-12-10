use std::collections::HashMap;

use crate::day7pt1::Directory;

pub fn find_smallest_needed_delete(
    root: Directory,
    total_space: usize,
    needed_space: usize,
) -> usize {
    let mut cache = HashMap::new();

    let used_space = root.size(&mut cache);
    let free_space = total_space - used_space;
    let target = needed_space - free_space;

    let mut candidates = root.descendants();

    // Anything goes!
    candidates.push(root);

    let best = candidates
        .iter()
        .map(|directory| directory.size(&mut cache))
        .filter(|size| *size >= target)
        .min();

    // We only accept the best
    best.expect("the best")
}

pub fn solve(input: &str) -> String {
    find_smallest_needed_delete(Directory::from(input), 70000000, 30000000).to_string()
}
