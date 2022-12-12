use crate::day12pt1::{manage_climbers, Climb, Point};

pub fn solve_from_point(mut climb: Climb, point: Point) -> Option<usize> {
    let mut starting_climber = climb.starting_climber.clone();
    starting_climber.visited.clear();
    starting_climber.position = point.clone();
    starting_climber.visited.insert(point);

    let mut climbers = vec![starting_climber];

    let mut best_climbers = vec![];

    while !climbers.is_empty() {
        climbers = manage_climbers(climbers, &mut climb);

        best_climbers = climbers.iter().filter(|p| p.is_done()).cloned().collect();

        //println!("Iteration {} climbers {}", iteration, climbers.len());

        if !best_climbers.is_empty() {
            //println!("Climber(s) reached goal");
            break;
        }

        // for (index, climber) in climbers.iter().enumerate() {
        //     println!(
        //         "Iteration {} climber {} position {:?} elevation {}",
        //         iteration, index, climber.position, climber.elevation
        //     );
        // }

        // println!()
    }

    climbers.first().map(|climber| climber.moves)
}

pub fn solve(input: &str) -> String {
    let climb = Climb::from(input);
    let starting_points = climb.starting_points();

    let solutions = starting_points
        .iter()
        .map(|p| solve_from_point(climb.clone(), p.clone()));

    let best = solutions.flatten().min().expect("solution");

    best.to_string()
}
