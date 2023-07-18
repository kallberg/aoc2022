#[derive(Clone, Debug)]
struct SequenceNumber {
    identifier: i64,
    value: i64,
}

struct State(Vec<SequenceNumber>);

impl State {
    fn remove(&mut self, id: i64) -> (SequenceNumber, usize) {
        for (i, number) in self.0.iter().enumerate() {
            if number.identifier == id {
                return (self.0.remove(i), i);
            }
        }

        panic!("identifier {} not in sequence", id)
    }

    fn insert(&mut self, base: usize, number: SequenceNumber) {
        let index = self.local(base as i64 + number.value);
        if index == 0 {
            self.0.push(number);
        } else {
            self.0.insert(index, number);
        }
    }

    fn mix_move(&mut self, id: i64) {
        let (number, index) = self.remove(id);
        self.insert(index, number);
    }

    fn local(&self, index: i64) -> usize {
        let l = self.0.len() as i64;

        let sized = index % l;

        ((sized + l) % l) as usize
    }

    fn mix(&mut self) {
        for id in 0..self.0.len() as i64 {
            self.mix_move(id);
        }
    }

    fn as_grove_coordinate(&self) -> i64 {
        let zero_pos = self
            .0
            .iter()
            .enumerate()
            .find(|(i, number)| number.value == 0)
            .unwrap()
            .0 as i64;

        let first = self.local(zero_pos + 1000);
        let second = self.local(zero_pos + 2000);
        let third = self.local(zero_pos + 3000);

        self.0[first].value + self.0[second].value + self.0[third].value
    }
}

pub fn solve_1(input: &str) -> String {
    let mut state = State(
        input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .enumerate()
            .map(|(id, value)| SequenceNumber {
                identifier: id as i64,
                value,
            })
            .collect(),
    );

    state.mix();

    state.as_grove_coordinate().to_string()
}
pub fn solve_2(input: &str) -> String {
    let mut state = State(
        input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .enumerate()
            .map(|(id, value)| SequenceNumber {
                identifier: id as i64,
                value: value * 811589153,
            })
            .collect(),
    );

    (0..10).for_each(|_| state.mix());

    state.as_grove_coordinate().to_string()
}
