pub fn parse_inventories(input: &str) -> impl Iterator<Item = Vec<u64>> + '_ {
    input.split("\n\n").map(|inventory_string| {
        inventory_string
            .lines()
            .map(|line| line.parse::<u64>().expect("parse inventory line as u64"))
            .collect::<Vec<u64>>()
    })
}

pub fn solve(input: &str) -> String {
    parse_inventories(input)
        .map(|i| i.iter().sum::<u64>())
        .max()
        .expect("find maximum")
        .to_string()
}
