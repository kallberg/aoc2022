use std::{
    collections::BinaryHeap,
    ops::{AddAssign, SubAssign},
};

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

const ORE: Inventory = Inventory {
    ore: 1,
    clay: 0,
    obsidian: 0,
    geode: 0,
};

const CLAY: Inventory = Inventory {
    ore: 0,
    clay: 1,
    obsidian: 0,
    geode: 0,
};

const OBSIDIAN: Inventory = Inventory {
    ore: 0,
    clay: 0,
    obsidian: 1,
    geode: 0,
};

const GEODE: Inventory = Inventory {
    ore: 0,
    clay: 0,
    obsidian: 0,
    geode: 1,
};

#[derive(Clone, PartialEq, Eq, Default)]
struct State {
    resources: Inventory,
    robots: Inventory,
    block_ore: bool,
    block_clay: bool,
    block_obsidian: bool,
}

#[derive(PartialEq, Eq)]
struct BranchState {
    state: State,
    bound: u8,
    time: u8,
}

impl Ord for BranchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bound.cmp(&other.bound)
    }
}

impl PartialOrd for BranchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
struct Inventory {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl AddAssign for Inventory {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl SubAssign for Inventory {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

#[derive(PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Robot = Resource;

impl Inventory {
    fn get(&self, resource: &Resource) -> u8 {
        match resource {
            Resource::Ore => self.ore,
            Resource::Clay => self.clay,
            Resource::Obsidian => self.obsidian,
            Resource::Geode => self.geode,
        }
    }
}

struct Blueprint {
    id: u8,
    ore: Inventory,
    clay: Inventory,
    obsidian: Inventory,
    geode: Inventory,
}

impl Blueprint {
    fn cost(&self, robot: &Robot) -> &Inventory {
        match robot {
            Robot::Ore => &self.ore,
            Robot::Clay => &self.clay,
            Robot::Obsidian => &self.obsidian,
            Robot::Geode => &self.geode,
        }
    }

    fn resource_max(&self, robot: &Robot) -> u8 {
        [
            self.ore.get(robot),
            self.clay.get(robot),
            self.obsidian.get(robot),
            self.geode.get(robot),
        ]
        .iter()
        .cloned()
        .max()
        .unwrap_or(0)
    }
}

impl State {
    fn buy(&mut self, cost: Inventory, robot: Robot) {
        self.resources -= cost;

        self.robots += match robot {
            Robot::Ore => ORE,
            Robot::Clay => CLAY,
            Robot::Obsidian => OBSIDIAN,
            Robot::Geode => GEODE,
        }
    }

    fn affords(&self, cost: &Inventory) -> bool {
        self.resources.ore >= cost.ore
            && self.resources.clay >= cost.clay
            && self.resources.obsidian >= cost.obsidian
    }

    fn has_max(&self, blueprint: &Blueprint, robot: &Robot) -> bool {
        if robot == &Robot::Geode {
            return false;
        }

        let max = blueprint.resource_max(robot);
        let current = self.robots.get(robot);
        current >= max
    }

    fn bound(&self, blueprint: &Blueprint, mut time: u8) -> u8 {
        let mut geodes = self.resources.geode + self.robots.geode * time;
        let mut obsidian = self.resources.obsidian;
        let mut obsidian_robots = self.robots.obsidian;
        let geode_cost = blueprint.cost(&Robot::Geode);

        while time > 0 {
            time -= 1;

            if obsidian >= geode_cost.obsidian {
                geodes = geodes.checked_add(time).unwrap_or(u8::MAX);
                obsidian -= geode_cost.obsidian;
                obsidian += obsidian_robots;
                continue;
            }

            obsidian += obsidian_robots;
            obsidian_robots += 1;
        }

        geodes
    }

    fn branch(&self, blueprint: &Blueprint) -> Vec<State> {
        let mut states = vec![];
        let production = self.robots.clone();

        if !self.has_max(blueprint, &Robot::Geode) && self.affords(&blueprint.geode) {
            let mut state = self.clone();
            state.buy(blueprint.geode.clone(), Robot::Geode);
            state.resources += production;
            return vec![state];
        }

        let mut wait = self.clone();

        for robot in [Robot::Obsidian, Robot::Clay, Robot::Ore] {
            if self.has_max(blueprint, &robot) {
                continue;
            }

            if self.block_clay && robot == Robot::Clay {
                continue;
            }

            if self.block_ore && robot == Robot::Ore {
                continue;
            }

            if self.block_obsidian && robot == Robot::Obsidian {
                continue;
            }

            let cost = match robot {
                Robot::Ore => blueprint.ore.clone(),
                Robot::Clay => blueprint.clay.clone(),
                Robot::Obsidian => blueprint.obsidian.clone(),
                Robot::Geode => blueprint.geode.clone(),
            };

            if self.affords(&cost) {
                match robot {
                    Resource::Ore => wait.block_ore = true,
                    Resource::Clay => wait.block_clay = true,
                    Resource::Obsidian => wait.block_obsidian = true,
                    Resource::Geode => {}
                }

                let mut state = self.clone();
                state.block_clay = false;
                state.block_obsidian = false;
                state.block_ore = false;
                state.buy(cost, robot);
                state.resources += production.clone();
                states.push(state);
            }
        }

        wait.resources += production;
        states.push(wait);

        states
    }

    fn branch_and_bound(
        &self,
        blueprint: &Blueprint,
        priority_queue: &mut BinaryHeap<BranchState>,
        time: u8,
        max: &mut u8,
    ) -> u8 {
        if time == 0 {
            *max = (*max).max(self.resources.geode);
            return self.resources.geode;
        }

        if time == 1 {
            *max = (*max).max(self.resources.geode + self.robots.geode);
            return self.resources.geode + self.robots.geode;
        }

        for state in self.branch(blueprint) {
            let bound = state.bound(blueprint, time - 1);
            priority_queue.push(BranchState { state, bound, time });
        }

        while let Some(BranchState { state, bound, time }) = priority_queue.pop() {
            if bound <= *max {
                continue;
            }

            let geodes = state.branch_and_bound(blueprint, priority_queue, time - 1, max);

            *max = (*max).max(geodes);
        }

        *max
    }

    /*     fn branch_and_bound(&self, blueprint: &Blueprint, time: u8, max: &mut u8) -> u8 {
        if time == 0 {
            return self.resources.geode;
        }

        if time == 1 {
            return self.resources.geode + self.robots.geode;
        }

        for state in self.branch(blueprint) {
            let bound = state.bound(blueprint, time - 1);

            if bound <= *max {
                continue;
            }

            let geodes = state.branch_and_bound(blueprint, time - 1, max);

            *max = (*max).max(geodes);
        }

        *max
    } */
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let parsed: Vec<u8> = value
            .replace(':', " ")
            .split_whitespace()
            .filter_map(|word| word.parse::<u8>().ok())
            .take(7)
            .collect();

        Self {
            id: parsed[0],
            ore: Inventory {
                ore: parsed[1],
                ..Default::default()
            },
            clay: Inventory {
                ore: parsed[2],
                ..Default::default()
            },
            obsidian: Inventory {
                ore: parsed[3],
                clay: parsed[4],
                ..Default::default()
            },
            geode: Inventory {
                ore: parsed[5],
                obsidian: parsed[6],
                ..Default::default()
            },
        }
    }
}

pub fn solve_1(input: &str) -> String {
    let blueprints: Vec<Blueprint> = input.lines().map(Blueprint::from).collect();

    let result: u32 = blueprints
        .par_iter()
        .map(|blueprint| {
            let state = State {
                resources: Default::default(),
                robots: ORE,
                ..Default::default()
            };

            let mut max_geodes = 0;
            state.branch_and_bound(blueprint, &mut BinaryHeap::new(), 24, &mut max_geodes);

            max_geodes as u32 * blueprint.id as u32
        })
        .sum();

    result.to_string()
}
pub fn solve_2(input: &str) -> String {
    let blueprints: Vec<Blueprint> = input.lines().map(Blueprint::from).collect();

    let result: u32 = blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| {
            let state = State {
                resources: Default::default(),
                robots: ORE,
                ..Default::default()
            };

            let mut max_geodes = 0;
            state.branch_and_bound(blueprint, &mut BinaryHeap::new(), 32, &mut max_geodes);

            max_geodes as u32
        })
        .product();

    result.to_string()
}
