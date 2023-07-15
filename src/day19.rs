use std::collections::HashMap;

#[derive(Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone)]
struct Robot(Resource);

#[derive(Clone, Hash, PartialEq, Eq)]
struct Cost {
    ore: u64,
    clay: u64,
    obsidian: u64,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Blueprint {
    id: u64,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Simulation {
    time: u64,
    ore_robots: u64,
    clay_robots: u64,
    obsidian_robots: u64,
    geode_robots: u64,
    blueprint: Blueprint,
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
    time_limit: u64,
}

impl Blueprint {
    fn cost(&self, robot: &Robot) -> Cost {
        match robot {
            Robot(Resource::Ore) => self.ore.clone(),
            Robot(Resource::Clay) => self.clay.clone(),
            Robot(Resource::Obsidian) => self.obsidian.clone(),
            Robot(Resource::Geode) => self.geode.clone(),
        }
    }

    fn value(&self, robot: &Robot) -> u64 {
        let Robot(resource) = robot;

        match resource {
            Resource::Ore => todo!(),
            Resource::Clay => todo!(),
            Resource::Obsidian => todo!(),
            Resource::Geode => todo!(),
        }
    }
}

impl Simulation {
    fn using_blueprint(blueprint: Blueprint) -> Self {
        Self {
            time: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            blueprint,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            time_limit: 24,
        }
    }

    fn can_buy(&self, robot: Robot) -> bool {
        let cost = self.blueprint.cost(&robot);

        cost.ore <= self.ore && cost.clay <= self.clay && cost.obsidian <= self.obsidian
    }

    fn buy(&mut self, robot: &Robot) {
        let cost = self.blueprint.cost(&robot);

        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;

        match robot {
            Robot(Resource::Ore) => self.ore_robots += 1,
            Robot(Resource::Clay) => self.clay_robots += 1,
            Robot(Resource::Obsidian) => self.obsidian_robots += 1,
            Robot(Resource::Geode) => self.geode_robots += 1,
        }
    }

    fn run(self, mut cache: &mut HashMap<Self, u64>) -> u64 {
        if self.time >= self.time_limit {
            cache.insert(self.clone(), self.geode);
            return self.geode;
        }

        if let Some(cached) = cache.get(&self) {
            return *cached;
        }

        let result = self
            .clone()
            .step()
            .into_iter()
            .map(|next| next.run(&mut cache))
            .max()
            .unwrap_or(0);

        cache.insert(self, result);

        result
    }

    fn step(mut self) -> Vec<Self> {
        let mut branches = vec![];

        self.time += 1;

        if self.can_buy(Robot(Resource::Ore)) {
            let mut branch = self.clone();
            branch.buy(&Robot(Resource::Ore));
            branches.push(branch);
        }

        if self.can_buy(Robot(Resource::Clay)) {
            let mut branch = self.clone();
            branch.buy(&Robot(Resource::Clay));
            branches.push(branch);
        }

        if self.can_buy(Robot(Resource::Obsidian)) {
            let mut branch = self.clone();
            branch.buy(&Robot(Resource::Obsidian));
            branches.push(branch);
        }

        if self.can_buy(Robot(Resource::Geode)) {
            let mut branch = self.clone();
            branch.buy(&Robot(Resource::Geode));
            branches.push(branch);
        }

        branches.push(self.clone());

        for branch in &mut branches {
            branch.ore += self.ore_robots;
            branch.clay += self.clay_robots;
            branch.obsidian += self.obsidian_robots;
            branch.geode += self.geode_robots;
        }

        branches
    }

    fn quality_level(self) -> u64 {
        let mut cache = HashMap::new();

        self.blueprint.id * self.run(&mut cache)
    }
}

impl From<&str> for Blueprint {
    fn from(value: &str) -> Self {
        let parsed: Vec<u64> = value
            .replace(":", " ")
            .split_whitespace()
            .filter_map(|word| word.parse::<u64>().ok())
            .take(7)
            .collect();

        println!("{:?}", parsed);

        Self {
            id: parsed[0],
            ore: Cost {
                ore: parsed[1],
                clay: 0,
                obsidian: 0,
            },
            clay: Cost {
                ore: parsed[2],
                clay: 0,
                obsidian: 0,
            },
            obsidian: Cost {
                ore: parsed[3],
                clay: parsed[4],
                obsidian: 0,
            },
            geode: Cost {
                ore: parsed[5],
                clay: 0,
                obsidian: parsed[6],
            },
        }
    }
}

pub fn solve_1(input: &str) -> String {
    let blueprints: Vec<Blueprint> = input.lines().map(Blueprint::from).collect();

    let simulations: Vec<Simulation> = blueprints
        .into_iter()
        .map(Simulation::using_blueprint)
        .collect();

    let mut total_quality = 0;

    for simulation in simulations {
        total_quality += simulation.quality_level();
    }

    total_quality.to_string()
}
pub fn solve_2(input: &str) -> String {
    todo!()
}
