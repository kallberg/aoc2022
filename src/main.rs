use std::fmt::Display;

mod day10pt1;
mod day10pt2;
mod day1pt1;
mod day1pt2;
mod day2pt1;
mod day2pt2;
mod day3pt1;
mod day3pt2;
mod day4pt1;
mod day4pt2;
mod day5pt1;
mod day5pt2;
mod day6pt1;
mod day6pt2;
mod day7pt1;
mod day7pt2;
mod day8pt1;
mod day8pt2;
mod day9pt1;
mod day9pt2;

mod extra;

type BoxedSolver = Box<dyn FnOnce(&str) -> String>;

fn main() {
    let solvers: Vec<BoxedSolver> = vec![
        Box::new(day1pt1::solve),
        Box::new(day1pt2::solve),
        Box::new(day2pt1::solve),
        Box::new(day2pt2::solve),
        Box::new(day3pt1::solve),
        Box::new(day3pt2::solve),
        Box::new(day4pt1::solve),
        Box::new(day4pt2::solve),
        Box::new(day5pt1::solve),
        Box::new(day5pt2::solve),
        Box::new(day6pt1::solve),
        Box::new(day6pt2::solve),
        Box::new(day7pt1::solve),
        Box::new(day7pt2::solve),
        Box::new(day8pt1::solve),
        Box::new(day8pt2::solve),
        Box::new(day9pt1::solve),
        Box::new(day9pt2::solve),
        Box::new(day10pt1::solve),
        Box::new(day10pt2::solve),
    ];

    let inputs = vec![
        include_str!("../input/day1pt1.txt"),
        include_str!("../input/day1pt1.txt"),
        include_str!("../input/day2pt1.txt"),
        include_str!("../input/day2pt1.txt"),
        include_str!("../input/day3pt1.txt"),
        include_str!("../input/day3pt1.txt"),
        include_str!("../input/day4pt1.txt"),
        include_str!("../input/day4pt1.txt"),
        include_str!("../input/day5pt1.txt"),
        include_str!("../input/day5pt1.txt"),
        include_str!("../input/day6pt1.txt"),
        include_str!("../input/day6pt1.txt"),
        include_str!("../input/day7pt1.txt"),
        include_str!("../input/day7pt1.txt"),
        include_str!("../input/day8pt1.txt"),
        include_str!("../input/day8pt1.txt"),
        include_str!("../input/day9pt1.txt"),
        include_str!("../input/day9pt1.txt"),
        include_str!("../input/day10pt1.txt"),
        include_str!("../input/day10pt1.txt"),
    ];

    let days = inputs.into_iter().zip(solvers.into_iter());

    for (index, (input, solver)) in days.enumerate() {
        let day = (index) / 2 + 1;
        let part = (index % 2) + 1;

        let multiline = day == 10 && part == 2;

        run(day, part, solver, input, multiline);
    }
}

fn run<F, R>(day: usize, part: usize, solver: F, input: &str, multiline: bool)
where
    F: FnOnce(&str) -> R,
    R: Display,
{
    print!("day{}pt{}: ", day, part);
    if multiline {
        println!()
    }
    let solution = solver(input);
    println!("{}", solution);
}

#[cfg(test)]
mod tests {
    use crate::{
        day10pt1, day10pt2, day1pt1, day1pt2, day2pt1, day2pt2, day3pt1, day3pt2, day4pt1, day4pt2,
        day5pt1, day5pt2, day6pt1, day6pt2, day7pt1, day7pt1::Directory, day7pt2, day8pt1,
        day8pt1::TreeGrid, day8pt2, day9pt1, day9pt2,
    };

    #[test]
    fn day1pt1_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt1::solve(input), "24000");
    }

    #[test]
    fn day1pt2_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt2::solve(input), "45000");
    }

    #[test]
    fn day2pt1_eq_example() {
        let input = include_str!("../example_input/day2pt1.txt");
        assert_eq!(day2pt1::solve(input), "15")
    }

    #[test]
    fn day2pt2_eq_example() {
        let input = include_str!("../example_input/day2pt1.txt");
        assert_eq!(day2pt2::solve(input), "12")
    }

    #[test]
    fn day3pt1_eq_example() {
        let input = include_str!("../example_input/day3pt1.txt");
        assert_eq!(day3pt1::solve(input), "157")
    }

    #[test]
    fn day3pt2_eq_example() {
        let input = include_str!("../example_input/day3pt1.txt");
        assert_eq!(day3pt2::solve(input), "70")
    }

    #[test]
    fn day4pt1_eq_example() {
        let input = include_str!("../example_input/day4pt1.txt");
        assert_eq!(day4pt1::solve(input), "2")
    }

    #[test]
    fn day4pt2_eq_example() {
        let input = include_str!("../example_input/day4pt1.txt");
        assert_eq!(day4pt2::solve(input), "4")
    }

    #[test]
    fn day5pt1_eq_example() {
        let input = include_str!("../example_input/day5pt1.txt");
        assert_eq!(day5pt1::solve(input), "CMZ".to_string())
    }

    #[test]
    fn day5pt2_eq_example() {
        let input = include_str!("../example_input/day5pt1.txt");
        assert_eq!(day5pt2::solve(input), "MCD".to_string())
    }

    #[test]
    fn day6pt1_eq_example() {
        let input = include_str!("../example_input/day6pt1.txt");
        assert_eq!(day6pt1::solve(input), "7")
    }

    #[test]
    fn day6pt2_eq_example() {
        let input = include_str!("../example_input/day6pt1.txt");
        assert_eq!(day6pt2::solve(input), "19")
    }

    #[test]
    fn day7pt1_print_dir() {
        let input = include_str!("../input/day7pt1.txt");
        let dir = Directory::from(input);
        println!("{}", dir)
    }

    #[test]
    fn day7pt1_eq_example() {
        let input = include_str!("../example_input/day7pt1.txt");
        assert_eq!(day7pt1::solve(input), "95437");
    }

    #[test]
    fn day7pt2_eq_example() {
        let input = include_str!("../example_input/day7pt1.txt");
        assert_eq!(day7pt2::solve(input), "24933642")
    }

    #[test]
    fn day8pt1_eq_example() {
        let input = include_str!("../example_input/day8pt1.txt");
        assert_eq!(day8pt1::solve(input), "21");
    }

    #[test]
    fn day8pt2_eq_example() {
        let input = include_str!("../example_input/day8pt1.txt");
        let scan = TreeGrid::from(input);
        assert_eq!(scan.scenic_score(&day8pt1::Coord { x: 2, y: 1 }), 4);
        assert_eq!(scan.scenic_score(&day8pt1::Coord { x: 0, y: 0 }), 0);
        assert_eq!(day8pt2::solve(input), "8")
    }

    #[test]
    fn day9pt1_eq_example() {
        let input = include_str!("../example_input/day9pt1.txt");
        assert_eq!(day9pt1::solve(input), "13");
    }

    #[test]
    fn day9pt2_eq_example() {
        let input = include_str!("../example_input/day9pt1.txt");

        assert_eq!(day9pt2::solve(input), "1");

        let input = include_str!("../example_input/day9pt2.txt");

        assert_eq!(day9pt2::solve(input), "36");
    }

    #[test]
    fn day10pt1_eq_example() {
        let input = include_str!("../example_input/day10pt1.txt");

        assert_eq!(day10pt1::solve(input), "13140");
    }

    #[test]
    fn day10pt2_eq_example() {
        let input = include_str!("../example_input/day10pt1.txt");

        assert_eq!(
            day10pt2::solve(input),
            r"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
