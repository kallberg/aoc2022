use crate::day1pt1::parse_inventories;

pub fn solve(input: &str) -> String {
    let mut inventory_sums: Vec<u64> = parse_inventories(input)
        .map(|inventory| inventory.into_iter().sum::<u64>())
        .collect();

    inventory_sums.sort_by(|a, b| b.cmp(a));

    inventory_sums.into_iter().take(3).sum::<u64>().to_string()
}
