use std::{collections::HashSet, fmt::Display, iter::repeat};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cell {
    Rock,
    Air,
    Sand(bool),
    SandSource,
}

impl Cell {
    pub fn blocks(&self) -> bool {
        Cell::Air.ne(self)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Rock => write!(f, "#"),
            Cell::Air => write!(f, "."),
            Cell::Sand(true) => write!(f, "~"),
            Cell::Sand(false) => write!(f, "#"),
            Cell::SandSource => write!(f, "+"),
        }
    }
}

impl Scan {
    pub fn add_path(&mut self, path: ScanPath) {
        let points = path.as_points();

        for rock_position in points {
            self.rocks.insert(rock_position);
        }
    }
}

#[derive(Debug)]
pub struct ScanPath {
    pub traces: Vec<(usize, usize)>,
}

impl ScanPath {
    pub fn as_points(&self) -> Vec<(usize, usize)> {
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

            let mut points: Vec<(usize, usize)> = match (left_x == right_x, left_y == right_y) {
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

pub struct Simulation {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub sand_x: usize,
    pub sand_y: usize,
    pub cells: Vec<Cell>,
    pub unsettled: Vec<usize>,
}

pub struct Scan {
    pub rocks: HashSet<(usize, usize)>,
}

impl From<&Scan> for Simulation {
    fn from(scan: &Scan) -> Self {
        let mut max_x = 0usize;
        let mut max_y = 0usize;
        let mut min_x = usize::MAX;
        let mut min_y = usize::MIN;

        for (x, y) in &scan.rocks {
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
        }

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let mut cells: Vec<Cell> = repeat(Cell::Air).take(width * height).collect();

        for (x, y) in &scan.rocks {
            let local_x = x - min_x;
            let local_y = y - min_y;
            let index = local_x + local_y * width;
            cells[index] = Cell::Rock
        }

        cells[500 - min_x] = Cell::SandSource;

        Simulation {
            x: min_x,
            y: min_y,
            width,
            height,
            sand_x: 500,
            sand_y: 0,
            cells,
            unsettled: vec![],
        }
    }
}

impl From<&str> for ScanPath {
    fn from(input: &str) -> Self {
        let coord_strs = input.split(" -> ");
        let coords: Vec<(usize, usize)> = coord_strs
            .filter_map(|coord_str| {
                coord_str
                    .split_once(',')
                    .and_then(|(left, right)| left.parse().ok().zip(right.parse().ok()))
            })
            .collect();

        Self { traces: coords }
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, cell) in self.cells.iter().enumerate() {
            cell.fmt(f)?;
            let size = self.width * self.height;

            if index > 0 && (index + 1) % self.width == 0 && index + 1 < size {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }

        Ok(())
    }
}

impl Simulation {
    pub fn cell_index_from_global(&self, x: usize, y: usize) -> usize {
        let local_x = x - self.x;
        let local_y = y - self.y;

        self.cell_index(local_x, local_y)
    }

    pub fn cell_index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);

        x + y * self.width
    }

    pub fn try_move_sand(&mut self, from: usize, to: usize) -> bool {
        let to_cell = &self.cells[to];

        if to_cell.blocks() {
            return false;
        }

        self.cells[to] = Cell::Sand(true);
        self.cells[from] = Cell::Air;

        true
    }

    pub fn step(&mut self) -> bool {
        let before = self.cells.clone();
        let source = &self.cells[self.cell_index_from_global(self.sand_x, self.sand_y)];
        assert_eq!(source, &Cell::SandSource);

        let below_source_index = self.cell_index_from_global(self.sand_x, self.sand_y + 1);
        let below_source = &self.cells[below_source_index];

        if Cell::Air.ne(below_source) {
            return false;
        }

        self.cells[below_source_index] = Cell::Sand(true);
        self.unsettled.push(below_source_index);

        let mut new_unsettled = vec![];

        for unsettled_index in 0..self.unsettled.len() {
            let index = self.unsettled[unsettled_index];
            // x + y * self.width
            let x = index % self.width;
            let y = (index - x) / self.width;

            let index = self.cell_index(x, y);

            if !self.unsettled.contains(&index) {
                continue;
            }

            let cell = &self.cells[index];

            if Cell::Sand(true).ne(cell) {
                continue;
            }

            if y + 1 == self.height {
                self.cells[index] = Cell::Air;
                continue;
            }

            let below_index = self.cell_index(x, y + 1);

            if self.try_move_sand(index, below_index) {
                new_unsettled.push(below_index);
                continue;
            }

            if x == 0 {
                self.cells[index] = Cell::Air;
                continue;
            }

            let left_down_index = self.cell_index(x - 1, y + 1);

            if self.try_move_sand(index, left_down_index) {
                new_unsettled.push(left_down_index);
                continue;
            }

            if x + 1 >= self.width {
                self.cells[index] = Cell::Air;
                continue;
            }

            let right_down_index = self.cell_index(x + 1, y + 1);

            if self.try_move_sand(index, right_down_index) {
                new_unsettled.push(right_down_index);
                continue;
            }

            self.cells[index] = Cell::Sand(false);
        }
        self.unsettled = new_unsettled;

        if before == self.cells {
            self.cells[below_source_index] = Cell::Sand(true);
            self.unsettled.push(below_source_index);
            return false;
        }

        true
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

    let mut iteration = 0;

    while simulation.step() {
        iteration += 1;

        assert!(iteration < 50000);
    }

    // println!("Iteration {}", iteration + 1);
    // println!("{}", simulation);

    let sand_cell_count: usize = simulation.cells.into_iter().fold(0, |count, cell| {
        if cell == Cell::Sand(false) {
            count + 1
        } else {
            count
        }
    });

    sand_cell_count.to_string()
}

pub fn solve_2(input: &str) -> String {
    "14".to_string()
}
