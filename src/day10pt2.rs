use std::fmt::Display;

use crate::day10pt1::{Cpu, Instruction};

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

pub fn solve(input: &str) -> String {
    let cpu = Cpu {
        cycle: 0,
        register_x: 1,
        program: input.lines().map(Instruction::from).collect(),
        ..Default::default()
    };

    let crt = Crt { cpu };

    format!("{}", crt)
}
