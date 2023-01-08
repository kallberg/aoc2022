use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Shape {
    parts: Vec<(u64, u64)>,
    max_y: u64,
}

impl Shape {
    fn one(y: u64) -> Self {
        let mut shape = Self {
            parts: vec![],
            max_y: y,
        };

        shape.parts.push((2, y));
        shape.parts.push((3, y));
        shape.parts.push((4, y));
        shape.parts.push((5, y));

        shape
    }

    fn two(y: u64) -> Self {
        let mut shape = Self {
            parts: vec![],
            max_y: y + 2,
        };

        shape.parts.push((3, y));

        shape.parts.push((2, y + 1));
        shape.parts.push((3, y + 1));
        shape.parts.push((4, y + 1));

        shape.parts.push((3, y + 2));

        shape
    }

    fn three(y: u64) -> Self {
        let mut shape = Self {
            parts: vec![],
            max_y: y + 2,
        };

        shape.parts.push((4, y + 2));
        shape.parts.push((4, y + 1));
        shape.parts.push((4, y));
        shape.parts.push((3, y));
        shape.parts.push((2, y));

        shape
    }

    fn four(y: u64) -> Self {
        let mut shape = Self {
            parts: vec![],
            max_y: y + 3,
        };

        shape.parts.push((2, y + 3));
        shape.parts.push((2, y + 2));
        shape.parts.push((2, y + 1));
        shape.parts.push((2, y));

        shape
    }

    fn five(y: u64) -> Self {
        let mut shape = Self {
            parts: vec![],
            max_y: y + 1,
        };

        shape.parts.push((2, y + 1));
        shape.parts.push((3, y + 1));
        shape.parts.push((2, y));
        shape.parts.push((3, y));

        shape
    }

    fn apply_jet(&mut self, jet: char) -> bool {
        assert!(jet == '<' || jet == '>');

        if jet == '<' && self.parts.iter().any(|(x, _)| *x == 0) {
            return false;
        }

        if jet == '>' && self.parts.iter().any(|(x, _)| *x == 6) {
            return false;
        }

        for (x, _) in &mut self.parts {
            if jet == '<' {
                *x -= 1;
            } else {
                *x += 1;
            }
        }

        true
    }

    fn move_down(&mut self) {
        for (_, y) in &mut self.parts {
            *y -= 1;
        }
        self.max_y -= 1;
    }

    fn move_up(&mut self) {
        for (_, y) in &mut self.parts {
            *y += 1;
        }
        self.max_y += 1;
    }

    fn intersects(&self, rocks: &HashSet<(u64, u64)>) -> bool {
        let mut disjoint = true;

        for part in &self.parts {
            if rocks.contains(part) {
                disjoint = false;

                break;
            }
        }

        disjoint
    }
}

fn fingerprint<'a, R>(rocks: R, top: u64, depth: u64) -> [u64; 7]
where
    R: IntoIterator<Item = &'a (u64, u64)>,
{
    let mut fingerprint = [0u64; 7];

    for (x, y) in rocks {
        let y = top - *y;
        let x = *x;

        if y > depth {
            continue;
        }

        fingerprint[x as usize] |= 1 << y;
    }

    fingerprint
}

pub fn solve(input: &str, total_shapes: u64) -> u64 {
    let chars: Vec<char> = input.chars().collect();
    let jets = input.len() as u64;
    let mut shape_counter = 0u64;
    let mut jet_counter = 0u64;
    let mut top = 0;
    let mut added = 0;
    let mut rocks_set: HashSet<(u64, u64)> = HashSet::new();
    let mut rocks_vec: Vec<(u64, u64)> = vec![];

    rocks_set.insert((0, 0));
    rocks_set.insert((1, 0));
    rocks_set.insert((2, 0));
    rocks_set.insert((3, 0));
    rocks_set.insert((4, 0));
    rocks_set.insert((5, 0));
    rocks_set.insert((6, 0));

    rocks_vec.extend(rocks_set.iter());

    type Key = (usize, usize, [u64; 7]);

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

            if moved && !shape.intersects(&rocks_set) {
                shape = copy;
            }

            shape.move_down();

            if !shape.intersects(&rocks_set) {
                shape.move_up();
                for part in shape.parts {
                    if rocks_set.insert(part) {
                        rocks_vec.push(part);
                    }
                }

                top = top.max(shape.max_y);

                let key: Key = (
                    jet_index,
                    shape_index,
                    fingerprint(rocks_vec.iter(), top, 40),
                );

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
