use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[derive(Clone)]
pub enum Shape {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Default for Shape {
    fn default() -> Self {
        Shape::One
    }
}

#[derive(Clone, Default)]
pub struct Rock {
    pub start: usize,
    pub end: usize,
    pub data: VecDeque<u64>,
    pub shape: Shape,
}

impl Rock {
    pub fn intersects(&self, other: &Rock) -> bool {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start > end {
            return false;
        }

        for page in start..=end {
            for x in 0..7 {
                let self_index = (page - self.start) * 7 + x;
                let other_index = (page - other.start) * 7 + x;

                if self.data[self_index] & other.data[other_index] > 0 {
                    return true;
                }
            }
        }

        false
    }

    fn can_fall(&self) -> bool {
        if self.start > 0 {
            return true;
        }

        let floor_bit = 1;

        for index in 0..7 {
            if self.data[index] & floor_bit != 0 {
                return false;
            }
        }

        return true;
    }

    fn can_apply_jet(&self, jet: Jet) -> bool {
        for page in self.start..=self.end {
            let index = (page - self.start) * 7;

            let touches_wall = match jet {
                Jet::Left => self.data[index] != 0,
                Jet::Right => self.data[index + 6] != 0,
            };

            if touches_wall {
                return false;
            }
        }

        true
    }

    pub fn fall(&mut self) -> bool {
        if !self.can_fall() {
            return false;
        }

        let mut carry = 0;

        for page in (self.start..=self.end).rev() {
            let mut carry_next = 0;
            for x in 0..7 {
                let index = (page - self.start) * 7 + x;
                let carry_bit = 1 << x;
                let carried = carry & carry_bit != 0;
                let bottom_bit = (self.data[index] & 1);

                carry_next |= bottom_bit << x;
                self.data[index] >>= 1;

                if carried {
                    self.data[index] |= 1 << 63;
                }
            }
            carry = carry_next;
        }

        if carry != 0 && self.start > 0 {
            let mut data = VecDeque::from(vec![0u64; 7]);

            for carry_index in 0..7 {
                let carry_bit = 1 << carry_index;
                let carried = carry & carry_bit != 0;
                if carried {
                    data[carry_index] = 1 << 63;
                }
            }

            data.append(&mut self.data);
            self.data = data;
            self.start -= 1;
        }

        self.trim();

        true
    }

    fn fill(&mut self, page: usize, amount: usize) {
        for x in 0..7 {
            let index = (page - self.start) * 7;

            for bit_index in 0..amount {
                self.data[index + x] |= 1 << bit_index;
            }
        }
    }

    pub fn set(&mut self, x: u8, y: u64) {
        let page = y / 64;
        let page_y = y % 64;
        let index = page as usize * 7 + x as usize;
        let bit = 1 << page_y;
        self.data[index] |= bit;
    }

    pub fn as_char(&self, page: usize, x: u8, y: u64, falling: bool) -> char {
        if page > self.end || page < self.start {
            return '.';
        }

        let index = (page - self.start) * 7 + x as usize;
        let bit = 1 << y;

        if self.data[index] & bit != 0 {
            return match falling {
                true => '@',
                false => '#',
            };
        }

        return '.';
    }

    pub fn move_jet(&mut self, jet: Jet) {
        if !self.can_apply_jet(jet) {
            return;
        }

        match jet {
            Jet::Left => {
                self.data.rotate_left(1);
            }
            Jet::Right => {
                self.data.rotate_right(1);
            }
        }
    }

    pub fn piece_one(x: u8, y: u64) -> Rock {
        let start = (y / 64) as usize;
        let end = start;

        let mut piece = Rock {
            data: VecDeque::from(vec![0u64; 7]),
            end,
            start,
            ..Default::default()
        };

        let local_y = y % 64;

        piece.set(x, local_y);
        piece.set(x + 1, local_y);
        piece.set(x + 2, local_y);
        piece.set(x + 3, local_y);

        piece
    }

    pub fn piece_two(x: u8, y: u64) -> Rock {
        let start = (y / 64) as usize;
        let end = ((y + 2) / 64) as usize;

        let data = VecDeque::from(if start != end {
            vec![0u64; 14]
        } else {
            vec![0u64; 7]
        });

        let mut piece = Rock {
            data,
            end,
            start,
            ..Default::default()
        };

        let local_y = y % 64;

        piece.set(x + 1, local_y);

        piece.set(x, local_y + 1);
        piece.set(x + 1, local_y + 1);
        piece.set(x + 2, local_y + 1);

        piece.set(x + 1, local_y + 2);

        piece
    }

    pub fn piece_three(x: u8, y: u64) -> Rock {
        let start = (y / 64) as usize;
        let end = ((y + 2) / 64) as usize;

        let data = VecDeque::from(if start != end {
            vec![0u64; 14]
        } else {
            vec![0u64; 7]
        });

        let mut piece = Rock {
            data,
            end,
            start,
            ..Default::default()
        };

        let local_y = y % 64;

        // ###
        piece.set(x, local_y);
        piece.set(x + 1, local_y);
        piece.set(x + 2, local_y);

        //   #
        // . #
        piece.set(x + 2, local_y + 1);
        piece.set(x + 2, local_y + 2);

        piece
    }

    pub fn piece_four(x: u8, y: u64) -> Rock {
        let start = (y / 64) as usize;
        let end = ((y + 3) / 64) as usize;

        let data = VecDeque::from(if start != end {
            vec![0u64; 14]
        } else {
            vec![0u64; 7]
        });

        let mut piece = Rock {
            data,
            end,
            start,
            ..Default::default()
        };

        let local_y = y % 64;

        // #
        // #
        // #
        // #
        piece.set(x, local_y + 0);
        piece.set(x, local_y + 1);
        piece.set(x, local_y + 2);
        piece.set(x, local_y + 3);

        piece
    }

    pub fn piece_five(x: u8, y: u64) -> Rock {
        let start = (y / 64) as usize;
        let end = ((y + 1) / 64) as usize;

        let data = VecDeque::from(if start != end {
            vec![0u64; 14]
        } else {
            vec![0u64; 7]
        });

        let mut piece = Rock {
            data,
            end,
            start,
            ..Default::default()
        };

        let local_y = y % 64;

        // ##
        // ##
        piece.set(x + 0, local_y + 0);
        piece.set(x + 1, local_y + 0);
        piece.set(x + 0, local_y + 1);
        piece.set(x + 1, local_y + 1);

        piece
    }

    fn top(&self) -> u64 {
        for page in (self.start..=self.end).rev() {
            let index = (page - self.start) * 7;
            let mut max = 0;

            for x in 0..7 {
                if self.data[index + x] == 0 {
                    continue;
                }

                let mut local_max = 0;
                let mut copy = self.data[index + x];

                while copy != 0 {
                    copy >>= 1;
                    local_max += 1;
                }

                max = local_max.max(max);
            }

            if max > 0 {
                return page as u64 * 64 + max;
            }
        }

        0
    }

    fn top_layer_empty(&mut self) -> bool {
        let index = (self.end - self.start) * 7;

        for x in 0..7 {
            if self.data[index + x] > 0 {
                return false;
            }
        }

        true
    }

    fn pop_top_layer(&mut self) {
        if self.end > 0 && self.end > self.start {
            self.data.resize(self.data.len() - 7, 0);
            self.end -= 1;
        }
    }

    fn trim(&mut self) {
        if self.end == self.start {
            return;
        }

        if self.top_layer_empty() {
            self.pop_top_layer();
        }
    }

    fn combine(&mut self, other: &Rock) {
        while self.start > other.start {
            for _ in 0..7 {
                self.data.push_front(0)
            }
            self.start -= 1;
        }

        while self.end < other.end {
            for _ in 0..7 {
                self.data.push_back(0)
            }
            self.end += 1;
        }

        for page in other.start..=other.end {
            let self_index = (page - self.start) * 7;
            let other_index = (page - other.start) * 7;

            for x in 0..7 {
                self.data[self_index + x] |= other.data[other_index + x];
            }
        }
    }

    fn read(&self, x: u8, y: u64, bits: usize) -> u64 {
        let page_start = y as usize / 64;
        let page_end = if y % 64 != 0 {
            page_start + 1
        } else {
            page_start
        };

        let start_offset = y % 64;
        let end_offset = 63 - start_offset;

        let mut data = self.data[page_start + x as usize] >> start_offset;
        data |= self.data[page_end + x as usize] << end_offset;

        let bit_shift = 64 - bits;

        data <<= bit_shift;
        data >>= bit_shift;

        data
    }
}

fn kmp_search(haystack: &[u64], needle: &[u64]) -> Option<usize> {
    // Preprocess the needle to create the failure function
    let mut failure_function = vec![0; needle.len()];
    let mut j = 0;
    for i in 1..needle.len() {
        while j > 0 && needle[j] != needle[i] {
            j = failure_function[j - 1];
        }
        if needle[j] == needle[i] {
            j += 1;
        }
        failure_function[i] = j;
    }

    // Search for the needle in the haystack
    let mut j = 0;
    for i in 0..haystack.len() {
        while j > 0 && haystack[i] != needle[j] {
            j = failure_function[j - 1];
        }
        if haystack[i] == needle[j] {
            j += 1;
        }
        if j == needle.len() {
            return Some(i - j + 1);
        }
    }

    // Return None if the needle is not found
    None
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for page in (self.start..=self.end).rev() {
            for y in (0..64).rev() {
                write!(f, "|")?;
                for x in 0..7 {
                    let index = (page - self.start) * 7 + x;
                    let bit = 1 << y;
                    let value = self.data[index] & bit != 0;

                    match value {
                        true => write!(f, "#"),
                        false => write!(f, "."),
                    }?;
                }
                writeln!(f, "|")?;
            }
        }
        writeln!(f, "+-------+")?;

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Clone, Default)]
pub struct RockFallSolver {
    pub pattern: Vec<Jet>,
    pub chamber: Rock,
    pub fall_counter: usize,
    pub jet_counter: usize,
    pub falling: Rock,
    pub max_fall: usize,
    pub cache: HashMap<(VecDeque<u64>, Shape), u64>,
}

impl RockFallSolver {
    #[inline(never)]
    fn next_rock(&mut self) {
        let top: u64 = self.chamber.top();
        let x = 2;
        let y = top + 3;

        let rock_index = self.fall_counter % 5;
        self.fall_counter += 1;

        let rock = match rock_index {
            0 => Rock::piece_one(x, y),
            1 => Rock::piece_two(x, y),
            2 => Rock::piece_three(x, y),
            3 => Rock::piece_four(x, y),
            4 => Rock::piece_five(x, y),
            _ => unreachable!(),
        };
        self.falling = rock;
    }
    #[inline(never)]
    pub fn step(&mut self) {
        let pattern_index = self.jet_counter % self.pattern.len();
        self.jet_counter += 1;
        let jet = self.pattern[pattern_index];

        let mut falling_backup = self.falling.clone();

        self.falling.move_jet(jet);

        let jet_valid = !self.chamber.intersects(&self.falling);

        if !jet_valid {
            self.falling = falling_backup;
        }

        falling_backup = self.falling.clone();

        if !self.falling.fall() {
            self.chamber.combine(&self.falling);
            return self.next_rock();
        }

        let intersected = self.chamber.intersects(&self.falling);

        if intersected {
            self.chamber.combine(&falling_backup);
            return self.next_rock();
        }
    }

    #[inline(never)]
    pub fn step_rock(&mut self) {
        let rock_count = self.fall_counter;

        while self.fall_counter - rock_count < 1 {
            self.step();
        }
    }
}

pub fn solver(input: &str) -> RockFallSolver {
    let pattern: Vec<Jet> = input
        .chars()
        .filter_map(|char| match char {
            '<' => Some(Jet::Left),
            '>' => Some(Jet::Right),
            _ => None,
        })
        .collect();

    let mut solver = RockFallSolver {
        pattern,
        chamber: Rock {
            start: 0,
            end: 0,
            data: VecDeque::from(vec![0u64; 7]),
            ..Default::default()
        },
        falling: Rock {
            start: 0,
            end: 0,
            data: VecDeque::new(),
            ..Default::default()
        },
        ..Default::default()
    };

    solver.next_rock();

    solver
}

impl Display for RockFallSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for page in (self.chamber.start..=self.chamber.end).rev() {
            for y in (0..64).rev() {
                let global_y: u64 = y + page as u64 * 64;

                // if (global_y + 1) % 40 == 0 {
                //     writeln!(f)?;
                // }

                // write!(f, "p={} y={:0>5} ", page, global_y)?;

                write!(f, "|")?;
                for x in 0..7 {
                    let falling_char = self.falling.as_char(page, x, y, true);

                    if falling_char != '.' {
                        write!(f, "{}", falling_char)?;
                    } else {
                        write!(f, "{}", self.chamber.as_char(page, x, y, false))?;
                    }
                }
                writeln!(f, "|")?;
            }
        }
        //writeln!(f, "+-------+")?;

        Ok(())
    }
}

pub fn print_solver_page(solver: &RockFallSolver, page: usize) {
    let mut rock = Rock {
        data: VecDeque::new(),
        end: 0,
        start: 0,
        ..Default::default()
    };

    for x in 0..7 {
        let index = (page - solver.chamber.start) * 7;
        rock.data.push_back(solver.chamber.data[index + x])
    }

    let mut display_solver = solver.clone();
    display_solver.chamber = rock;

    println!("{}", display_solver);
}

pub fn test_rock_fall() {
    let mut rock = Rock::piece_three(2, 70);

    while rock.fall() {
        println!("{}", rock);
    }
}

pub fn solve_1(input: &str) -> String {
    let rock = Rock::piece_two(2, 5);

    println!("{}", rock);
    let rock_data = rock.read(2, 0, 20);

    let mut solver = solver(input);

    for _ in 0..2022 {
        solver.step_rock();
    }

    let solution = solver.chamber.top();

    println!("{}", solver.max_fall);
    println!("{}", solver);

    solution.to_string()
}

pub fn solve_2(input: &str) -> String {
    let mut solver = solver(input);

    for _ in 0..10000 {
        solver.step_rock();
    }

    let solution = solver.chamber.top();

    println!("{}", solver.max_fall);
    println!("{}", solver);

    solution.to_string()

    // return "Ok".into();

    // let mut solver = solver(input);

    // for _ in 0u64..1000000000000 {
    //     solver.step_rock();
    // }

    // let solution = solver.chamber.top();

    // solution.to_string()
}
