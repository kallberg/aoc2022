use std::{collections::HashSet, iter::repeat};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::U => self.y -= 1,
            Direction::R => self.x += 1,
            Direction::D => self.y += 1,
            Direction::L => self.x -= 1,
        }
    }

    fn follow(&mut self, head: &Position) {
        let dx = head.x - self.x;
        let dy = head.y - self.y;

        if dx.abs() >= 2 && dy.abs() >= 2 {
            self.x = if dx > 0 { head.x - 1 } else { head.x + 1 };
            self.y = if dy > 0 { head.y - 1 } else { head.y + 1 };
            return;
        }

        if dx.abs() >= 2 {
            self.x = if dx > 0 { head.x - 1 } else { head.x + 1 };
            self.y = head.y;
            return;
        }

        if dy.abs() >= 2 {
            self.y = if dy > 0 { head.y - 1 } else { head.y + 1 };
            self.x = head.x;
            return;
        }

        if dx == 0 && dy.abs() > 0 {
            self.y = if dy > 0 { head.y - 1 } else { head.y + 1 };
            return;
        }

        if dy == 0 && dx.abs() > 0 {
            self.x = if dx > 0 { head.x - 1 } else { head.x + 1 };
        }
    }
}

#[derive(Default, Debug)]
pub struct BoundingBox {
    pub lower: Position,
    pub upper: Position,
}

#[derive(Clone)]
pub enum Direction {
    U,
    R,
    D,
    L,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::R,
            "U" => Direction::U,
            "L" => Direction::L,
            "D" => Direction::D,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct RopeSimulation {
    pub parts: Vec<Position>,
    pub bounding_box: BoundingBox,
    pub tail_markers: HashSet<Position>,
}

impl RopeSimulation {
    pub fn mark_tail(&mut self) {
        self.tail_markers
            .insert(self.parts.last().expect("last tail part").clone());
    }

    pub fn perform_move(&mut self, instruction: Direction) {
        self.parts[0].move_direction(&instruction);
        let head = self.parts[0].clone();
        let mut cursor = self.parts[0].clone();

        for part in self.parts.iter_mut().skip(1) {
            part.follow(&cursor);
            cursor = part.clone();
        }

        self.mark_tail();
        self.bounding_box.lower.x = self.bounding_box.lower.x.min(head.x);
        self.bounding_box.lower.y = self.bounding_box.lower.x.min(head.y);
        self.bounding_box.upper.x = self.bounding_box.upper.x.max(head.x);
        self.bounding_box.upper.y = self.bounding_box.upper.x.max(head.y);
    }
}

pub fn display_area(sim: &RopeSimulation, bounding_box: &BoundingBox) {
    for y in bounding_box.lower.y..=bounding_box.upper.y {
        for x in bounding_box.lower.x..=bounding_box.upper.x {
            let point = Position { x, y };

            let char = sim
                .parts
                .iter()
                .enumerate()
                .find(|(_, part)| part.eq(&&point))
                .map(|(index, _)| match index {
                    0 => "H".to_string(),
                    1 => (if sim.parts.len() > 2 { "1" } else { "T" }).to_string(),
                    i => format!("{}", i),
                })
                .unwrap_or_else(|| {
                    (if sim.tail_markers.contains(&point) {
                        "#"
                    } else {
                        "."
                    })
                    .to_string()
                });

            print!("{}", char);
        }
        println!();
    }
}

pub fn parse_directions(input: &str) -> Vec<Direction> {
    let mut output = vec![];

    for line in input.lines() {
        let (dir_str, count_str) = line.split_once(' ').unwrap();
        let dir = Direction::from(dir_str);
        let count: usize = count_str.parse().unwrap();

        let mut moves: Vec<Direction> = repeat(dir).take(count).collect();
        output.append(&mut moves)
    }

    output
}

pub fn solve(input: &str) -> usize {
    let moves: Vec<Direction> = parse_directions(input);

    let mut sim = RopeSimulation::default();
    sim.parts
        .append(&mut repeat(Position::default()).take(2).collect());
    sim.mark_tail();

    for instruction in moves {
        sim.perform_move(instruction);
    }

    sim.tail_markers.len()
}
