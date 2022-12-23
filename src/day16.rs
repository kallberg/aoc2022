use std::{cmp::min, collections::HashMap};

#[derive(Clone)]
pub struct Valve {
    pub id: usize,
    pub name: String,
    pub flow_rate: usize,
}

#[derive(Clone)]
pub struct ValveSystem {
    pub size: usize,
    pub names: Vec<String>,
    pub flow_rates: Vec<usize>,
    pub distances: Vec<Vec<usize>>,
    pub solution_cache: HashMap<SolverInput, usize>,
    pub elephant_solution_cache: HashMap<ElephantSolverInput, usize>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SolverInput {
    pub position: usize,
    pub time_left: usize,
    pub visited_mask: u64,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ElephantSolverInput {
    pub position: usize,
    pub time_left: usize,
    pub elephant_position: usize,
    pub elephant_time_left: usize,
    pub visited_mask: u64,
}

impl ValveSystem {
    pub fn solve(&mut self, input: SolverInput) -> usize {
        if let Some(cached) = self.solution_cache.get(&input) {
            return *cached;
        }

        let mut best_score = 0;

        for index in 0..self.size {
            let visit_bit = 1u64 << index;

            if input.visited_mask & visit_bit != 0 {
                continue;
            }

            let flow_rate = self.flow_rates[index];

            if flow_rate == 0 {
                continue;
            }

            let distance = self.distances[input.position][index];
            let time_spent = distance + 1;

            if time_spent >= input.time_left {
                continue;
            }

            let time_left = input.time_left - time_spent;

            let mut score = flow_rate * time_left;

            if time_left > 2 {
                let mut solver_next = input.clone();

                solver_next.visited_mask |= visit_bit;
                solver_next.time_left = time_left;
                solver_next.position = index;

                score += self.solve(solver_next);
            }

            if score > best_score {
                best_score = score;
            }
        }

        self.solution_cache.insert(input, best_score);

        best_score
    }

    pub fn elephant_solve(&mut self, input: ElephantSolverInput) -> usize {
        if let Some(cached) = self.elephant_solution_cache.get(&input) {
            return *cached;
        }

        if input.time_left < 2 && input.elephant_time_left < 2 {
            return 0;
        }

        let mut best_score = 0;

        for index in 0..self.size {
            for elephant_index in 0..self.size {
                if index == elephant_index {
                    continue;
                }

                /*                 if self.flow_rates[index] == 0 || self.flow_rates[elephant_index] == 0 {
                    continue;
                } */

                let visit_bits = 1u64 << index | 1u64 << elephant_index;

                if input.visited_mask & visit_bits != 0 {
                    continue;
                }

                let flow_rate = self.flow_rates[index];
                let elephant_flow_rate = self.flow_rates[elephant_index];

                if flow_rate == 0 || elephant_flow_rate == 0 {
                    continue;
                }

                let mut next = input.clone();

                let mut score = 0;
                let flow_rate = self.flow_rates[index];

                next.visited_mask |= visit_bits;
                next.position = index;
                next.elephant_position = elephant_index;

                let distance = self.distances[input.position][index];
                let time_spent = distance + 1;

                if time_spent < input.time_left {
                    let time_left = input.time_left - time_spent;
                    score += flow_rate * time_left;
                    next.time_left = time_left;
                } else {
                    continue;
                }

                let elephant_distance = self.distances[input.elephant_position][elephant_index];
                let elephant_time_spent = elephant_distance + 1;

                if elephant_time_spent < input.elephant_time_left {
                    let elephant_time_left = input.elephant_time_left - elephant_time_spent;
                    score += elephant_flow_rate * elephant_time_left;
                    next.elephant_time_left = elephant_time_left;
                } else {
                    continue;
                }

                if score > 0 {
                    score += self.elephant_solve(next);

                    if score > best_score {
                        best_score = score;
                    }
                }
            }
        }

        self.elephant_solution_cache.insert(input, best_score);

        best_score
    }
}

impl From<&str> for ValveSystem {
    fn from(value: &str) -> Self {
        let mut output = Self {
            size: 0,
            names: vec![],
            distances: vec![],
            solution_cache: HashMap::new(),
            elephant_solution_cache: HashMap::new(),
            flow_rates: vec![],
        };

        let mut neighbours = vec![];

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

            neighbours.push(connections);
        });

        let mut distances = vec![vec![None; output.size]; output.size];

        for (index, neighbours) in neighbours.iter().cloned().enumerate() {
            for neighbour in neighbours {
                let Some((valve_id, _)) = output
                    .names
                    .clone()
                    .into_iter()
                    .enumerate()
                    .find(|(_, name)| neighbour.eq(name)) else {
                        continue;
                    };

                distances[index][valve_id] = Some(1usize);
            }
        }

        for identity in 0..distances.len() {
            distances[identity][identity] = Some(0);
        }

        let distances = floyd_warshall(&distances);
        let distances: Vec<Vec<usize>> = distances
            .iter()
            .map(|row| {
                row.iter()
                    .map(|dist| dist.expect("distance to all nodes known"))
                    .collect()
            })
            .collect();

        output.distances = distances;

        output
    }
}

fn floyd_warshall(graph: &Vec<Vec<Option<usize>>>) -> Vec<Vec<Option<usize>>> {
    let n = graph.len();
    let mut distance = graph.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(x), Some(y)) = (distance[i][k], distance[k][j]) {
                    if let Some(z) = distance[i][j] {
                        distance[i][j] = Some(min(z, x + y));
                    } else {
                        distance[i][j] = Some(x + y);
                    }
                }
            }
        }
    }

    distance
}

pub fn solve_1(input: &str) -> String {
    let mut system = ValveSystem::from(input);

    let mut start = 0;

    let start_name = "AA".to_string();

    for name in &system.names {
        if start_name.eq(name) {
            break;
        }

        start += 1;
    }

    let solution = system
        .solve(SolverInput {
            position: start,
            time_left: 30,
            visited_mask: 0,
        })
        .to_string();

    solution
}
pub fn solve_2(input: &str) -> String {
    let mut system = ValveSystem::from(input);

    let mut start = 0;

    let start_name = "AA".to_string();

    for name in &system.names {
        if start_name.eq(name) {
            break;
        }

        start += 1;
    }

    let solution = system
        .elephant_solve(ElephantSolverInput {
            position: start,
            time_left: 26,
            elephant_position: start,
            elephant_time_left: 26,
            visited_mask: 0,
        })
        .to_string();

    solution
}
