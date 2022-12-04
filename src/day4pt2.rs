use crate::day4pt1::left_right_sets_from_line;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = left_right_sets_from_line(line);

            usize::from(!left.is_disjoint(&right))
        })
        .sum()
}
