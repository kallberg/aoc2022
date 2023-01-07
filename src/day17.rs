use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Shape {
    parts: HashSet<(u64, u64)>,
}

impl Shape {
    fn one(y: u64) -> Self {
        let mut shape = Self {
            parts: HashSet::new(),
        };

        shape.parts.insert((2, y));
        shape.parts.insert((3, y));
        shape.parts.insert((4, y));
        shape.parts.insert((5, y));

        shape
    }

    fn two(y: u64) -> Self {
        let mut shape = Self {
            parts: HashSet::new(),
        };

        shape.parts.insert((3, y));

        shape.parts.insert((2, y + 1));
        shape.parts.insert((3, y + 1));
        shape.parts.insert((4, y + 1));

        shape.parts.insert((3, y + 2));

        shape
    }

    fn three(y: u64) -> Self {
        let mut shape = Self {
            parts: HashSet::new(),
        };

        shape.parts.insert((4, y + 2));
        shape.parts.insert((4, y + 1));
        shape.parts.insert((4, y));
        shape.parts.insert((3, y));
        shape.parts.insert((2, y));

        shape
    }

    fn four(y: u64) -> Self {
        let mut shape = Self {
            parts: HashSet::new(),
        };

        shape.parts.insert((2, y + 3));
        shape.parts.insert((2, y + 2));
        shape.parts.insert((2, y + 1));
        shape.parts.insert((2, y));

        shape
    }

    fn five(y: u64) -> Self {
        let mut shape = Self {
            parts: HashSet::new(),
        };

        shape.parts.insert((2, y + 1));
        shape.parts.insert((3, y + 1));
        shape.parts.insert((2, y));
        shape.parts.insert((3, y));

        shape
    }

    fn apply_jet(&mut self, jet: char) -> bool {
        assert!(jet == '<' || jet == '>');

        let mut new_parts = HashSet::new();

        for (x, y) in &self.parts {
            let (x, y) = (*x, *y);
            if jet == '<' {
                if x > 0 {
                    new_parts.insert((x - 1, y));
                } else {
                    return false;
                }
            } else if x < 6 {
                new_parts.insert((x + 1, y));
            } else {
                return false;
            }
        }

        self.parts = new_parts;

        true
    }

    fn move_down(&mut self) {
        let mut new_parts = HashSet::new();

        for (x, y) in &self.parts {
            let (x, y) = (*x, *y);
            if y > 0 {
                new_parts.insert((x, y - 1));
            } else {
                return;
            }
        }

        self.parts = new_parts;
    }

    fn move_up(&mut self) {
        let mut new_parts = HashSet::new();

        for (x, y) in &self.parts {
            let (x, y) = (*x, *y);
            new_parts.insert((x, y + 1));
        }

        self.parts = new_parts;
    }
}

fn fingerprint<'a, R>(rocks: R, top: u64, depth: u64) -> Vec<(u64, u64)>
where
    R: IntoIterator<Item = &'a (u64, u64)>,
{
    let mut fingerprint = vec![];

    for (x, y) in rocks {
        if top - *y <= depth {
            fingerprint.push((*x, top - *y));
        }
    }

    fingerprint.sort();

    fingerprint
}

pub fn solve(input: &str, total_shapes: u64) -> u64 {
    let chars: Vec<char> = input.chars().collect();
    let jets = input.len() as u64;
    let mut shape_counter = 0u64;
    let mut jet_counter = 0u64;
    let mut top = 0;
    let mut added = 0;
    let mut rocks: HashSet<(u64, u64)> = HashSet::new();
    rocks.insert((0, 0));
    rocks.insert((1, 0));
    rocks.insert((2, 0));
    rocks.insert((3, 0));
    rocks.insert((4, 0));
    rocks.insert((5, 0));
    rocks.insert((6, 0));

    type Key = (usize, usize, Vec<(u64, u64)>);

    let mut cache = HashMap::<Key, (u64, u64)>::new();

    while shape_counter < total_shapes {
        let shape_index = (shape_counter % 5) as usize;
        shape_counter += 1;

        let start_y = top + 4;

        let mut shape = match shape_index {
            0 => Shape::one(start_y),
            1 => Shape::two(start_y),
            2 => Shape::three(start_y),
            3 => Shape::four(start_y),
            4 => Shape::five(start_y),
            _ => unreachable!(),
        };

        loop {
            let jet_index = (jet_counter % jets) as usize;

            let jet = chars[jet_index];
            jet_counter += 1;

            let copy = shape.clone();
            let moved = shape.apply_jet(jet);

            if moved && !shape.parts.is_disjoint(&rocks) {
                shape = copy;
            }

            shape.move_down();

            if !shape.parts.is_disjoint(&rocks) {
                shape.move_up();
                rocks.extend(shape.parts);

                let max_rock = rocks.iter().map(|(_, y)| *y).max();

                top = max_rock.unwrap();

                let key: Key = (jet_index, shape_index, fingerprint(&rocks, top, 40));

                if let Some(cache_hit) = cache.get(&key) {
                    let (old_shape_counter, old_top) = *cache_hit;

                    let delta_top = top - old_top;
                    let delta_counter = shape_counter - old_shape_counter;
                    let shapes_left = total_shapes - shape_counter;
                    let repeats = shapes_left / delta_counter;
                    added += repeats * delta_top;
                    shape_counter += repeats * delta_counter;
                }

                cache.insert(key, (shape_counter, top));

                break;
            }
        }
    }

    top + added
}

pub fn solve_1(input: &str) -> String {
    solve(input, 2022).to_string()
}

pub fn solve_2(input: &str) -> String {
    solve(input, 1000000000000).to_string()
}
