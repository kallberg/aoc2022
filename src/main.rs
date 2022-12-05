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

fn main() {
    println!("{}", day1pt1::solve(include_str!("../input/day1pt1.txt")));
    println!("{}", day1pt2::solve(include_str!("../input/day1pt1.txt")));
    println!("{}", day2pt1::solve(include_str!("../input/day2pt1.txt")));
    println!("{}", day2pt2::solve(include_str!("../input/day2pt1.txt")));
    println!("{}", day3pt1::solve(include_str!("../input/day3pt1.txt")));
    println!("{}", day3pt2::solve(include_str!("../input/day3pt1.txt")));
    println!("{}", day4pt1::solve(include_str!("../input/day4pt1.txt")));
    println!("{}", day4pt2::solve(include_str!("../input/day4pt1.txt")));
    println!("{}", day5pt1::solve(include_str!("../input/day5pt1.txt")));
    println!("{}", day5pt2::solve(include_str!("../input/day5pt1.txt")));
    println!("{}", day6pt1::solve(include_str!("../input/day6pt1.txt")));
    println!("{}", day6pt2::solve(include_str!("../input/day6pt1.txt")));
}

#[cfg(test)]
mod tests {
    use crate::{
        day1pt1, day1pt2, day2pt1, day2pt2, day3pt1, day3pt2, day4pt1, day4pt2, day5pt1, day5pt2,
        day6pt1, day6pt2,
    };

    #[test]
    fn day1pt1_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt1::solve(input), 24000);
    }

    #[test]
    fn day1pt2_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt2::solve(input), 45000);
    }

    #[test]
    fn day2pt1_eq_example() {
        let input = include_str!("../example_input/day2pt1.txt");
        assert_eq!(day2pt1::solve(input), 15)
    }

    #[test]
    fn day2pt2_eq_example() {
        let input = include_str!("../example_input/day2pt1.txt");
        assert_eq!(day2pt2::solve(input), 12)
    }

    #[test]
    fn day3pt1_eq_example() {
        let input = include_str!("../example_input/day3pt1.txt");
        assert_eq!(day3pt1::solve(input), 157)
    }

    #[test]
    fn day3pt2_eq_example() {
        let input = include_str!("../example_input/day3pt1.txt");
        assert_eq!(day3pt2::solve(input), 70)
    }

    #[test]
    fn day4pt1_eq_example() {
        let input = include_str!("../example_input/day4pt1.txt");
        assert_eq!(day4pt1::solve(input), 2)
    }

    #[test]
    fn day4pt2_eq_example() {
        let input = include_str!("../example_input/day4pt1.txt");
        assert_eq!(day4pt2::solve(input), 4)
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
        assert_eq!(day6pt1::solve(input), "day6".to_string())
    }

    #[test]
    fn day6pt2_eq_example() {
        let input = include_str!("../example_input/day6pt1.txt");
        assert_eq!(day6pt2::solve(input), "day6".to_string())
    }
}
