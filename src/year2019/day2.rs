use crate::create_solution;
use crate::prelude::StringTools;
use crate::puzzle::{Answerable, Solution};
use crate::year2019::{run_program, IntCodeComputer};
use anyhow::anyhow;
use itertools::Itertools;

create_solution!(Day2, 2019, 2);

impl Solution for Day2 {
    fn handle_input(&mut self, input: &str) -> anyhow::Result<()> {
        let initial_state = input.nums::<i64>().collect_vec();

        let mut p1 = IntCodeComputer::from(&initial_state);

        p1.write(1, 12);
        p1.write(2, 2);
        p1.run_until_halt(Default::default());

        self.submit_part1(p1.read(0));

        for a in 0..100 {
            for b in 0..100 {
                let mut p2 = IntCodeComputer::from(&initial_state);
                p2.write(1, a);
                p2.write(2, b);
                p2.run_until_halt(Default::default());
                if p2.read(0) == 19690720 {
                    self.submit_part2(100 * a + b);
                }
            }
        }

        Ok(())
    }
}
