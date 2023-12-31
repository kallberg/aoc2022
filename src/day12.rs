use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::{Arc, RwLock},
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::extra::visualize;

#[derive(Debug, Clone)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn moved(&self, move_value: &Move) -> Point {
        match move_value {
            Move::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Move::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Move::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Move::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Climber {
    pub position: Point,
    pub goal: Point,
    pub elevation: usize,
    pub visited: HashSet<Point>,
    pub history: HashMap<Point, Move>,
    pub moves: usize,
    pub max_moves: usize,
}

impl Default for Climber {
    fn default() -> Self {
        Self {
            max_moves: usize::MAX,
            position: Default::default(),
            elevation: Default::default(),
            goal: Default::default(),
            history: Default::default(),
            moves: Default::default(),
            visited: Default::default(),
        }
    }
}

#[derive(Default, Clone)]
pub struct Climb {
    pub grid: Vec<Vec<usize>>,
    pub height: usize,
    pub width: usize,
    pub starting_climber: Climber,
    pub route_lengths: HashMap<Point, usize>,
}

impl From<&str> for Climb {
    fn from(input: &str) -> Self {
        let mut output = Climb::default();

        for (y, line) in input.lines().enumerate() {
            let mut row = vec![];
            output.height = y.max(output.height);

            for (x, char) in line.chars().enumerate() {
                output.width = x.max(output.width);
                let mut elevation = 0;

                if char == 'S' {
                    output.starting_climber.position.x = x;
                    output.starting_climber.position.y = y;
                    output.starting_climber.visited.insert(Point { x, y });
                } else if char == 'E' {
                    output.starting_climber.goal.x = x;
                    output.starting_climber.goal.y = y;
                    elevation = ('a'..='z').count() - 1;
                } else {
                    elevation = ('a'..='z')
                        .enumerate()
                        .find(|(_, reference)| char.eq(reference))
                        .map(|(index, _)| index)
                        .unwrap();
                }

                row.push(elevation);
            }

            output.grid.push(row);
        }
        output
    }
}

impl Display for Climb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let position = Point { x, y };

                if self.starting_climber.position.eq(&position) {
                    write!(f, "S")?;
                } else if self.starting_climber.goal.eq(&position) {
                    write!(f, "E")?;
                } else {
                    write!(f, "{}", ('a'..='z').nth(*col).unwrap())?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Climb {
    pub fn elevation_at(&self, position: &Point) -> usize {
        self.grid[position.y][position.x]
    }

    pub fn starting_points(&self) -> Vec<Point> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, height)| (x, y, height))
            })
            .filter(|(_, _, height)| **height == 0)
            .map(|(x, y, _)| Point { x, y })
            .collect()
    }
}

impl Climber {
    pub fn can_move_to(&self, climb: &Climb, position: &Point) -> bool {
        if self.moves >= self.max_moves {
            return false;
        }

        if self.visited.contains(position) {
            return false;
        }

        let elevation = climb.elevation_at(position);

        self.elevation + 1 >= elevation
    }

    pub fn perform_move(&mut self, instruction: Move, elevation: usize) {
        let position = self.position.moved(&instruction);
        self.history.insert(self.position.clone(), instruction);
        self.position = position.clone();
        self.elevation = elevation;
        self.visited.insert(position);
        self.moves += 1;
    }

    pub fn moves(&self, climb: &Climb) -> Vec<Move> {
        let mut output = vec![];

        if self.position.y > 0 {
            let up = self.position.moved(&Move::Up);

            if self.can_move_to(climb, &up) {
                output.push(Move::Up);
            }
        }

        if self.position.y < climb.height {
            let down = self.position.moved(&Move::Down);

            if self.can_move_to(climb, &down) {
                output.push(Move::Down);
            }
        }

        if self.position.x > 0 {
            let left = self.position.moved(&Move::Left);

            if self.can_move_to(climb, &left) {
                output.push(Move::Left);
            }
        }

        if self.position.x < climb.width {
            let right = self.position.moved(&Move::Right);

            if self.can_move_to(climb, &right) {
                output.push(Move::Right);
            }
        }

        output
    }

    pub fn is_done(&self) -> bool {
        self.position.eq(&self.goal)
    }

    pub fn climb_par(&self, climb: Arc<RwLock<Climb>>) -> Vec<Climber> {
        let mut output = vec![];

        let moves: Vec<Move>;

        {
            let readable_climb = climb.read().unwrap();
            moves = self.moves(&readable_climb);
        }

        for move_value in moves {
            let mut next = self.clone();
            let point = self.position.moved(&move_value);

            {
                let readable_climb = climb.read().unwrap();
                if let Some(existing_route) = readable_climb.route_lengths.get(&point) {
                    if *existing_route <= next.moves + 1 {
                        continue;
                    }
                }
            }

            let elevation: usize;
            {
                let readable_climb = climb.read().unwrap();
                elevation = readable_climb.elevation_at(&point);
            }

            next.perform_move(move_value, elevation);

            {
                let mut writeable_climb = climb.write().unwrap();
                writeable_climb
                    .route_lengths
                    .insert(next.position.clone(), next.moves);
            }

            output.push(next)
        }

        output
    }

    pub fn climb(&self, climb: &mut Climb) -> Vec<Climber> {
        let mut output = vec![];

        let moves = self.moves(climb);

        for move_value in moves {
            let mut next = self.clone();
            let point = self.position.moved(&move_value);

            if let Some(existing_route) = climb.route_lengths.get(&point) {
                if *existing_route <= next.moves + 1 {
                    continue;
                }
            }

            let elevation = climb.elevation_at(&point);
            next.perform_move(move_value, elevation);

            climb
                .route_lengths
                .insert(next.position.clone(), next.moves);

            output.push(next)
        }

        output
    }
}

impl Display for Climber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let visited: Vec<Point> = self.visited.clone().into_iter().collect();

        let max_x = visited.iter().map(|p| p.x).max().unwrap();
        let max_y = visited.iter().map(|p| p.y).max().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                write!(
                    f,
                    "{}",
                    match self.history.get(&Point { x, y }) {
                        Some(move_value) => match move_value {
                            Move::Up => '▲',
                            Move::Down => '▼',
                            Move::Left => '◀',
                            Move::Right => '▶',
                        },
                        None => match (
                            self.goal.eq(&Point { x, y }),
                            self.position.eq(&Point { x, y }),
                        ) {
                            (true, true) => 'X',
                            (true, false) => 'E',
                            (false, true) => 'S',
                            (false, false) => '·',
                        },
                    }
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn manage_climbers_par(climbers: Vec<Climber>, climb: Arc<RwLock<Climb>>) -> Vec<Climber> {
    let mut next = vec![];

    for climber in climbers {
        let mut new = climber.climb_par(climb.clone());
        next.append(&mut new);
    }

    next
}

pub fn manage_climbers(climbers: Vec<Climber>, climb: &mut Climb) -> Vec<Climber> {
    let mut next = vec![];

    for climber in climbers {
        let mut new = climber.climb(climb);
        next.append(&mut new);
    }

    next
}

pub fn solve_1(input: &str) -> String {
    let mut climb = Climb::from(input);
    let mut climbers = vec![climb.starting_climber.clone()];

    let mut best_climbers: Vec<Climber>;

    for _ in 0..500 {
        climbers = manage_climbers(climbers, &mut climb);

        best_climbers = climbers.iter().filter(|p| p.is_done()).cloned().collect();

        if !best_climbers.is_empty() {
            break;
        }
    }

    let best_climber = climbers.first().unwrap();

    visualize(12, 1, &best_climber);

    best_climber.moves.to_string()
}

pub fn best_climber_from_point(
    climb: Arc<RwLock<Climb>>,
    point: Point,
    best: usize,
) -> Option<Climber> {
    let mut starting_climber: Climber;
    {
        let readable_climb = climb.read().unwrap();
        starting_climber = readable_climb.starting_climber.clone();
    }

    starting_climber.visited.clear();
    starting_climber.position = point.clone();
    starting_climber.visited.insert(point);
    starting_climber.max_moves = best - 1;

    let mut climbers = vec![starting_climber];

    let mut best_climbers: Vec<Climber>;

    while !climbers.is_empty() {
        climbers = manage_climbers_par(climbers, climb.clone());

        best_climbers = climbers.iter().filter(|p| p.is_done()).cloned().collect();

        if !best_climbers.is_empty() {
            break;
        }
    }

    climbers.first().cloned()
}

pub fn solve_2(input: &str) -> String {
    let climb = Climb::from(input);
    let starting_points = climb.starting_points();
    let climb = Arc::new(RwLock::new(climb));

    let best = usize::MAX;

    let climbers: Vec<Climber> = starting_points
        .par_iter()
        .filter_map(|point| best_climber_from_point(climb.clone(), point.clone(), best))
        .collect();

    let mut best_climber: Option<Climber> = None;

    for climber in climbers {
        if let Some(current_best) = &best_climber {
            if current_best.moves > climber.moves {
                best_climber = Some(climber);
            }
        } else {
            best_climber = Some(climber);
        }
    }

    let best = best_climber.unwrap();

    visualize(12, 2, &best);

    best.moves.to_string()
}
