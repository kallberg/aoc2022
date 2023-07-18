use std::collections::HashMap;

#[derive(Clone)]
struct ValveSystem {
    size: u8,
    names: Vec<String>,
    flow_rates: Vec<u8>,
    flow_rates_sorted: Vec<u8>,
    distances: Vec<Vec<u8>>,
}

impl ValveSystem {
    fn trim(&mut self) {
        let start = self.start();

        let mut trimmed = Self {
            distances: vec![],
            names: vec![],
            size: 0,
            flow_rates: vec![],
            flow_rates_sorted: vec![],
        };

        for index in 0..self.size as usize {
            if self.flow_rates[index] == 0 && index != start as usize {
                continue;
            }

            trimmed.names.push(self.names[index].clone());
            trimmed.flow_rates.push(self.flow_rates[index]);
            trimmed.size += 1;

            let mut distance_row = vec![];

            for to in 0..self.size as usize {
                if self.flow_rates[to] == 0 && to != start as usize {
                    continue;
                }

                distance_row.push(self.distances[index][to])
            }

            trimmed.distances.push(distance_row);
        }

        self.distances = trimmed.distances;
        self.names = trimmed.names;
        self.size = trimmed.size;
        self.flow_rates = trimmed.flow_rates;
        self.flow_rates_sorted = trimmed.flow_rates_sorted;
    }

    fn start(&self) -> u8 {
        let mut start = 0;

        let start_name = "AA".to_string();

        for name in &self.names {
            if start_name.eq(name) {
                break;
            }

            start += 1;
        }

        start
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    position: u8,
    visited_mask: u64,
    time_remaining: u8,
    pressure: u16,
}

#[derive(PartialEq, Eq)]
struct BoundedState {
    state: State,
    bound: u16,
}

impl Ord for BoundedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bound.cmp(&other.bound)
    }
}

impl PartialOrd for BoundedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for ValveSystem {
    fn from(value: &str) -> Self {
        let mut output = Self {
            size: 0,
            names: vec![],
            distances: vec![],
            flow_rates: vec![],
            flow_rates_sorted: vec![],
        };

        let mut neighbors = vec![];

        value.lines().for_each(|line| {
            let line = line.strip_prefix("Valve ").unwrap();
            let (valve_name, line) = line.split_once(' ').unwrap();
            let (flow_rate_str, line) = line
                .strip_prefix("has flow rate=")
                .unwrap()
                .split_once(';')
                .unwrap();
            let (_, line) = line.split_once("valve").unwrap().1.split_once(' ').unwrap();

            let mut connections: Vec<String> =
                line.split(", ").map(|str| str.to_string()).collect();
            if connections.is_empty() {
                connections = vec![line.to_string()];
            }
            let valve_name = valve_name.to_string();

            let flow_rate = flow_rate_str.parse().unwrap();

            output.names.push(valve_name);
            output.flow_rates.push(flow_rate);
            output.size += 1;

            neighbors.push(connections);
        });

        let mut distances = vec![vec![None; output.size as usize]; output.size as usize];

        for (index, neighbors) in neighbors.iter().cloned().enumerate() {
            for neighbor in neighbors {
                let Some((valve_id, _)) = output
                    .names
                    .clone()
                    .into_iter()
                    .enumerate()
                    .find(|(_, name)| neighbor.eq(name)) else {
                        continue;
                    };

                distances[index][valve_id] = Some(1);
            }
        }

        (0..distances.len()).for_each(|identity| {
            distances[identity][identity] = Some(0);
        });

        let distances = floyd_warshall(&distances);
        let distances: Vec<Vec<u8>> = distances
            .iter()
            .map(|row| {
                row.iter()
                    .map(|dist| dist.expect("distance to all nodes known"))
                    .collect()
            })
            .collect();

        output.distances = distances;

        output.trim();

        output.flow_rates_sorted = output.flow_rates.clone();
        output.flow_rates_sorted.sort();
        output.flow_rates_sorted.reverse();

        output
    }
}
fn floyd_warshall(graph: &Vec<Vec<Option<u8>>>) -> Vec<Vec<Option<u8>>> {
    let n = graph.len();
    let mut distance = graph.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(x), Some(y)) = (distance[i][k], distance[k][j]) {
                    if let Some(z) = distance[i][j] {
                        distance[i][j] = Some(std::cmp::min(z, x + y));
                    } else {
                        distance[i][j] = Some(x + y);
                    }
                }
            }
        }
    }

    distance
}

impl State {
    fn branch<'a>(&'a self, system: &'a ValveSystem) -> impl IntoIterator<Item = State> + 'a {
        (0..system.size as usize).filter_map(move |index| {
            let visit_bit = 1u64 << index;

            if self.visited_mask & visit_bit != 0 {
                return None;
            }

            let flow_rate = system.flow_rates[index];

            if flow_rate == 0 {
                return None;
            }

            let distance = system.distances[self.position as usize][index];
            let time_spent = distance + 1;

            if time_spent >= self.time_remaining {
                return None;
            }

            let time_remaining = self.time_remaining - time_spent;

            let mut branch = self.clone();

            branch.visited_mask |= visit_bit;
            branch.time_remaining = time_remaining;
            branch.position = index as u8;
            branch.pressure += flow_rate as u16 * time_remaining as u16;

            Some(branch)
        })
    }

    fn bound(&self, system: &ValveSystem) -> u16 {
        self.pressure
            + (0..=self.time_remaining)
                .rev()
                .step_by(2)
                .skip(1)
                .zip(system.flow_rates_sorted.clone())
                .map(|(time_left, flow_rate)| flow_rate as u16 * time_left as u16)
                .sum::<u16>()
    }

    fn branch_and_bound(&self, system: &ValveSystem, pressure: &mut u16) {
        *pressure = self.pressure.max(*pressure);

        for branch in self.branch(system) {
            let upper_bound = branch.bound(system);

            if upper_bound <= *pressure {
                continue;
            }

            branch.branch_and_bound(system, pressure);
        }
    }
}

pub fn solve_1(input: &str) -> String {
    let system = ValveSystem::from(input);

    let state = State {
        position: system.start(),
        pressure: 0,
        time_remaining: 30,
        visited_mask: 0,
    };

    let mut pressure = 0;

    state.branch_and_bound(&system, &mut pressure);

    pressure.to_string()
}
pub fn solve_2(input: &str) -> String {
    let system = ValveSystem::from(input);

    let mask = (1u64 << system.size) - 1;

    let halfway = (mask + 1) / 2;

    let mut best = 0;

    for index in 0..halfway {
        let visited = index;
        let elephant_visited = index ^ mask;

        let state = State {
            position: system.start(),
            pressure: 0,
            time_remaining: 26,
            visited_mask: visited,
        };

        let elephant = State {
            position: system.start(),
            pressure: 0,
            time_remaining: 26,
            visited_mask: elephant_visited,
        };

        let mut pressure = 0;

        state.branch_and_bound(&system, &mut pressure);

        let mut elephant_pressure = 0;

        elephant.branch_and_bound(&system, &mut elephant_pressure);

        best = best.max(pressure + elephant_pressure)
    }

    best.to_string()
}
