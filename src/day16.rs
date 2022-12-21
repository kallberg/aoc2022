use std::{collections::HashSet, hash::Hash};

#[derive(Eq, Clone, Debug)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub distance: usize,
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

#[derive(Debug)]
pub struct Route {
    from: String,
    to: String,
    visits: HashSet<String>,
    distance: usize,
}

impl Graph {
    fn routes(&self, from: String) -> Vec<Route> {
        let mut output = vec![];

        for node in &self.nodes {
            if node.id == from {
                continue;
            }

            let Some(route) = self.shortest_route(&from, &node.id) else {
                continue;
            };

            output.push(route);
        }

        output
    }

    fn shortest_route(&self, from: &String, to: &String) -> Option<Route> {
        let mut edges_to: Vec<Edge> = self
            .edges
            .iter()
            .filter(|edge| &edge.to == to)
            .cloned()
            .collect::<Vec<Edge>>();

        edges_to.sort_by_key(|edge| edge.distance);
        let closest_to = edges_to.first()?;

        let mut edges_from: Vec<Edge> = self
            .edges
            .iter()
            .filter(|edge| &edge.from == from)
            .cloned()
            .collect();

        edges_from.sort_by_key(|edge| edge.distance);

        let closest_from = edges_from.first()?;

        let direct = self
            .edges
            .iter()
            .find(|edge| &edge.from == from && &edge.to == to)?;

        let mut visits = HashSet::new();
        visits.insert(from.clone());
        visits.insert(to.clone());

        if direct.distance < closest_from.distance + closest_to.distance {
            return Some(Route {
                from: from.clone(),
                to: to.clone(),
                distance: direct.distance,
                visits: HashSet::new(),
            });
        }

        visits.insert(closest_from.to.clone());
        visits.insert(closest_to.from.clone());

        Some(Route {
            from: to.clone(),
            to: to.clone(),
            visits,
            distance: closest_from.distance + closest_to.distance,
        })
    }

    fn make_connections(&mut self) {
        let mut discovered = self.discover_edges();

        while !discovered.is_empty() {
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
            node.starting_cost = edge.distance;
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
                        distance: other_edge.distance + edge.distance,
                    };

                    let Some(existing) = self.edges.get(&edge) else {
                        output.insert(edge);
                        continue;
                    };

                    if existing.distance > edge.distance {
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
                    distance: 1,
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

    let routes = graph.routes("FF".into());

    routes.iter().for_each(|route| println!("{:?}", route));

    todo!()
}
pub fn solve_2(input: &str) -> String {
    "todo".to_string()
}
