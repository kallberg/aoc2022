use std::ops::Deref;

pub type Coord = (usize, usize);

#[derive(Debug)]
pub struct HeightMap(Vec<Vec<usize>>);

impl Deref for HeightMap {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for HeightMap {
    fn from(input: &str) -> Self {
        let mut data = vec![];

        for line in input.lines() {
            let mut row = vec![];
            for char in line.chars() {
                let h: usize = char.to_string().parse().expect("parse height");

                row.push(h);
            }

            data.push(row);
        }

        Self(data)
    }
}

impl HeightMap {
    pub fn size(&self) -> Coord {
        let data = &self.0;

        let height: usize = data.len();
        let width: usize = data.get(0).expect("one row to check width with").len();

        (width, height)
    }

    pub fn trees(&self, coords: Vec<Coord>) -> Vec<usize> {
        let mut output = vec![];

        for (x, y) in coords {
            let tree = self.0[y][x];
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
    let from_right = ((tree_x + 1)..width).zip(std::iter::repeat(tree_y));
    let from_top = (std::iter::repeat(tree_x)).zip(0..tree_y);
    let from_bottom = (std::iter::repeat(tree_x)).zip((tree_y + 1)..height);

    let tree_height = scan[tree_y][tree_x];

    visible_compared_to(tree_height, scan.trees(from_left.collect()))
        || visible_compared_to(tree_height, scan.trees(from_right.collect()))
        || visible_compared_to(tree_height, scan.trees(from_bottom.collect()))
        || visible_compared_to(tree_height, scan.trees(from_top.collect()))
}

pub fn solve(input: &str) -> usize {
    let scan = HeightMap::from(input);

    let mut visible: usize = 0;
    let size = scan.size();

    for x in 0..size.0 {
        for y in 0..size.1 {
            if tree_is_visible(&scan, (x, y)) {
                visible += 1
            }
        }
    }

    visible
}
