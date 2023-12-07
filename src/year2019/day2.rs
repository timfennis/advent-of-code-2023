use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use crate::year2019::run_program;
use anyhow::anyhow;
use itertools::Itertools;

create_solution!(Day2, 2019, 2);

impl Solution for Day2 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let initial_state = input.nums::<i64>().collect_vec();

        let mut p1 = initial_state.clone();
        p1[1] = 12;
        p1[2] = 2;
        run_program(&mut p1, Default::default());
        self.submit_part1(p1[0]);

        for a in 0..100 {
            for b in 0..100 {
                let mut p2 = initial_state.clone();
                p2[1] = a;
                p2[2] = b;
                run_program(&mut p2, Default::default());
                if p2[0] == 19690720 {
                    self.submit_part2(100 * a + b);
                }
            }
        }

        Ok(())
    }
}
