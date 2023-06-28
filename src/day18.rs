#[derive(Clone, PartialEq)]
pub struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl Position {
    pub fn connects(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z) == 1
    }

    pub fn adjacent(&self) -> Vec<Position> {
        let mut output = vec![];

        if self.x > 0 {
            output.push(Self {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            });
        }
        output.push(Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        });

        if self.y > 0 {
            output.push(Self {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            });
        }
        output.push(Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        });

        if self.z > 0 {
            output.push(Self {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            });
        }
        output.push(Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        });

        output
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.splitn(3, ',').collect();

        Self {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        }
    }
}

pub fn count_connected_sides(mut cubes: Vec<Position>) -> usize {
    let mut count = 0;

    loop {
        let Some(current) = cubes.pop() else {
            break;
        };

        for other in &cubes {
            if current.connects(other) {
                count += 1;
            }
        }
    }

    count
}

fn bounds(cubes: &Vec<Position>) -> Position {
    let mut max_y = 0;
    let mut max_x = 0;
    let mut max_z = 0;

    for cube in cubes {
        max_x = max_x.max(cube.x);
        max_y = max_y.max(cube.y);
        max_z = max_z.max(cube.z);
    }

    Position {
        x: max_x,
        y: max_y,
        z: max_z,
    }
}

pub fn fill_water(cubes: &Vec<Position>, start: Position) -> Vec<Position> {
    let mut filled = vec![];
    let bounds = bounds(&cubes);

    let max_y = bounds.x + 1;
    let max_x = bounds.y + 1;
    let max_z = bounds.z + 1;

    let mut checked: Vec<Position> = vec![];
    let mut to_check = vec![start];

    while let Some(current) = to_check.pop() {
        if checked.contains(&current)
            || cubes.contains(&current)
            || current.x > max_x
            || current.y > max_y
            || current.z > max_z
        {
            continue;
        }

        checked.push(current.clone());

        let mut adjacent = current.adjacent();
        filled.push(current);
        to_check.append(&mut adjacent);
    }

    filled
}

pub fn find_trapped_air(cubes: &Vec<Position>) -> Vec<Position> {
    let bounds = bounds(cubes);
    let mut water = fill_water(cubes, Position { x: 0, y: 0, z: 0 });
    let mut not_trapped = cubes.clone();
    not_trapped.append(&mut water);
    let mut trapped = vec![];

    for x in 0..=bounds.x {
        for y in 0..=bounds.y {
            for z in 0..=bounds.z {
                let current = Position { x, y, z };
                if !not_trapped.contains(&current) {
                    trapped.push(current);
                }
            }
        }
    }

    trapped
}

pub fn solve_1(input: &str) -> String {
    let cubes: Vec<Position> = input.lines().map(Position::from).collect();
    let sides = cubes.len() * 6;

    (sides - count_connected_sides(cubes) * 2).to_string()
}
pub fn solve_2(input: &str) -> String {
    let mut cubes: Vec<Position> = input.lines().map(Position::from).collect();
    let mut trapped_air = find_trapped_air(&cubes);
    cubes.append(&mut trapped_air);

    let sides = cubes.len() * 6;
    (sides - count_connected_sides(cubes) * 2).to_string()
}
