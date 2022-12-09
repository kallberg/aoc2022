use std::{fmt::Display, collections::HashSet};

#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Default)]
pub struct BoundingBox {
    lower: Position,
    upper: Position,
}

pub enum Direction {
    U,
    R,
    D,
    L,
}

pub struct Move {
    pub direction: Direction,
    pub distance: i64,
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

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let (dir_str, distance_str) = value.split_once(' ').expect("parse move");

        Move {
            direction: Direction::from(dir_str),
            distance: distance_str.parse().expect("parse distance"),
        }
    }
}

#[derive(Default)]
pub struct RopeSimulation {
    pub head: Position,
    pub tail: Position,
    pub bounding_box: BoundingBox,
    pub tail_markers: HashSet<Position>
}

impl RopeSimulation {
    pub fn tail_correction(&mut self) {
        let delta_x = self.head.x - self.tail.x;
        let delta_y = self.head.y - self.tail.y;

        match (delta_x, delta_y) {
            (-2, _) => {
                self.tail.x -= 1;
                self.tail.y = self.head.y
            },
            (2, _) => {
                self.tail.x += 1;
                self.tail.y = self.head.y
            },
            (_, -2) => {
                self.tail.y -= 1;
                self.tail.x = self.head.x;
            },
            (_, 2) => {
                self.tail.y += 1;
                self.tail.x = self.head.x;
            },
            (1, 1) => {},
            (1, -1) => {},
            (-1, 1) => {},
            (-1, -1) => {},
            (0, 1) => {},
            (1, 0) => {},
            (0, 0) => {},
            (0, -1) => {},
            (-1, 0) => {},
            value => 
                {
                    println!("hmm {:?}", value);
                    panic!("invalid tail state")
                }
        }
    }

    pub fn tail_aligned(&self) -> bool {
        self.head.x == self.tail.x || self.head.y == self.tail.y
    }

    pub fn mark_tail(&mut self) {
        self.tail_markers.insert(self.tail.clone());
    }

    pub fn perform_move(&mut self, mut instruction: Move) {
        if instruction.distance == 0 {
            return;
        }

        match instruction.direction {
            Direction::U => self.head.y -= 1,
            Direction::R => self.head.x += 1,
            Direction::D => self.head.y += 1,
            Direction::L => self.head.x -= 1,
        }

        self.tail_correction();
        self.mark_tail();

        self.bounding_box.lower.x = self.bounding_box.lower.x.min(self.head.x);
        self.bounding_box.lower.y = self.bounding_box.lower.x.min(self.head.y);
        self.bounding_box.upper.x = self.bounding_box.upper.x.max(self.head.x);
        self.bounding_box.upper.y = self.bounding_box.upper.x.max(self.head.y);

        if instruction.distance > 1 {
            instruction.distance -= 1;
            self.perform_move(instruction)
        }
    }
}

impl Display for RopeSimulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.bounding_box.lower.y ..= self.bounding_box.upper.y {
            for x in self.bounding_box.lower.x ..= self.bounding_box.upper.y {
                if self.head.x == x && self.head.y == y {
                    write!(f, "H")?;
                } else if self.tail.x == x && self.tail.y == y {
                    write!(f, "T")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn display_area(sim: &RopeSimulation, bounding_box: BoundingBox) {
    for y in bounding_box.lower.y..=bounding_box.upper.y {
        for x in bounding_box.lower.x..=bounding_box.upper.x {
            let point = Position {x,y};

            if sim.head.eq(&point) {
                print!("H");
            } else if sim.tail.eq(&point) {
                print!("T");
            } else if sim.tail_markers.contains(&point) {
                print!("#");
            }
            else {
                print!(".");
            } 
        }
        println!()
    }
}

pub fn solve(input: &str) -> usize {
    let moves: Vec<Move> = input.lines().map(Move::from).collect();

    let mut sim = RopeSimulation::default();
    sim.mark_tail();
    
    display_area(&sim, BoundingBox {  lower: Position { x: -5, y: -5 }, upper: Position { x: 5, y: 5 } });
    println!();

    for instruction in moves {
        sim.perform_move(instruction);
        display_area(&sim, BoundingBox {  lower: Position { x: -5, y: -5 }, upper: Position { x: 5, y: 5 } });
        println!();
    }

    sim.tail_markers.len()
}
