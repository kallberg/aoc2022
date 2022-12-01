mod day1pt1;
mod day1pt2;

fn main() {
    println!(
        "{}",
        day1pt1::find_solution(include_str!("../input/day1pt1.txt")).expect("finding solution")
    );
    println!(
        "{}",
        day1pt2::find_solution(include_str!("../input/day1pt1.txt"))
    );
}

#[cfg(test)]
mod tests {
    use crate::{day1pt1, day1pt2};

    #[test]
    fn day1pt1_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt1::find_solution(input), Some(24000));
    }

    #[test]
    fn day1pt2_eq_example() {
        let input = include_str!("../example_input/day1pt1.txt");
        assert_eq!(day1pt2::find_solution(input), 45000);
    }
}
