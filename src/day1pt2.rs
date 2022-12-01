use crate::day1pt1::parse_inventories;

pub fn find_solution(input: &str) -> u64 {
    let mut inventory_sums: Vec<u64> = parse_inventories(input)
        .into_iter()
        .map(|inventory| inventory.into_iter().sum::<u64>())
        .collect();

    inventory_sums.sort_by(|a, b| b.cmp(a));

    inventory_sums.into_iter().take(3).sum()
}
