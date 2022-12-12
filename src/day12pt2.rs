use crate::day12pt1::{manage_climbers, Climb, Climber, Point};

pub fn solve_from_point(climb: &mut Climb, point: Point, best: usize) -> Option<usize> {
    let mut starting_climber = climb.starting_climber.clone();
    starting_climber.visited.clear();
    starting_climber.position = point.clone();
    starting_climber.visited.insert(point);
    starting_climber.max_moves = best - 1;

    let mut climbers = vec![starting_climber];

    let mut best_climbers: Vec<Climber>;

    while !climbers.is_empty() {
        climbers = manage_climbers(climbers, climb);

        best_climbers = climbers.iter().filter(|p| p.is_done()).cloned().collect();

        if !best_climbers.is_empty() {
            break;
        }
    }

    climbers.first().map(|climber| climber.moves)
}

pub fn solve(input: &str) -> String {
    let mut climb = Climb::from(input);
    let starting_points = climb.starting_points();

    let mut best = usize::MAX;

    for point in starting_points {
        if let Some(result) = solve_from_point(&mut climb, point.clone(), best) {
            best = best.min(result);
        }
    }

    best.to_string()
}
