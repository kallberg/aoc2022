use crate::day11pt1::parse_monkeys;

pub fn solve(input: &str) -> String {
    let mut monkeys = parse_monkeys(input);

    let monkey_cycle: usize = monkeys.iter().map(|monkey| monkey.test).product();

    for _round in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_index];
            let throws = monkey.turn(false);

            for throw in throws {
                let receiver = &mut monkeys[throw.monkey_index];
                receiver.items.push(throw.item % monkey_cycle);
            }
        }
    }

    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<usize>>();

    inspections.sort();
    inspections.reverse();

    inspections.iter().take(2).product::<usize>().to_string()
}
