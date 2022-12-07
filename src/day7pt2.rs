use std::{collections::HashMap, path::PathBuf};

use crate::day7pt1::{listing_to_directory, parse_listings, Directory};

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

pub fn solve(input: &str) -> usize {
    let listings = parse_listings(input);
    let root_listing = listings.get(&PathBuf::from("/")).unwrap();
    let root = listing_to_directory(root_listing, &listings);

    find_smallest_needed_delete(root, 70000000, 30000000)
}
