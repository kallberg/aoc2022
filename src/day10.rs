use std::fmt::Display;

use crate::extra::visualize;

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> bool {
        match self {
            Instruction::Noop => true,
            Instruction::AddX(x) => {
                if cpu.instruction_cycle == 1 {
                    cpu.register_x += x;
                    true
                } else {
                    cpu.instruction_cycle += 1;
                    false
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    pub register_x: i64,
    pub program: Vec<Instruction>,
    pub program_counter: usize,
    pub instruction: Option<Instruction>,
    pub instruction_cycle: usize,
    pub cycle: i64,
}

impl Cpu {
    pub fn tick(&mut self) -> bool {
        if self.instruction.is_none() {
            if self.program_counter >= self.program.len() - 1 {
                return false;
            }

            self.instruction = Some(self.program[self.program_counter].clone());
            self.program_counter += 1;
            self.instruction_cycle = 0;
        }

        let instruction = self.instruction.clone().unwrap();

        if instruction.execute(self) {
            self.instruction = None;
        }

        self.cycle += 1;
        true
    }
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        if line.starts_with("noop") {
            return Self::Noop;
        }

        if line.starts_with("addx") {
            let x_str = line.strip_prefix("addx ").unwrap();
            let x: i64 = x_str.parse().unwrap();
            return Self::AddX(x);
        }

        unreachable!()
    }
}

pub fn solve_1(input: &str) -> String {
    let mut cpu = Cpu {
        cycle: 1,
        register_x: 1,
        program: input.lines().map(Instruction::from).collect(),
        ..Default::default()
    };

    let mut signal_strengths = vec![];

    while cpu.tick() {
        match cpu.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => signal_strengths.push(cpu.cycle * cpu.register_x),
            _ => {}
        }
    }

    signal_strengths.iter().sum::<i64>().to_string()
}

pub struct Crt {
    pub cpu: Cpu,
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cpu = self.cpu.clone();

        if cpu.register_x < 2 {
            write!(f, "#")?
        } else {
            write!(f, ".")?
        }

        while cpu.tick() {
            let position = cpu.cycle % 40;

            if position == 0 && cpu.cycle > 0 {
                writeln!(f)?;
            }

            if (cpu.register_x - position).abs() < 2 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

pub fn solve_2(input: &str) -> String {
    let cpu = Cpu {
        cycle: 0,
        register_x: 1,
        program: input.lines().map(Instruction::from).collect(),
        ..Default::default()
    };

    let crt = Crt { cpu };

    visualize(10, 2, &crt);

    format!("{}", crt)
}
