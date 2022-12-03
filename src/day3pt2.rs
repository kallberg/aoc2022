use std::{collections::HashSet, str::Lines};

use crate::day3pt1::priority;

type CharSet = HashSet<char>;

pub fn parse_charset(line: &str) -> CharSet {
    HashSet::from_iter(line.chars())
}

pub fn parse_backpack_group(input: &mut Lines) -> Option<(CharSet, CharSet, CharSet)> {
    let l1 = input.next()?;
    let l2 = input.next()?;
    let l3 = input.next()?;

    Some((parse_charset(l1), parse_charset(l2), parse_charset(l3)))
}

pub fn common_char(input: &mut Lines) -> Option<char> {
    let (a, b, c) = parse_backpack_group(input)?;

    a.iter()
        .filter(|k| b.contains(k))
        .find(|k| c.contains(k))
        .copied()
}

pub fn try_solve(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut sum: u32 = 0;

    while let Some(group_common) = common_char(&mut lines) {
        if let Some(score) = priority(&group_common) {
            sum += score;
        }
    }

    Some(sum)
}
