use std::ops::{Deref, Mul};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

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

        for coord in coords {
            let tree = self.inner[coord.y][coord.x];
            output.push(tree);
        }

        output
    }

    pub fn visible(&self, coord: Coord) -> bool {
        let tree_x = coord.x;
        let tree_y = coord.y;
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

pub fn solve_1(input: &str) -> String {
    let scan = TreeGrid::from(input);
    let width = scan.width;
    let height = scan.height;

    let mut visible: usize = 0;

    for y in 0..width {
        for x in 0..height {
            if scan.visible(Coord { x, y }) {
                visible += 1;
            }
        }
    }

    visible.to_string()
}

pub enum ViewDirection {
    L,
    R,
    U,
    D,
}

impl TreeGrid {
    pub fn view_distanece(&self, position: &Coord, direction: ViewDirection) -> usize {
        if position.x.eq(&0)
            || position.y.eq(&0)
            || (position.x + 1).ge(&self.width)
            || (position.y + 1).ge(&self.height)
        {
            return 0;
        }

        let check = match direction {
            ViewDirection::L => ((0..position.x).rev())
                .zip(std::iter::repeat(position.y))
                .map(|(x, y)| Coord { x, y })
                .collect(),
            ViewDirection::R => ((position.x + 1)..self.width)
                .zip(std::iter::repeat(position.y))
                .map(|(x, y)| Coord { x, y })
                .collect(),
            ViewDirection::U => (std::iter::repeat(position.x))
                .zip((0..(position.y)).rev())
                .map(|(x, y)| Coord { x, y })
                .collect(),
            ViewDirection::D => (std::iter::repeat(position.x))
                .zip((position.y + 1)..self.height)
                .map(|(x, y)| Coord { x, y })
                .collect(),
        };

        let mut score = 0;

        for tree in self.trees(check) {
            score += 1;
            if self[position.y][position.x] <= tree {
                break;
            }
        }

        score
    }

    pub fn scenic_score(&self, position: &Coord) -> usize {
        self.view_distanece(position, ViewDirection::L)
            .mul(self.view_distanece(position, ViewDirection::R))
            .mul(self.view_distanece(position, ViewDirection::U))
            .mul(self.view_distanece(position, ViewDirection::D))
    }
}

pub fn solve_2(input: &str) -> String {
    let scan = TreeGrid::from(input);

    let mut max = 0;

    let width = scan.width;
    let height = scan.height;

    for x in 0..width {
        for y in 0..height {
            let score = scan.scenic_score(&Coord { x, y });

            if score > max {
                max = score;
            }
        }
    }

    max.to_string()
}
