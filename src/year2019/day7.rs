use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use crate::year2019::{run_program, IntCodeComputer};
use itertools::Itertools;
use std::cmp::max;
create_solution!(Day7, 2019, 7);

impl Solution for Day7 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let program = input
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect_vec();

        let mut ans = 0;

        for values in (0..5).permutations(5) {
            let mut next = 0;
            for v in values {
                let output = IntCodeComputer::from(program.clone()).run_until_halt(vec![v, next]);
                next = output[0];
            }

            ans = max(ans, next);
        }

        self.submit_part1(ans);

        let mut ans = 0;
        'perm: for values in (5i64..10).permutations(5) {
            let mut computers = vec![
                IntCodeComputer::from(program.clone()),
                IntCodeComputer::from(program.clone()),
                IntCodeComputer::from(program.clone()),
                IntCodeComputer::from(program.clone()),
                IntCodeComputer::from(program.clone()),
            ];

            let mut next = 0;

            for (m_idx, v) in values.iter().enumerate() {
                let computer = &mut computers[m_idx];
                computer.run_with_input_until_halt(vec![*v]);
            }

            for i in 0.. {
                let computer = &mut computers[i % 5];

                if computer.halted() {
                    ans = max(ans, next);
                    // continue with the next permuations
                    continue 'perm;
                }

                let result = computer.run_with_input_until_halt(vec![next]);
                assert_eq!(result.len(), 1);
                next = *result.first().unwrap();
            }
        }

        self.submit_part2(ans);

        Ok(())
    }
}
