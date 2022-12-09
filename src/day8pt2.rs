use crate::day8pt1::{Coord, TreeGrid};

pub fn scenic_score(scan: &TreeGrid, tree: Coord) -> usize {
    let (tree_x, tree_y) = tree;
    let width = scan.width;
    let height = scan.height;
    let tree_height = scan[tree_y][tree_x];

    if tree_x.eq(&0) || tree_y.eq(&0) {
        return 0;
    }

    let from_left = ((0..tree_x).rev()).zip(std::iter::repeat(tree_y)).collect();
    let from_right = ((tree_x + 1)..width)
        .zip(std::iter::repeat(tree_y))
        .collect();
    let from_top = (std::iter::repeat(tree_x))
        .zip((0..(tree_y)).rev())
        .collect();
    let from_bottom = (std::iter::repeat(tree_x))
        .zip((tree_y + 1)..height)
        .collect();

    let mut scores = vec![];
    let mut score = 0;

    for tree in scan.trees(from_top) {
        score += 1;
        if tree_height <= tree {
            break;
        }
    }

    scores.push(score);
    score = 0;

    for tree in scan.trees(from_left) {
        score += 1;
        if tree_height <= tree {
            break;
        }
    }

    scores.push(score);
    score = 0;

    for tree in scan.trees(from_right) {
        score += 1;
        if tree_height <= tree {
            break;
        }
    }

    scores.push(score);
    score = 0;

    for tree in scan.trees(from_bottom) {
        score += 1;
        if tree_height <= tree {
            break;
        }
    }

    scores.push(score);

    scores.iter().product()
}

pub fn solve(input: &str) -> usize {
    let scan = TreeGrid::from(input);

    let mut max = 0;

    let width = scan.width;
    let height = scan.height;

    for x in 0..width {
        for y in 0..height {
            let score = scenic_score(&scan, (x, y));

            if score > max {
                max = score;
            }
        }
    }

    max
}
