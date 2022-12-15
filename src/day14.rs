use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    iter::repeat,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Rock,
    Sand(bool),
    SandSource,
}

#[derive(Debug)]
pub struct ScanPath {
    pub traces: Vec<(i64, i64)>,
}

pub struct Simulation {
    pub x: i64,
    pub y: i64,
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub cells: HashMap<(i64, i64), Cell>,
    pub spawn_source: (i64, i64),
    pub resting: usize,
}

pub struct Scan {
    pub rocks: HashSet<(i64, i64)>,
}

impl Scan {
    pub fn add_path(&mut self, path: ScanPath) {
        let points = path.as_points();

        for rock_position in points {
            self.rocks.insert(rock_position);
        }
    }
}

impl Cell {
    pub fn blocks(&self) -> bool {
        !matches!(self, Cell::SandSource)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Rock => write!(f, "#"),
            Cell::Sand(true) => write!(f, "~"),
            Cell::Sand(false) => write!(f, "o"),
            Cell::SandSource => write!(f, "+"),
        }
    }
}

impl ScanPath {
    pub fn as_points(&self) -> Vec<(i64, i64)> {
        let mut output = Vec::new();

        let windows = self.traces.windows(2);

        for window in windows {
            let (left_x, left_y) = window[0];
            let (right_x, right_y) = window[1];

            let from_x = left_x.min(right_x);
            let from_y = left_y.min(right_y);
            let to_x = left_x.max(right_x);
            let to_y = left_y.max(right_y);
            let range_x = from_x..=to_x;
            let range_y = from_y..=to_y;

            let mut points: Vec<(i64, i64)> = match (left_x == right_x, left_y == right_y) {
                (true, true) => panic!("scanner malfunction, duplicate path registered"),
                (true, false) => repeat(left_x).zip(range_y).collect(),
                (false, true) => (range_x).zip(repeat(left_y)).collect(),
                (false, false) => panic!("scanner malfunction, diagnoal path registered"),
            };

            output.append(&mut points);
        }

        output
    }
}

impl From<&Scan> for Simulation {
    fn from(scan: &Scan) -> Self {
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        let mut min_x = i64::MAX;

        for (x, y) in &scan.rocks {
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
            min_x = min_x.min(*x);
        }

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y + 1) as usize;
        let mut cells = HashMap::new();

        for (x, y) in &scan.rocks {
            cells.insert((*x, *y), Cell::Rock);
        }

        let spawn_source = (500, 0);
        cells.insert((500, 0), Cell::SandSource);

        Simulation {
            x: min_x,
            y: 0,
            width,
            height,
            size: width * height,
            cells,
            spawn_source,
            resting: 0,
        }
    }
}

impl From<&str> for ScanPath {
    fn from(input: &str) -> Self {
        let coord_strs = input.split(" -> ");
        let coords: Vec<(i64, i64)> = coord_strs
            .filter_map(|coord_str| {
                coord_str
                    .split_once(',')
                    .and_then(|(left, right)| left.parse().ok().zip(right.parse().ok()))
            })
            .collect();

        Self { traces: coords }
    }
}

fn read_digit(num: i64, magnitude: u32) -> i64 {
    (num / 10i64.pow(magnitude - 1)) % 10
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_legend_height = (self.x as f64 + self.width as f64).log10() as u32 + 1;
        let y_legend_width = ((self.y as f64 + self.width as f64).log2() / 4f64).floor() as u32 + 1;

        for y in (0..x_legend_height).rev() {
            write!(f, "{:>width$}", "", width = y_legend_width as usize + 1)?;
            for x in self.x..(self.x + self.width as i64) {
                if x % 10 == 0 {
                    write!(f, "{} ", read_digit(x, y + 1))?
                } else if x + 1 - self.x != self.width as i64 {
                    write!(f, "* ")?;
                }
            }
            write!(f, "*")?;
            writeln!(f)?;
        }
        for y in self.y..(self.y + self.height as i64) {
            write!(f, "{:0>width$x} ", y, width = y_legend_width as usize)?;
            for x in self.x..(self.x + self.width as i64) {
                match self.cells.get(&(x, y)) {
                    Some(c) => c.fmt(f),
                    None => write!(f, "."),
                }?;
                if x + 1 - self.x != self.width as i64 {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Simulation {
    pub fn next_position(&self, position: (i64, i64)) -> (i64, i64) {
        let (mut x, mut y) = position;

        if y < 0 {
            return (x, 0);
        }

        if self.is_blocked((x, y)) {
            return position;
        }

        if y >= self.height as i64 {
            return position;
        }

        y += 1;

        if !self.is_blocked((x, y)) {
            return (x, y);
        }

        // Move left
        x -= 1;

        if !self.is_blocked((x, y)) {
            // left is available
            return (x, y);
        }

        // Move to right side of block from left side
        x += 2;

        if !self.is_blocked((x, y)) {
            // Down right is available check there
            return (x, y);
        }

        // Left and right blocked, go back up
        x -= 1;
        y -= 1;

        (x, y)
    }

    pub fn is_blocked(&self, position: (i64, i64)) -> bool {
        if let Some(cell) = self.cells.get(&position) {
            cell.blocks()
        } else {
            false
        }
    }

    pub fn add_floor(&mut self, height: usize) {
        let from_x = self.x - self.width as i64;
        let to_x = self.x + self.width as i64 * 2;

        for x in from_x..to_x {
            self.cells.insert((x, height as i64), Cell::Rock);
        }
    }

    pub fn step_resting(&mut self) -> bool {
        let spawn_index = self.spawn_source;
        let mut current = spawn_index;

        loop {
            let next = self.next_position(current);

            if current == next {
                break;
            }

            current = next;
        }

        if current.1 + 1 >= self.height as i64 {
            return false;
        }

        match self.cells.get(&current) {
            Some(cell) => match cell {
                Cell::Rock => false,
                Cell::Sand(false) => {
                    self.cells.insert(current, Cell::Sand(true));
                    false
                }
                Cell::Sand(true) => false,
                Cell::SandSource => {
                    self.cells.insert(current, Cell::Sand(false));
                    self.resting += 1;
                    false
                }
            },
            None => {
                self.cells.insert(current, Cell::Sand(false));
                self.resting += 1;
                true
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            if !self.step_resting() {
                break;
            }
        }
        let mut step = self.next_position(self.spawn_source);

        loop {
            let next = self.next_position(step);

            if next == step {
                break;
            }
            self.cells.insert(step, Cell::Sand(true));
            step = next;
        }
    }
}

pub fn solve_1(input: &str) -> String {
    let mut scan = Scan {
        rocks: HashSet::new(),
    };

    for path in input.lines().map(ScanPath::from) {
        scan.add_path(path);
    }

    let mut simulation = Simulation::from(&scan);

    simulation.height += 2;
    simulation.width += 2;
    simulation.x -= 1;

    simulation.run();
    // println!("{}", simulation);

    simulation.resting.to_string()
}

pub fn solve_2(input: &str) -> String {
    let mut scan = Scan {
        rocks: HashSet::new(),
    };

    for path in input.lines().map(ScanPath::from) {
        scan.add_path(path);
    }

    let mut simulation = Simulation::from(&scan);

    let needed_width = (simulation.height + 3) * 2;

    let delta_width = needed_width - simulation.width;

    simulation.width += delta_width;
    simulation.add_floor(simulation.height + 1);
    simulation.height += 2;
    simulation.width -= simulation.width % 2 + 1;
    simulation.x = simulation.spawn_source.0 - simulation.width as i64 / 2;

    simulation.run();
    // println!("{}", simulation);
    simulation.resting.to_string()
}
