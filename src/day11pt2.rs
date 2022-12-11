use crate::day11pt1::Monkey;

pub fn solve(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let monkey_lines = lines.chunks(7);

    let monkey_strs = monkey_lines.map(|lines| lines.join("\n"));

    let mut monkeys: Vec<Monkey> = monkey_strs
        .map(|monkey_string| Monkey::from(monkey_string.as_str()))
        .collect();

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

        // println!("round {}", round + 1);

        // for (index, monkey) in monkeys.iter().enumerate() {
        //     println!("Monkey {}: {:?}", index, monkey.items)
        // }

        // println!()
    }

    for (index, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected items {} times.",
            index, monkey.inspections
        )
    }

    let mut inspections = monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<usize>>();

    inspections.sort();
    inspections.reverse();

    inspections.iter().take(2).product::<usize>().to_string()
}
