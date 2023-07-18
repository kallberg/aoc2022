pub enum OperationValue {
    Old,
    Value(usize),
}

pub enum Operation {
    Add(OperationValue, OperationValue),
    Multiply(OperationValue, OperationValue),
}

pub struct Throw {
    pub item: usize,
    pub monkey_index: usize,
}

pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: Operation,
    pub test: usize,
    pub test_true: usize,
    pub test_false: usize,
    pub inspections: usize,
}

impl From<&str> for OperationValue {
    fn from(value: &str) -> Self {
        if value == "old" {
            OperationValue::Old
        } else {
            OperationValue::Value(value.parse().unwrap())
        }
    }
}

impl OperationValue {
    pub fn evaluate(&self, old: usize) -> usize {
        match self {
            OperationValue::Old => old,
            OperationValue::Value(value) => *value,
        }
    }
}

impl Operation {
    pub fn evaluate(&self, old: usize) -> usize {
        match self {
            Operation::Add(lhs, rhs) => lhs.evaluate(old) + rhs.evaluate(old),
            Operation::Multiply(lhs, rhs) => lhs.evaluate(old) * rhs.evaluate(old),
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let operation_str = value.strip_prefix("  Operation: new = ").unwrap();
        let operation_parts: Vec<&str> = operation_str.splitn(3, ' ').collect();
        assert!(operation_parts.len() == 3);

        let lhs = OperationValue::from(operation_parts[0]);
        let op_str = operation_parts[1];
        let rhs = OperationValue::from(operation_parts[2]);

        match op_str {
            "+" => Operation::Add(lhs, rhs),
            "*" => Operation::Multiply(lhs, rhs),
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let monkey_lines: Vec<&str> = value
            .lines()
            .skip_while(|p| !p.starts_with("Monkey"))
            .skip(1)
            .take(5)
            .collect();

        let items: Vec<usize> = monkey_lines[0]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item_str| item_str.parse().unwrap())
            .collect();

        let operation = Operation::from(monkey_lines[1]);

        let test: usize = monkey_lines[2]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let test_true: usize = monkey_lines[3]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let test_false: usize = monkey_lines[4]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            items,
            operation,
            test,
            test_true,
            test_false,
            inspections: 0,
        }
    }
}

impl Monkey {
    pub fn turn(&mut self, lowers_worry: bool) -> Vec<Throw> {
        let mut thrown = Vec::<Throw>::new();

        self.inspections += self.items.len();

        for item in self.items.iter() {
            let mut worry = self.operation.evaluate(*item);

            if lowers_worry {
                worry /= 3;
            }

            if worry % self.test == 0 {
                thrown.push(Throw {
                    item: worry,
                    monkey_index: self.test_true,
                })
            } else {
                thrown.push(Throw {
                    item: worry,
                    monkey_index: self.test_false,
                })
            }
        }

        self.items.clear();

        thrown
    }
}

pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let lines: Vec<&str> = input.lines().collect();
    let monkey_lines = lines.chunks(7);

    let monkey_strings = monkey_lines.map(|lines| lines.join("\n"));

    monkey_strings
        .map(|monkey_string| Monkey::from(monkey_string.as_str()))
        .collect()
}

pub fn solve_1(input: &str) -> String {
    let mut monkeys = parse_monkeys(input);

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_index];
            let throws = monkey.turn(true);

            for throw in throws {
                let receiver = &mut monkeys[throw.monkey_index];
                receiver.items.push(throw.item);
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

pub fn solve_2(input: &str) -> String {
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
