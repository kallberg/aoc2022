use std::ops::Mul;

use crate::day8pt1::{Coord, TreeGrid};

pub enum ViewDirection {
    L,
    R,
    U,
    D,
}

impl TreeGrid {
    pub fn view_distanec(&self, position: &Coord, direction: ViewDirection) -> usize {
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
        self.view_distanec(position, ViewDirection::L)
            .mul(self.view_distanec(position, ViewDirection::R))
            .mul(self.view_distanec(position, ViewDirection::U))
            .mul(self.view_distanec(position, ViewDirection::D))
    }
}

pub fn solve(input: &str) -> String {
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
