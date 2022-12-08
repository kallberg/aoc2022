use std::{collections::HashMap, ops::Deref};

pub type Coord = (usize, usize);

#[derive(Debug)]
pub struct HeightMap(HashMap<Coord, usize>);

impl Deref for HeightMap {
    type Target = HashMap<Coord, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        let mut data = HashMap::<Coord, usize>::new();

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let h: usize = char.to_string().parse().expect("parse height");

                data.insert((x, y), h);
            }
        }

        Self(data)
    }
}

impl HeightMap {
    pub fn size(&self) -> Coord {
        let data = &self.0;

        let height: usize = *data.keys().map(|(_, y)| y).max().expect("height");
        let width: usize = *data.keys().map(|(x, _)| x).max().expect("width");

        (width, height)
    }

    pub fn trees(&self, coords: Vec<Coord>) -> Vec<usize> {
        let mut output = vec![];

        for coord in coords {
            let tree = *self.0.get(&coord).expect("tree at coord");
            output.push(tree);
        }

        output
    }
}

pub fn visible_compared_to(tree_height: usize, others: Vec<usize>) -> bool {
    for other in others {
        if tree_height <= other {
            return false;
        }
    }

    true
}

pub fn tree_is_visible(scan: &HeightMap, tree: Coord) -> bool {
    let (tree_x, tree_y) = tree;
    let (width, height) = scan.size();

    let from_left = (0..tree_x).zip(std::iter::repeat(tree_y));
    let from_right = ((tree_x + 1)..=width).zip(std::iter::repeat(tree_y));
    let from_top = (std::iter::repeat(tree_x)).zip(0..tree_y);
    let from_bottom = (std::iter::repeat(tree_x)).zip((tree_y + 1)..=height);

    let tree_height = *scan.get(&tree).expect("tree");

    visible_compared_to(tree_height, scan.trees(from_left.collect()))
        || visible_compared_to(tree_height, scan.trees(from_right.collect()))
        || visible_compared_to(tree_height, scan.trees(from_bottom.collect()))
        || visible_compared_to(tree_height, scan.trees(from_top.collect()))
}

pub fn solve(input: &str) -> usize {
    let scan = HeightMap::from(input);

    let mut visible: usize = 0;

    for coord in scan.keys() {
        if tree_is_visible(&scan, *coord) {
            visible += 1
        }
    }

    visible
}
