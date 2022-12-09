#[derive(Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
pub enum Direction {
    U,
    R,
    D,
    L,
}

pub struct Move {
    pub direction: Direction,
    pub distance: usize,
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
}

impl RopeSimulation {
    pub fn tail_correction(&mut self) {
        if self.tail_aligned() {
            if self.tail.x + 1 < self.head.x {
                self.tail.x = self.head.x - 1
            } else if self.tail.x - 1 > self.head.x {
                self.tail.x = self.head.x + 1
            } else if self.tail.y + 1 < self.head.y {
                self.tail.y = self.head.y - 1
            } else if self.tail.y - 1 > self.head.y {
                self.tail.y = self.head.y + 1
            }
        } else {
        }
    }

    pub fn tail_aligned(&self) -> bool {
        self.head.x == self.tail.x || self.head.y == self.tail.y
    }

    pub fn perform_move(&mut self, instruction: Move) {
        if instruction.distance == 0 {
            return;
        }

        match instruction.direction {
            Direction::U => self.head.y -= 1,
            Direction::R => self.head.x += 1,
            Direction::D => self.head.y += 1,
            Direction::L => self.head.x -= 1,
        }
    }
}

pub fn solve(input: &str) -> usize {
    let moves: Vec<Move> = input.lines().map(Move::from).collect();

    let mut sim = RopeSimulation::default();

    for instruction in moves {
        sim.perform_move(instruction)
    }

    0
}
