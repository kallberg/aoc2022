use std::ops::Deref;

pub type Coord = (usize, usize);

#[derive(Debug)]
pub struct TreeGrid {
    pub width: usize,
    pub height: usize,
    pub inner: Vec<Vec<usize>>,
}

impl Deref for TreeGrid {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<&str> for TreeGrid {
    fn from(input: &str) -> Self {
        let mut data = vec![];

        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            if width < line.len() {
                width = line.len();
            }

            let mut row = vec![];
            for char in line.chars() {
                let h: usize = char.to_string().parse().expect("parse height");

                row.push(h);
            }

            data.push(row);
        }

        Self {
            inner: data,
            width,
            height,
        }
    }
}

impl TreeGrid {
    pub fn trees(&self, coords: Vec<Coord>) -> Vec<usize> {
        let mut output = vec![];

        for (x, y) in coords {
            let tree = self.inner[y][x];
            output.push(tree);
        }

        output
    }

    pub fn visible(&self, coord: Coord) -> bool {
        let (tree_x, tree_y) = coord;
        let width = self.width;
        let height = self.height;

        let mut from_left = (0..tree_x).zip(std::iter::repeat(tree_y));
        let mut from_right = ((tree_x + 1)..width).zip(std::iter::repeat(tree_y));
        let mut from_top = (std::iter::repeat(tree_x)).zip(0..tree_y);
        let mut from_bottom = (std::iter::repeat(tree_x)).zip((tree_y + 1)..height);

        let tree_height = self[tree_y][tree_x];

        !(from_left.any(|(x, y)| self[y][x] >= tree_height)
            && from_right.any(|(x, y)| self[y][x] >= tree_height)
            && from_top.any(|(x, y)| self[y][x] >= tree_height)
            && from_bottom.any(|(x, y)| self[y][x] >= tree_height))
    }
}

pub fn solve(input: &str) -> usize {
    let scan = TreeGrid::from(input);
    let width = scan.width;
    let height = scan.height;

    let mut visible: usize = 0;

    for y in 0..width {
        for x in 0..height {
            if scan.visible((x, y)) {
                visible += 1;
            }
        }
    }

    visible
}
