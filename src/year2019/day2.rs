use crate::create_solution;
use crate::puzzle::{Answerable, Solution};
use anyhow::anyhow;
use itertools::Itertools;

create_solution!(Day2, 2019, 2);

impl Solution for Day2 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let initial_state = input
            .trim()
            .split(',')
            .map(|n| {
                n.parse::<usize>()
                    .unwrap_or_else(|_| panic!("{} must be a valid number", n))
            })
            .collect_vec();

        let mut p1 = initial_state.clone();
        p1[1] = 12;
        p1[2] = 2;
        run_program(&mut p1);
        self.submit_part1(p1[0]);

        for a in 0..100 {
            for b in 0..100 {
                let mut p2 = initial_state.clone();
                p2[1] = a;
                p2[2] = b;
                if run_program(&mut p2) == 19690720 {
                    self.submit_part2(100 * a + b);
                }
            }
        }

        Ok(())
    }
}

pub fn run_program(codes: &mut [usize]) -> usize {
    let mut pc = 0;
    loop {
        let op = codes[pc];

        let p1 = codes[pc + 1];
        let p2 = codes[pc + 2];
        let p3 = codes[pc + 3];
        match op {
            1 => {
                // Add
                codes[p3] = codes[p1] + codes[p2];
            }
            2 => {
                // multiply
                codes[p3] = codes[p1] * codes[p2];
            }
            99 => {
                return codes[0];
            }
            _ => panic!("Invalid instruction encountered"),
        }

        pc += 4;
    }
}
