use std::{collections::HashSet, hash::Hash};

#[derive(Eq, Clone, Debug)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub dist: usize,
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Node {
    pub id: String,
    pub value: usize,
    pub starting_cost: usize,
}

pub struct Graph {
    pub nodes: HashSet<Node>,
    pub edges: HashSet<Edge>,
}

impl Graph {
    pub fn node_permutations(&self) {}

    fn make_connections(&mut self) {
        let mut discovered = self.discover_edges();

        while !discovered.is_empty() {
            println!("{:?}", discovered);

            self.edges = self.edges.union(&self.discover_edges()).cloned().collect();

            discovered = self.discover_edges();
        }
    }

    fn simplify(&mut self, start_node: String) {
        let nodes = self.nodes.clone();

        for edge in &self.edges {
            if edge.from != start_node {
                continue;
            }

            let Some(node) = nodes.iter().find(|node| node.id == edge.to) else {
                continue;
            };

            let mut node = node.clone();
            self.nodes.remove(&node);
            node.starting_cost = edge.dist;
            self.nodes.insert(node);
        }

        for node in nodes {
            if node.value == 0 {
                self.nodes.remove(&node);

                self.edges
                    .retain(|edge| edge.from != node.id && edge.to != node.id);
            }
        }
    }

    fn discover_edges(&self) -> HashSet<Edge> {
        let mut output = HashSet::new();

        let edges = self.edges.clone();

        for node in &self.nodes {
            for edge in &edges {
                for other_edge in &edges {
                    if other_edge.eq(edge) {
                        continue;
                    }

                    let edge = Edge {
                        from: node.id.to_string(),
                        to: other_edge.to.clone(),
                        dist: other_edge.dist + edge.dist,
                    };

                    let Some(existing) = self.edges.get(&edge) else {
                        output.insert(edge);
                        continue;
                    };

                    if existing.dist > edge.dist {
                        output.remove(existing);
                        output.insert(edge);
                    }
                }
            }
        }

        output
    }
}

impl From<&str> for Graph {
    fn from(value: &str) -> Self {
        let mut output = Self {
            edges: HashSet::new(),
            nodes: HashSet::new(),
        };

        for line in value.lines() {
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

            output.nodes.insert(Node {
                id: valve_name.to_owned(),
                value: flow_rate_str.parse().unwrap(),
                starting_cost: 0,
            });

            let mut edge_set = HashSet::new();

            for connection in connections {
                edge_set.insert(Edge {
                    from: valve_name.to_owned(),
                    to: connection,
                    dist: 1,
                });
            }

            output.edges = output.edges.union(&edge_set).cloned().collect();
        }

        output.make_connections();
        output.simplify("AA".into());

        output
    }
}

pub fn solve_1(input: &str) -> String {
    let mut graph = Graph::from(input);

    todo!()
}
pub fn solve_2(input: &str) -> String {
    "todo".to_string()
}

fn permute(choices: &Vec<String>) -> Vec<Vec<String>> {
    if choices.is_empty() {
        // base case: return an empty permutation if there are no choices
        return vec![vec![]];
    }

    // recursive case: generate all permutations by adding one element at a time
    let mut permutations = Vec::new();
    for (i, c) in choices.iter().enumerate() {
        // get all permutations without c
        let mut sub_choices = choices.to_vec();
        sub_choices.remove(i);
        let sub_permutations = permute(&sub_choices);

        // add c to all permutations
        for mut permutation in sub_permutations {
            permutation.insert(0, c.clone());
            permutations.push(permutation);
        }
    }
    permutations
}
