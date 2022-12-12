pub fn parse_inventories(input: &str) -> impl Iterator<Item = Vec<u64>> + '_ {
    input.split("\n\n").map(|inventory_string| {
        inventory_string
            .lines()
            .map(|line| line.parse::<u64>().expect("parse inventory line as u64"))
            .collect::<Vec<u64>>()
    })
}

pub fn solve_1(input: &str) -> String {
    parse_inventories(input)
        .map(|i| i.iter().sum::<u64>())
        .max()
        .expect("find maximum")
        .to_string()
}

pub fn solve_2(input: &str) -> String {
    let mut inventory_sums: Vec<u64> = parse_inventories(input)
        .map(|inventory| inventory.into_iter().sum::<u64>())
        .collect();

    inventory_sums.sort_by(|a, b| b.cmp(a));

    inventory_sums.into_iter().take(3).sum::<u64>().to_string()
}
